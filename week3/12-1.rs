//! Style #12
//! ==============================
//! Constraints:
//! - The larger problem is decomposed into 'things' that make sense for
//!   the problem domain
//! - Each 'thing' is a capsule of data that exposes one single procedure,
//!   namely the ability to receive and dispatch messages that are sent to it
//! - Message dispatch can result in sending the message to another capsule
//!
//! Possible names:
//! - Letterbox
//! - Messaging style
//! - Objects
//! - Actors

use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::{BufRead, BufReader},
    panic,
};

/// This is core abstraction: each *THING* should have one single exposed procedure.
trait Letterbox {
    type Input;
    type Output;
    fn dispatch(&mut self, command: &'static str, message: Self::Input) -> Self::Output;
}

/// The main logic lies inside.
#[derive(Debug, Default)]
struct WordFrequencyController {
    data: Option<DataStorageManager>,
    stop_words: Option<StopWordsManager>,
    counter: Option<WordFrequencyManager>,
}
impl Letterbox for WordFrequencyController {
    type Input = String;
    type Output = ();
    fn dispatch(&mut self, command: &'static str, message: String) {
        let file = message;
        match command {
            "init" => {
                self.data = Some(DataStorageManager::default());
                self.stop_words = Some(StopWordsManager::default());
                self.counter = Some(WordFrequencyManager::default());
                self.data.as_mut().unwrap().dispatch("init", file);
                self.stop_words
                    .as_mut()
                    .unwrap()
                    .dispatch("init", "".to_string());
            }
            "run" => {
                let words = self
                    .data
                    .as_mut()
                    .unwrap()
                    .dispatch("words", "".to_string());
                for w in words {
                    if !self
                        .stop_words
                        .as_mut()
                        .unwrap()
                        .dispatch("is_stop_word", w.clone())
                    {
                        self.counter.as_mut().unwrap().dispatch("incr", w);
                    }
                }
                for (index, (word, times)) in self
                    .counter
                    .as_mut()
                    .unwrap()
                    .dispatch("top", "25".to_string())
                    .into_iter()
                    .enumerate()
                {
                    println!("[{:>2}] {:>20} - {:>4}", index + 1, word, times);
                }
            }
            _ => {
                panic!(format!("unknown command: {}", command));
            }
        }
    }
}

/// Store & split the words.
#[derive(Debug, Default)]
struct DataStorageManager {
    /// The words from file
    words: Vec<String>,
}
impl Letterbox for DataStorageManager {
    type Input = String;
    type Output = Vec<String>;
    fn dispatch(&mut self, command: &'static str, message: String) -> Self::Output {
        let file = message;
        match command {
            "init" => {
                let f = File::open(file).unwrap();
                let reader = BufReader::new(f);
                for line in reader.lines() {
                    let line = line.unwrap();
                    self.words.extend(
                        line.split(|ch: char| !ch.is_ascii_alphabetic())
                            .filter(|w| w.len() > 1)
                            .map(|s| s.to_lowercase()),
                    );
                }
                vec![]
            }
            "words" => self.words.clone(),
            _ => {
                panic!(format!("unknown command: {}", command));
            }
        }
    }
}

#[derive(Debug, Default)]
struct StopWordsManager {
    stop_words: HashSet<String>,
}
impl Letterbox for StopWordsManager {
    type Input = String;
    type Output = bool;
    fn dispatch(&mut self, command: &'static str, message: String) -> bool {
        let word = message;
        match command {
            "init" => {
                let f = File::open("../stop_words.txt").unwrap();
                let reader = BufReader::new(f);
                for line in reader.lines() {
                    self.stop_words
                        .extend(line.unwrap().split(',').map(|s| s.to_string()));
                }
                true
            }
            "is_stop_word" => self.stop_words.contains(&word),
            _ => {
                panic!(format!("unknown command: {}", command));
            }
        }
    }
}

#[derive(Debug, Default)]
struct WordFrequencyManager {
    counter: HashMap<String, usize>,
}
impl Letterbox for WordFrequencyManager {
    type Input = String;
    type Output = Vec<(String, usize)>;
    fn dispatch(&mut self, command: &'static str, message: String) -> Vec<(String, usize)> {
        match command {
            "incr" => {
                *self.counter.entry(message).or_default() += 1;
                vec![]
            }
            "top" => {
                let mut entries: Vec<(String, usize)> =
                    self.counter.iter().map(|(w, c)| (w.clone(), *c)).collect();
                entries.sort_unstable_by_key(|en| Reverse(en.1));
                entries.truncate(message.parse().unwrap());
                entries
            }
            _ => {
                panic!(format!("unknown command: {}", command));
            }
        }
    }
}

fn main() {
    let mut controller = WordFrequencyController::default();
    let file = env::args().skip(1).next().expect("No file provided.");
    controller.dispatch("init", file);
    controller.dispatch("run", "".to_string());
}
