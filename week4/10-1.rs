use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufReader, Read},
};

struct TheOne<T> {
    value: T,
}
impl<T> TheOne<T> {
    pub fn bind<O, F>(self, f: F) -> TheOne<O>
    where
        F: FnOnce(T) -> O,
    {
        TheOne::<O>::new(f(self.value))
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

fn read_file(path: String) -> String {
    let mut f = BufReader::new(File::open(path).unwrap());
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    s
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

fn remove_stop_words(words: Vec<String>) -> Vec<String> {
    let mut f = BufReader::new(File::open("../stop_words.txt").unwrap());
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    let stop_words: HashSet<String> = buf.split(',').map(|s| s.to_string()).collect();
    words
        .into_iter()
        .filter(|w| w.len() > 1 && !stop_words.contains(w))
        .collect()
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
    TheOne::new(std::env::args().nth(1).unwrap())
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
