//! Style #26
//! ==============================
//! Constraints:
//! - The input data of the problem is modeled as entities with relations between them
//! - The data is placed in tables, with columns potentially cross-referencing data in other tables
//! - Existence of a relational query engine
//! - The problem is solved by issuing queries over the tabular data
//!
//! Possible names:
//! - Tabular
//! - Flatland
//! - Relational
//!

use rusqlite::{params, Connection};
use std::{
    collections::HashSet,
    env,
    error::Error,
    fs::{remove_file, File},
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn get_connection() -> Result<Connection> {
    let path = PathBuf::from("./26-1.sqlite");
    if path.exists() {
        remove_file(&path)?
    }
    Ok(Connection::open(path)?)
    // Ok(Connection::open_in_memory()?)
}

fn create_db_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
    CREATE TABLE `documents` (
        id      INTEGER PRIMARY KEY AUTOINCREMENT,
        name    TEXT
    );
    CREATE TABLE `words` (
        id      INTEGER PRIMARY KEY AUTOINCREMENT,
        doc_id  INTEGER,
        value   TEXT
    ); "#,
    )?;
    Ok(())
}

fn get_stop_words() -> Result<HashSet<String>> {
    let reader = BufReader::new(File::open("../stop_words.txt")?);
    let mut ret = HashSet::new();
    reader.lines().filter_map(|l| l.ok()).for_each(|l| {
        l.split(',').for_each(|l| {
            ret.insert(l.to_string());
        })
    });

    Ok(ret)
}

fn get_words(path: &Path) -> Result<impl Iterator<Item = String>> {
    let stop_words = get_stop_words()?;

    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let words = reader
        .lines()
        .filter_map(|l| l.ok())
        .flat_map(move |l| {
            l.split(|ch: char| !ch.is_ascii_alphanumeric())
                .map(|s| s.to_lowercase())
                .collect::<Vec<_>>()
                .into_iter()
        })
        .filter(move |s| s.len() > 1 && !stop_words.contains(s));
    Ok(words)
}

fn save_filename_to_db(path: &Path, conn: &Connection) -> Result<u32> {
    // add document
    conn.execute(
        r#"INSERT INTO `documents` (name) VALUES (?)"#,
        params![path.to_str()],
    )?;
    let mut stmt = conn.prepare(
        r#"
        SELECT id FROM documents WHERE name = ?;
        "#,
    )?;
    let doc_id = stmt.query_row(params![path.to_str()], |row| {
        let id: u32 = row.get(0)?;
        Ok(id)
    })?;
    Ok(doc_id)
}

fn load_file(path: &Path, doc_id: u32, conn: &mut Connection) -> Result<u32> {
    // insert words
    let words = get_words(&path)?;
    // use transaction to accelerate insert
    let trans = conn.transaction()?;
    {
        let mut insert_stmt = trans.prepare_cached(
            r"
            INSERT INTO words (doc_id, value)
            VALUES
            (?, ?);",
        )?;
        for word in words {
            insert_stmt.execute(params![doc_id, word])?;
        }
    }
    trans.commit()?;
    Ok(doc_id)
}

fn print_stats(doc_id: u32, conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare(
        r#"
        SELECT value, COUNT(*) AS cnt
        FROM words
        WHERE doc_id = ?
        GROUP BY value
        ORDER BY cnt DESC
        LIMIT 25;
    "#,
    )?;
    struct Item {
        word: String,
        count: u32,
    }
    let rows = stmt
        .query_map(params![doc_id], |row| {
            Ok(Item {
                word: row.get(0)?,
                count: row.get(1)?,
            })
        })?
        .collect::<Vec<_>>();
    for row in rows {
        let row = row?;
        println!("{} - {}", row.word, row.count);
    }

    // print number of unique words with 'z'
    let mut stmt = conn.prepare(
        r#"
        SELECT COUNT(*) FROM (
            SELECT value, COUNT(*) FROM words
            WHERE value like '%z%'
            GROUP BY value
        ); "#,
    )?;
    let r: u32 = stmt.query_row(params![], |row| row.get(0))?;
    println!("Count of unique words with z: {}", r);

    Ok(())
}

fn main() -> Result<()> {
    let mut conn = get_connection()?;
    create_db_schema(&conn)?;

    let path: PathBuf = env::args()
        .nth(1)
        .expect("Usage: 26-1 <path-to-file>")
        .into();

    let doc_id = save_filename_to_db(&path, &conn)?;
    load_file(&path, doc_id, &mut conn)?;
    print_stats(doc_id, &conn)?;

    Ok(())
}
