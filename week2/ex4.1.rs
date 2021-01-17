//! Constraints:
//! + No abstractions
//! + No use of library functions
//! Possible names:
//! + Monolith
//! + Labyrinth
//! + Brain dump

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // read stop words
    let stop_words_file = File::open("../stop_words.txt").unwrap();
    let stop_words_reader = BufReader::new(stop_words_file).unwrap();
    let mut buf = String::new();
}
