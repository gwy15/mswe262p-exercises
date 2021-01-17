//! Style #6
//! ==============================
//! Constraints:
//! - Larger problem decomposed in functional abstractions. Functions, according to Mathematics, are relations from inputs to outputs.
//! - Larger problem solved as a pipeline of function applications
//! Possible names:
//! - Candy factory
//! - Functional
//! - Pipeline

use std::ops::BitOr;

// ================= pipeline framework ==================

/// The `Pipe` type wraps a function pointer and impl the `|` operator so as to
/// provide a shell-like operation.
///
/// # Example
/// ```
/// Value::v(1) | Pipe::f(|i| println!("{}", i));
/// ```
struct Pipe<F> {
    inner: F,
}
impl<F> Pipe<F> {
    fn f(f: F) -> Pipe<F> {
        Self { inner: f }
    }

    pub fn call<Input, Output>(self, input: Input) -> Output
    where
        F: FnOnce(Input) -> Output,
    {
        (self.inner)(input)
    }
}

/// The `Value` type wraps a value so that `Value | Pipe` can be implemented without
/// violating the orphan rule in Rust.
struct Value<V> {
    inner: V,
}
impl<V> Value<V> {
    fn v(v: V) -> Value<V> {
        Self { inner: v }
    }
    fn value(self) -> V {
        self.inner
    }
}

impl<F, Output, Input> BitOr<Pipe<F>> for Value<Input>
where
    F: FnOnce(Input) -> Output,
{
    type Output = Value<Output>;
    fn bitor(self, pipe: Pipe<F>) -> Self::Output {
        Value::v(pipe.call(self.value()))
    }
}

// ===============  exercise related logic ======================
use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn get_reader() -> BufReader<File> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    // read file
    let f = File::open(filename).unwrap();
    BufReader::new(f)
}

fn get_words(reader: BufReader<File>) -> Vec<String> {
    let mut words = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        words.extend(
            line.split(|c: char| !c.is_ascii_alphabetic())
                .map(|s| s.to_string()),
        );
    }
    words
}

fn lower(words: Vec<String>) -> Vec<String> {
    words.into_iter().map(|w| w.to_lowercase()).collect()
}

/// filter out single characters and stop words
fn filter(words: Vec<String>) -> Vec<String> {
    let f = File::open("../stop_words.txt").unwrap();
    let mut reader = BufReader::new(f);
    let mut buf = String::new();
    reader.read_line(&mut buf).unwrap();
    let stop_words: HashSet<String> = buf.split(',').map(|s| s.to_string()).collect();

    words
        .into_iter()
        .filter(|w| w.len() > 1 && !stop_words.contains(w))
        .collect()
}

fn count(words: Vec<String>) -> Vec<(String, usize)> {
    let mut counter = HashMap::new();
    for w in words {
        *counter.entry(w).or_default() += 1;
    }
    counter.into_iter().collect()
}

fn sort(mut items: Vec<(String, usize)>) -> Vec<(String, usize)> {
    items.sort_unstable_by_key(|item| Reverse(item.1));
    items
}

fn truncate_25(mut items: Vec<(String, usize)>) -> Vec<(String, usize)> {
    items.truncate(25);
    items
}

fn print(data: Vec<(String, usize)>) {
    for (idx, row) in data.into_iter().enumerate() {
        println!("[{:>2}] {:>20} - {:>4}", idx + 1, row.0, row.1);
    }
}

fn main() {
    let _ = Value::v(get_reader())
        | Pipe::f(get_words)
        | Pipe::f(lower)
        | Pipe::f(filter)
        | Pipe::f(count)
        | Pipe::f(sort)
        | Pipe::f(truncate_25)
        | Pipe::f(print);
}
