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
    // check if file exist, if not, open default file
    let f = File::open(path_to_file).or_else(|_e| File::open("../pride-and-prejudice.txt"));
    // if file still fails to open, return empty result
    let f = match f {
        Ok(f) => f,
        Err(e) => {
            eprintln!("failed to open file: {}", e);
            return vec![];
        }
    };
    let reader = BufReader::new(f);
    let words: Vec<_> = reader
        .lines()
        // check whether is valid utf8 string, or else skip this line
        .filter_map(|l| l.ok())
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
    let stop_words_str = File::open("../stop_words.txt")
        .and_then(|f| {
            let mut reader = BufReader::new(f);
            let mut buf = String::new();
            // fail-fast
            reader.read_to_string(&mut buf)?;
            Ok(buf)
        })
        .unwrap_or_default();
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
    let filename: PathBuf = env::args()
        .nth(1)
        .unwrap_or_else(|| "../pride-and-prejudice.txt".to_string())
        .into();
    let words = extract_words(&filename);
    let non_stop_words = remove_stop_words(words);
    let freq = get_freq(non_stop_words);
    let mut freq = sort_freq(freq);
    freq.truncate(25);
    for (w, t) in freq {
        println!("{} - {}", w, t);
    }
}
