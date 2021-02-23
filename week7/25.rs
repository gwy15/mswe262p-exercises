//! Style #25
//! ==============================
//! This style is a variation of style #09, The One, with the following additional constraints:
//! Constraints:
//! - Core program functions have no side effects of any kind, including IO
//! - All IO actions must be contained in computation sequences that are
//!   clearly separated from the pure functions
//! - All sequences that have IO must be called from the main program
//! Possible names:
//! - Quarantine
//! - Monadic IO
//! - Imperative functional style
//!

use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufReader, Error as IOError, ErrorKind, Read},
};

type Result<T> = std::result::Result<T, IOError>;

// =========================  core implementation =======================

/// The higher-order function that actually does IO.
/// Any first-function that wishes to do IO should return `Func<O>` so that it can be a pure function.
/// The returned higher-order function will then be called as trait `Unwrap`.
struct Func<O> {
    f: Box<dyn FnOnce() -> Result<O>>,
}
impl<O> Func<O> {
    pub fn new(f: impl FnOnce() -> Result<O> + 'static) -> Self {
        Self { f: Box::new(f) }
    }
}

trait Unwrap<O> {
    fn my_unwrap(self) -> O;
}
/// Higher-order function
impl<O> Unwrap<O> for Func<O> {
    fn my_unwrap(self) -> O {
        (self.f)().unwrap()
    }
}
/// The default impl
impl<O> Unwrap<O> for O {
    fn my_unwrap(self) -> O {
        self
    }
}

struct TheOne<T> {
    value: T,
}
impl<T> TheOne<T> {
    pub fn bind<O, F, R>(self, f: F) -> TheOne<O>
    where
        F: FnOnce(T) -> R,
        R: Unwrap<O>,
    {
        let ret = f(self.value);
        // if f includes IO, it will return a higher-order function and will be called here
        let ret = ret.my_unwrap();
        TheOne::<O>::new(ret)
    }

    pub fn new(value: T) -> Self {
        Self { value }
    }
}
impl TheOne<String> {
    pub fn printme(self) {
        print!("{}", self.value);
    }
}

// ============================= logical related implementations =====================
// functions that include IO have signature that returns Func<>.

/// include IO
fn get_filepath(_: ()) -> Func<String> {
    Func::new(move || match std::env::args().nth(1) {
        Some(s) => Ok(s),
        None => Err(ErrorKind::InvalidInput.into()),
    })
}

/// include IO
fn read_file(path: String) -> Func<String> {
    Func::new(move || {
        let mut f = BufReader::new(File::open(path)?);
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    })
}

fn filter_chars(s: String) -> String {
    s.chars()
        .into_iter()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { ' ' })
        .collect()
}

fn normalize(s: String) -> String {
    s.to_lowercase()
}

fn scan(s: String) -> Vec<String> {
    s.split(' ').map(|s| s.to_string()).collect()
}

/// include IO
fn remove_stop_words(words: Vec<String>) -> Func<Vec<String>> {
    Func::new(move || {
        let mut f = BufReader::new(File::open("../stop_words.txt")?);
        let mut buf = String::new();
        f.read_to_string(&mut buf)?;
        let stop_words: HashSet<String> = buf.split(',').map(|s| s.to_string()).collect();
        let ret = words
            .into_iter()
            .filter(|w| w.len() > 1 && !stop_words.contains(w))
            .collect();
        Ok(ret)
    })
}

fn frequencies(words: Vec<String>) -> HashMap<String, usize> {
    let mut ret = HashMap::new();
    for w in words {
        *ret.entry(w).or_default() += 1;
    }
    ret
}

fn sort(freq: HashMap<String, usize>) -> Vec<(String, usize)> {
    let mut ret: Vec<_> = freq.into_iter().collect();
    ret.sort_unstable_by_key(|(_s, t)| Reverse(*t));
    ret
}

fn top25_freq(mut entries: Vec<(String, usize)>) -> String {
    let mut ret = String::new();
    entries.truncate(25);
    for (word, count) in entries {
        ret.push_str(&format!("{} - {}\n", word, count));
    }
    ret
}

fn main() {
    TheOne::new(())
        .bind(get_filepath)
        .bind(read_file)
        .bind(filter_chars)
        .bind(normalize)
        .bind(scan)
        .bind(remove_stop_words)
        .bind(frequencies)
        .bind(sort)
        .bind(top25_freq)
        .printme();
}
