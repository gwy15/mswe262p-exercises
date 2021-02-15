//! Style #28
//! ==============================
//! Constraints:
//! - Data comes to functions in streams, rather than as a complete whole all at at once
//! - Functions are filters / transformers from one kind of data stream to another
//! Possible names:
//! - Lazy rivers
//! - Data streams
//! - Dataflow
//! - Data generators
//!

use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Read},
    path::{Path, PathBuf},
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn all_lines(filename: &Path) -> Result<impl Iterator<Item = String>> {
    let reader = BufReader::new(File::open(filename)?);

    let words = reader //
        .lines()
        .filter_map(|l| l.ok());
    Ok(words)
}

fn all_words(filename: &Path) -> Result<impl Iterator<Item = String>> {
    let lines = all_lines(filename)?;
    let iter = lines //
        .flat_map(move |l| {
            l.split(|ch: char| !ch.is_ascii_alphanumeric())
                .map(|s| s.to_lowercase())
                .collect::<Vec<_>>()
                .into_iter()
        });
    Ok(iter)
}

fn non_stop_words(filename: &Path) -> Result<impl Iterator<Item = String>> {
    let mut buf = String::new();
    File::open("../stop_words.txt")?.read_to_string(&mut buf)?;
    let stop_words: HashSet<String> = buf.split(',').map(|s| s.to_string()).collect();

    let iter = all_words(filename)?;
    let iter = iter.filter(move |s| s.len() > 1 && !stop_words.contains(s));
    Ok(iter)
}

fn count_and_sort(filename: &Path) -> Result<impl Iterator<Item = (String, usize)>> {
    let mut count = HashMap::new();
    for w in non_stop_words(filename)? {
        *count.entry(w).or_default() += 1;
    }
    let mut entries: Vec<(String, usize)> = count.into_iter().collect();
    entries.sort_by_key(|(_w, c)| Reverse(*c));
    Ok(entries.into_iter())
}

fn main() -> Result<()> {
    let filename: PathBuf = env::args().nth(1).expect("Usage: ./28 <path>").into();
    for (w, c) in count_and_sort(&filename)?.take(25) {
        println!("{} - {}", w, c);
    }
    Ok(())
}
