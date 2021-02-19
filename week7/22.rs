//! Style #21
//! ==============================
//! Constraints:
//! - Every single procedure and function checks the sanity of its
//!   arguments and either returns something sensible when the arguments
//!   are unreasonable or assigns them reasonable values
//! - All code blocks check for possible errors and escape the block
//!   when things go wrong, setting the state to something reasonable
//!
//! Possible names:
//! - Constructive
//! - Defensive
//! - Hopeful
//! - Shit happens, life goes on

use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::{BufRead, BufReader, Read},
    path::{Path, PathBuf},
};

fn extract_words(path_to_file: &Path) -> Vec<String> {
    let f = File::open(path_to_file).expect("failed to open file");
    let reader = BufReader::new(f);
    let words: Vec<_> = reader
        .lines()
        // check whether is valid utf8 string
        .map(|l| l.expect("invalid line"))
        .flat_map(move |line| {
            line.split(|ch: char| !ch.is_ascii_alphanumeric())
                .map(|s| s.to_lowercase())
                .collect::<Vec<_>>()
                .into_iter()
        })
        .collect();
    words
}

fn remove_stop_words(words: Vec<String>) -> Vec<String> {
    let f = File::open("../stop_words.txt").expect("failed to open stop_words.txt");
    let stop_words_str = {
        let mut reader = BufReader::new(f);
        let mut buf = String::new();
        // fail-fast
        reader
            .read_to_string(&mut buf)
            .expect("failed to read from file");
        buf
    };
    let stop_words_set: HashSet<_> = stop_words_str.split(',').map(|s| s.to_string()).collect();

    let non_stop_words = words
        .into_iter()
        .filter(move |w| w.len() > 1 && !stop_words_set.contains(w))
        .collect::<Vec<_>>();
    non_stop_words
}

fn get_freq(words: Vec<String>) -> HashMap<String, usize> {
    let mut freq = HashMap::new();
    for w in words {
        *freq.entry(w).or_default() += 1;
    }
    freq
}

fn sort_freq(freq: HashMap<String, usize>) -> Vec<(String, usize)> {
    let mut entries: Vec<_> = freq.into_iter().collect();
    entries.sort_unstable_by_key(|(_w, t)| Reverse(*t));
    entries
}

fn main() {
    let filename: PathBuf = env::args().nth(1).expect("Usage: ./22 <path>").into();
    let words = extract_words(&filename);
    let non_stop_words = remove_stop_words(words);
    let freq = get_freq(non_stop_words);
    let mut freq = sort_freq(freq);
    freq.truncate(25);
    for (w, t) in freq {
        println!("{} - {}", w, t);
    }
}
