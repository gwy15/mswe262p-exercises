//! Style #4
//! ==============================
//! Constraints:
//! + No abstractions
//! + No use of library functions
//! Possible names:
//! + Monolith
//! + Labyrinth
//! + Brain dump

use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // read stop words
    let stop_words_file = File::open("../stop_words.txt").unwrap();
    let mut stop_words_reader = BufReader::new(stop_words_file);
    let mut buf = String::new();
    stop_words_reader.read_line(&mut buf).unwrap();
    let mut stop_words = Vec::new();
    let mut word = String::new();
    for ch in buf.chars() {
        if ch.is_alphabetic() {
            word.push(ch);
        } else {
            stop_words.push(word);
            word = String::new();
        }
    }
    if word.len() > 0 {
        stop_words.push(word);
    }
    // read file
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // process
    // [ (word, times) ]
    let mut counter: Vec<(String, usize)> = Vec::new();

    // iterate through lines is also used in tf-04.py
    for line in reader.lines() {
        let line = line.unwrap();
        let char_count = line.chars().count();

        let mut word = String::new();
        // process through chars
        for (idx, ch) in line.chars().enumerate() {
            if ch.is_alphabetic() {
                // since a utf-8 char does not necessary have a lower case in a single char
                // we're using string
                word.push_str(&ch.to_lowercase().to_string());
            }
            // since rust `lines` methods does not have NEWLINE char, manually detect
            if !ch.is_alphabetic() || idx == char_count - 1 {
                // test single characters
                let mut acceptable = word.len() > 1;
                // test stop words
                if acceptable {
                    for stop_word in stop_words.iter() {
                        if stop_word == &word {
                            acceptable = false;
                            break;
                        }
                    }
                }

                if acceptable {
                    // count the word
                    let mut found = false;
                    for i in 0..counter.len() {
                        if counter[i].0 == word {
                            counter[i].1 += 1;
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        counter.push((word, 1));
                    }
                }
                // reset word
                word = String::new();
            }
        }
    }

    // sort counter
    let n = counter.len();
    for i in 0..n {
        for j in (i + 1..n).rev() {
            if counter[j - 1].1 < counter[j].1 {
                // swap i and j
                counter.swap(j - 1, j);
            }
        }
    }
    // print
    for i in 0..25 {
        if i >= counter.len() {
            break;
        }
        println!("[{:>2}] {:>20} - {:>4}", i + 1, counter[i].0, counter[i].1);
    }
}
