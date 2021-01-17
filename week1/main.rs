use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::{BufRead, BufReader},
    process,
};

mod retcode {
    pub const OPEN_FILE: i32 = 1;
    pub const READ: i32 = 2;
    pub const ARG: i32 = 3;
}

fn get_stop_words() -> HashSet<String> {
    let stop_words_file = File::open("../stop_words.txt").unwrap_or_else(|e| {
        eprintln!("Error open stop words file: {}", e);
        process::exit(retcode::OPEN_FILE);
    });
    let mut stop_words_reader = BufReader::new(stop_words_file);
    let mut buf = Default::default();
    stop_words_reader.read_line(&mut buf).unwrap_or_else(|e| {
        eprintln!("Error read stop words: {}", e);
        process::exit(retcode::READ);
    });
    buf.split(',').map(|s| s.to_string()).collect()
}

fn get_reader() -> BufReader<File> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <some_text_file>", args[0]);
        process::exit(retcode::ARG);
    }
    let filename = &args[1];
    // read file
    let f = File::open(filename).unwrap_or_else(|e| {
        eprintln!("Error open file `{}`: {}", filename, e);
        process::exit(retcode::OPEN_FILE);
    });
    BufReader::new(f)
}

fn main() {
    // read stop words
    let stop_words = get_stop_words();
    // read file
    let reader = get_reader();
    // parse & count
    let mut counter: HashMap<String, usize> = HashMap::new();
    for line in reader.lines() {
        let line = line.unwrap_or_else(|e| {
            eprintln!("Failed to parse line: {}", e);
            process::exit(retcode::READ);
        });
        line.split(|c: char| !c.is_ascii_alphabetic())
            .filter(|word| word.len() > 1)
            .map(|word| word.to_lowercase())
            .filter(|word| !stop_words.contains(word))
            .for_each(|word| {
                *counter.entry(word).or_default() += 1;
            })
    }
    // sort & print
    let mut entries: Vec<(Reverse<usize>, String)> = counter
        .into_iter()
        .map(|(word, times)| (Reverse(times), word))
        .collect();
    entries.sort_unstable();
    // top 25 only
    entries.truncate(25);
    for (idx, (Reverse(times), word)) in entries.into_iter().enumerate() {
        println!("[{:>2}] {:>20} - {:>4}", idx + 1, word, times);
    }
}
