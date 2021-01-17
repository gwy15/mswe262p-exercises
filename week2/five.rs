//! Style #5
//! ==============================
//! Constraints:
//! - Larger problem decomposed in procedural abstractions
//! - Larger problem solved as a sequence of commands, each corresponding to a procedure
//! Possible names:
//! - Cookbook
//! - Procedural

use std::{
    cmp::Reverse,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

// Please know that Rust does not allow static mutable variables because they
// are UNSAFE and may cause memory violation. Therefore, I'm using local variables
// with closures.

fn main() {
    // shared mutable data
    let mut stop_words = Vec::new();
    let mut words = Vec::new();
    // [ (word, times) ]
    let mut counter: Vec<(String, usize)> = Vec::new();

    // This is closure in Rust, works like a function / procedure except that it
    // can capture local variables, to some extent.
    //
    // read the stop word file to the stop_words variable.
    let mut read_stop_words = || {
        let stop_words_file = File::open("../stop_words.txt").unwrap();
        let mut stop_words_reader = BufReader::new(stop_words_file);
        let mut buf = String::new();
        stop_words_reader.read_line(&mut buf).unwrap();
        stop_words = buf.split(',').map(|s| s.to_string()).collect();
    };

    // read the input file to the words variable.
    let mut read_input_file = || {
        let args: Vec<String> = env::args().collect();
        let filename = &args[1];
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        // process
        for line in reader.lines() {
            let line = line.unwrap();

            // process through chars
            for word in line.split(|ch: char| !ch.is_ascii_alphabetic()) {
                // add to words
                words.push(word.to_lowercase());
            }
        }
    };

    // An mutable object cannot be referenced as mutable multiple times
    // because it may cause data race and thus is UNSAFE. I'm calling
    // the first two procedures here to avoid that.
    read_stop_words();
    read_input_file();

    // filter the read words by length and stop words.
    let mut filter = || {
        for word in words.iter_mut() {
            *word = word.to_lowercase();
        }
        let mut to_remove_indexes = Vec::new();
        for i in 0..words.len() {
            if words[i].len() <= 1 {
                to_remove_indexes.push(i);
                continue;
            }
            for stop_word in stop_words.iter() {
                if stop_word == &words[i] {
                    to_remove_indexes.push(i);
                    break;
                }
            }
        }
        for i in to_remove_indexes.into_iter().rev() {
            words.remove(i);
        }
    };

    filter();

    // make words to counter.
    let mut count = || {
        for word in words.iter() {
            let mut seen = false;
            for (w, times) in counter.iter_mut() {
                if w == word {
                    *times += 1;
                    seen = true;
                    break;
                }
            }
            if !seen {
                counter.push((word.clone(), 1));
            }
        }
    };

    count();

    // now sort the counter.
    let mut sort = || {
        counter.sort_unstable_by_key(|tuple| Reverse(tuple.1));
    };

    sort();

    // print the counter.
    let print = || {
        for i in 0..25 {
            if i >= counter.len() {
                break;
            }
            println!("[{:>2}] {:>20} - {:>4}", i + 1, counter[i].0, counter[i].1);
        }
    };

    print();
}
