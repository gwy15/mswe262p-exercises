//! Style #32
//! ==============================
//! Very similar to style #30, but with an additional twist
//!
//! Constraints:
//! - Input data is divided in chunks, similar to what an inverse multiplexer does to input signals
//! - A map function applies a given worker function to each chunk of data, potentially in parallel
//! - The results of the many worker functions are reshuffled in a way
//!   that allows for the reduce step to be also parallelized
//! - The reshuffled chunks of data are given as input to a second map
//!   function that takes a reducible function as input
//!
//! Possible names:
//! - Map-reduce
//! - Hadoop style
//! - Double inverse multiplexer
//!

use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    env,
    error::Error,
    fs::File,
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

///
fn read_file(f: &Path) -> Result<String> {
    let mut r = BufReader::new(File::open(f)?);
    let mut buf = String::new();
    r.read_to_string(&mut buf)?;
    Ok(buf)
}

fn partition(s: String, chunk_size: usize) -> impl Iterator<Item = String> {
    let mut i = 0;
    let mut chunk = String::new();
    let mut result = vec![];
    for line in s.lines() {
        i += 1;
        chunk += line;
        if i == chunk_size - 1 {
            i = 0;
            result.push(chunk);
            chunk = String::new();
        } else {
            chunk.push('\n');
        }
    }
    if chunk.len() > 0 {
        result.push(chunk);
    }
    result.into_iter()
}

/// Takes a string, returns a list of pairs (word, 1),
/// one for each word in the input, so
/// [(w1, 1), (w2, 1), ..., (wn, 1)]
fn split_words(s: String) -> Vec<(String, usize)> {
    let stop_words: HashSet<String> = {
        let mut r = BufReader::new(File::open("../stop_words.txt").unwrap());
        let mut buf = String::new();
        r.read_to_string(&mut buf).unwrap();
        buf.split(',').map(|s| s.to_string()).collect()
    };
    s.split(|ch: char| !ch.is_ascii_alphanumeric())
        .map(|s| s.to_lowercase())
        .filter(|s| s.len() > 1 && !stop_words.contains(s))
        .map(|s| (s, 1))
        .collect()
}

/// Takes a list of lists of pairs of the form
/// [[(w1, 1), (w2, 1), ..., (wn, 1)],
/// [(w1, 1), (w2, 1), ..., (wn, 1)],
/// ...]
/// and returns a dictionary mapping each unique word to the
/// corresponding list of pairs, so
/// { w1 : [(w1, 1), (w1, 1)...],
///  w2 : [(w2, 1), (w2, 1)...],
///  ...}
fn regroup(
    groups: impl Iterator<Item = Vec<(String, usize)>>,
) -> HashMap<String, Vec<(String, usize)>> {
    let mut result: HashMap<String, Vec<(String, usize)>> = HashMap::new();
    for group in groups {
        for (word, times) in group {
            if result.contains_key(&word) {
                result.get_mut(&word).unwrap().push((word, times));
            } else {
                result.insert(word.clone(), vec![(word, times)]);
            }
        }
    }
    result
}

/// Takes a mapping of the form (word, [(word, 1), (word, 1)...)])
/// and returns a pair (word, frequency), where frequency is the
/// sum of all the reported occurrences
fn count_words(item: (String, Vec<(String, usize)>)) -> (String, usize) {
    (item.0, item.1.into_iter().fold(0, |a, b| a + b.1))
}

fn main() -> Result<()> {
    let f: PathBuf = env::args().nth(1).expect("Usage: ./32 <path>").into();
    let mapped = partition(read_file(&f)?, 200).map(split_words);
    let regrouped = regroup(mapped);

    let mut sorted: Vec<(String, usize)> = regrouped.into_iter().map(count_words).collect();
    sorted.sort_unstable_by_key(|(_w, times)| Reverse(*times));
    sorted.truncate(25);
    for (w, c) in sorted {
        println!("{} - {}", w, c);
    }
    Ok(())
}
