//! Style #29
//! ==============================
//! Similar to the letterbox style, but where the 'things' have
//! independent threads of execution.
//! Constraints:
//! - The larger problem is decomposed into 'things' that make sense for
//!   the problem domain
//! - Each 'thing' has a queue meant for other \textit{things} to place
//!   messages in it
//! - Each 'thing' is a capsule of data that exposes only its
//!   ability to receive messages via the queue
//! - Each 'thing' has its own thread of execution independent of the
//!   others.
//! Possible names:
//! - Free agents
//! - Active letterbox
//! - Actors
//!
//!
//! data flow:  -init-> controller -init->          data
//!                                -init->          stop_words
//!             -run--> controller -run->           words
//!                     words      -filter->        stop_words
//!                     stop_words -word ->         counter
//!                     words      -finish->        stop_words
//!                     stop_words -top25->         counter
//!             -end--> controller -end->           data
//!                                -end->           stop_words
//!                                -end->           counter

use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    default::Default,
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    panic,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Mutex, RwLock,
    },
    thread::{self, JoinHandle},
};

/// This code is a bit long to read. But the main idea is that there are four components,
/// `WordFrequencyController`, `DataStorageManager`, `StopWordsManager` and `WordFrequencyManager`.
/// To quickly go through the logic, see their implementation of method `dispatch` in trait
/// implementation of trait `Letterbox`.

/// ====   core abstraction ============
/// Each letter box has a sender that can be provided to outer world and receive message through
/// the corresponding receiver. The sender and receiver are two side of a queue but thread safe and
/// support multiple senders.

/// The `MsgSender` and `MsgReceiver` send & receive messages. Messages are composed of
/// a command name and a payload.
type MsgSender<T> = Sender<(&'static str, T)>;
type MsgReceiver<T> = Receiver<(&'static str, T)>;

#[derive(Debug)]
struct Channel<T> {
    pub tx: MsgSender<T>,
    pub rx: MsgReceiver<T>,
}
impl<T> Default for Channel<T> {
    fn default() -> Self {
        let (tx, rx) = channel();
        Self { tx, rx }
    }
}

/// Each 'Letterbox' object can receive a message
trait Letterbox {
    type Input;
    /// expose the letterbox to the outside world.
    fn letterbox(&self) -> MsgSender<Self::Input>;

    /// how you handle a message
    fn dispatch(&self, command: &'static str, message: Self::Input);

    /// Internal get next message. Basically you should get message from a internal queue.
    fn next(&self) -> (&'static str, Self::Input);

    /// run in delicated threads.
    fn run(self) -> JoinHandle<()>
    where
        Self: Sized + Send + 'static,
    {
        thread::spawn(move || loop {
            let (s, input) = self.next();
            self.dispatch(s, input);
            if s == "end" {
                break;
            }
        })
    }
}

/// The main logic
#[derive(Debug)]
struct WordFrequencyController {
    chan: Channel<String>,
    data_letterbox: MsgSender<String>,
    stop_words_letterbox: MsgSender<String>,
    counter_letterbox: MsgSender<String>,
}
impl WordFrequencyController {
    pub fn new(
        data_letterbox: MsgSender<String>,
        stop_words_letterbox: MsgSender<String>,
        counter_letterbox: MsgSender<String>,
    ) -> Self {
        Self {
            chan: Channel::default(),
            data_letterbox,
            stop_words_letterbox,
            counter_letterbox,
        }
    }
}
impl Letterbox for WordFrequencyController {
    type Input = String;
    fn dispatch(&self, command: &'static str, message: String) {
        let file = message;
        match command {
            "init" => {
                // have data and stop_words to init as well.
                self.data_letterbox.send(("init", file)).unwrap();
                self.stop_words_letterbox
                    .send(("init", "".to_string()))
                    .unwrap();
            }
            "run" => {
                // tell data to start processing data.
                self.data_letterbox.send(("run", "".to_string())).unwrap();
            }
            "end" => {
                self.data_letterbox.send(("end", "".to_string())).unwrap();
            }
            _ => {
                panic!(format!("unknown command: {}", command));
            }
        }
    }

    fn next(&self) -> (&'static str, Self::Input) {
        self.chan.rx.recv().unwrap()
    }

    fn letterbox(&self) -> MsgSender<Self::Input> {
        self.chan.tx.clone()
    }
}

/// Store & split the words.
#[derive(Debug)]
struct DataStorageManager {
    chan: Channel<String>,
    /// The words from file
    words: Mutex<Vec<String>>,
    stop_words_letterbox: MsgSender<String>,
}
impl DataStorageManager {
    pub fn new(stop_words_letterbox: MsgSender<String>) -> Self {
        Self {
            chan: Channel::default(),
            words: Default::default(),
            stop_words_letterbox,
        }
    }
}
impl Letterbox for DataStorageManager {
    type Input = String;
    fn letterbox(&self) -> MsgSender<Self::Input> {
        self.chan.tx.clone()
    }
    fn next(&self) -> (&'static str, Self::Input) {
        self.chan.rx.recv().unwrap()
    }
    fn dispatch(&self, command: &'static str, message: String) {
        let file = message;
        match command {
            "init" => {
                let f = File::open(file).unwrap();
                let reader = BufReader::new(f);
                let mut words = Vec::new();
                for line in reader.lines() {
                    let line = line.unwrap();
                    words.extend(
                        line.split(|ch: char| !ch.is_ascii_alphabetic())
                            .filter(|w| w.len() > 1)
                            .map(|s| s.to_lowercase()),
                    );
                }
                *self.words.lock().unwrap() = words;
            }
            "run" => {
                for word in self.words.lock().unwrap().drain(..) {
                    self.stop_words_letterbox.send(("filter", word)).unwrap();
                }
                self.stop_words_letterbox
                    .send(("finish", "".to_string()))
                    .unwrap();
            }
            "end" => {
                self.stop_words_letterbox
                    .send(("end", "".to_string()))
                    .unwrap();
            }
            _ => {
                panic!(format!("unknown command: {}", command));
            }
        }
    }
}

#[derive(Debug)]
struct StopWordsManager {
    chan: Channel<String>,
    stop_words: RwLock<HashSet<String>>,
    counter_letterbox: MsgSender<String>,
}
impl StopWordsManager {
    pub fn new(counter_letterbox: MsgSender<String>) -> Self {
        Self {
            chan: Channel::default(),
            stop_words: Default::default(),
            counter_letterbox,
        }
    }
}
impl Letterbox for StopWordsManager {
    type Input = String;
    fn letterbox(&self) -> MsgSender<Self::Input> {
        self.chan.tx.clone()
    }
    fn next(&self) -> (&'static str, Self::Input) {
        self.chan.rx.recv().unwrap()
    }
    fn dispatch(&self, command: &'static str, message: String) {
        let word = message;
        match command {
            "init" => {
                let f = File::open("../stop_words.txt").unwrap();
                let reader = BufReader::new(f);
                let mut stop_words = HashSet::new();
                for line in reader.lines() {
                    stop_words.extend(line.unwrap().split(',').map(|s| s.to_string()));
                }
                *self.stop_words.write().unwrap() = stop_words;
            }
            "filter" => {
                if !self.stop_words.read().unwrap().contains(&word) {
                    self.counter_letterbox.send(("word", word)).unwrap();
                }
            }
            "finish" => {
                self.counter_letterbox
                    .send(("top25", "".to_string()))
                    .unwrap();
            }
            "end" => {
                self.counter_letterbox
                    .send(("end", "".to_string()))
                    .unwrap();
            }
            _ => {
                panic!(format!("unknown command: {}", command));
            }
        }
    }
}

#[derive(Debug, Default)]
struct WordFrequencyManager {
    chan: Channel<String>,
    counter: Mutex<HashMap<String, usize>>,
}
impl Letterbox for WordFrequencyManager {
    type Input = String;
    fn letterbox(&self) -> MsgSender<Self::Input> {
        self.chan.tx.clone()
    }
    fn next(&self) -> (&'static str, Self::Input) {
        self.chan.rx.recv().unwrap()
    }
    fn dispatch(&self, command: &'static str, message: Self::Input) {
        match command {
            "word" => {
                *self.counter.lock().unwrap().entry(message).or_default() += 1;
            }
            "top25" => {
                let mut entries: Vec<(String, usize)> = self
                    .counter
                    .lock()
                    .unwrap()
                    .iter()
                    .map(|(w, c)| (w.clone(), *c))
                    .collect();
                entries.sort_unstable_by_key(|en| Reverse(en.1));
                entries.truncate(25);
                for (s, times) in entries {
                    println!("{} - {}", s, times);
                }
            }
            "end" => {}
            _ => {
                panic!(format!("unknown command: {}", command));
            }
        }
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let counter = WordFrequencyManager::default();
    let counter_letterbox = counter.letterbox();
    let counter_handler = counter.run();

    let stop_words = StopWordsManager::new(counter_letterbox.clone());
    let stop_words_letterbox = stop_words.letterbox();
    let stop_words_handler = stop_words.run();

    let data = DataStorageManager::new(stop_words_letterbox.clone());
    let data_letterbox = data.letterbox();
    let data_handler = data.run();

    let controller = WordFrequencyController::new(
        data_letterbox.clone(),
        stop_words_letterbox.clone(),
        counter_letterbox.clone(),
    );
    let controller_letterbox = controller.letterbox();
    let controller_handler = controller.run();

    controller_letterbox.send((
        "init",
        env::args()
            .nth(1)
            .expect("No file provided. Usage: ./29 <path>"),
    ))?;
    controller_letterbox.send(("run", "".to_string()))?;
    controller_letterbox.send(("end", "".to_string()))?;

    counter_handler.join().unwrap();
    stop_words_handler.join().unwrap();
    data_handler.join().unwrap();
    controller_handler.join().unwrap();

    Ok(())
}
