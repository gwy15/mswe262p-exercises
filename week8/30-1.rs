//! Style #30
//! ==============================
//! Constraints:
//! - Existence of one or more units that execute concurrently
//! - Existence of one or more data spaces where concurrent units store and
//!   retrieve data
//! - No direct data exchanges between the concurrent units, other than via the data spaces
//!
//! Possible names:
//! - Dataspaces
//! - Linda
//!

use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Read},
    sync::{Arc, Mutex},
    thread,
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    // public spaces lies in here
    let word_space = Arc::new(Mutex::new(VecDeque::new()));
    let freq_space = Arc::new(Mutex::new(VecDeque::new()));

    // get stop_words
    let stop_words: Arc<HashSet<String>> = {
        let mut buf = String::new();
        BufReader::new(File::open("../stop_words.txt")?).read_to_string(&mut buf)?;
        Arc::new(buf.split(',').map(|s| s.to_string()).collect())
    };

    // put words to word_space
    let filepath = std::env::args().nth(1).unwrap();
    for l in BufReader::new(File::open(filepath)?).lines() {
        for w in l?
            .split(|ch: char| !ch.is_ascii_alphanumeric())
            .map(|s| s.to_lowercase())
        {
            word_space.lock().unwrap().push_back(w);
        }
    }

    // start multiple workers process_words
    let workers: Vec<_> = (0..5)
        .map(|_| {
            let word_space = word_space.clone();
            let freq_space = freq_space.clone();
            let stop_words = stop_words.clone();
            thread::spawn(move || {
                let mut freq: HashMap<String, usize> = HashMap::new();
                loop {
                    let w = { word_space.lock().unwrap().pop_front() };
                    match w {
                        Some(w) => {
                            if w.len() > 1 && !stop_words.contains(&w) {
                                *freq.entry(w).or_default() += 1;
                            }
                        }
                        None => break,
                    }
                }
                freq_space.lock().unwrap().push_back(freq);
            })
        })
        .collect();
    // wait all workers to stop
    workers.into_iter().for_each(|h| h.join().unwrap());
    // merge frequencies
    let mut freq: HashMap<String, usize> = HashMap::new();
    for f in freq_space.lock().unwrap().drain(..) {
        for (k, v) in f {
            *freq.entry(k).or_default() += v;
        }
    }
    // print top 25
    let mut items: Vec<(String, usize)> = freq.into_iter().collect();
    items.sort_unstable_by_key(|(_word, t)| Reverse(*t));
    items.truncate(25);
    for (word, times) in items {
        println!("{} - {}", word, times);
    }
    Ok(())
}
