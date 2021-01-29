//! Style #16
//! ==============================
//! Constraints:
//! - Larger problem is decomposed into entities using some form of abstraction
//!   (objects, modules or similar)
//! - The entities are never called on directly for actions
//! - Existence of an infrastructure for publishing and subscribing to
//!   events, aka the bulletin board
//! - Entities post event subscriptions (aka 'wanted') to the bulletin
//!   board and publish events (aka 'offered') to the bulletin board. the
//!   bulletin board does all the event management and distribution
//! Possible names:
//! - Bulletin board
//! - Publish-Subscribe

//!  ====================== README ==================
//! Please know that this program will cause memory leakage and shall only be used as an exercise
//! for the publish-subscribe programming style.
//! The memory leakage is due to looped reference and can be solved by weak pointer, but that's not
//! worth it for this homework.
//!
//! Another thing worth mentioning is that, the publish-subscribe pattern is, to some extent, not
//! safe. Let's consider this: handler A is called for event a, during processing it publishes
//! another event b that implicitly invoke handler B. Now, if handler B publishes event a again
//! -- handler A is called twice! This may cause unexpected behavior. For example, a HashMap could
//! be modified while iteration.
//! Thankfully Rust by its design has detected that issue and forbid compiling in the first place.
//! However, in order to complete my homework, I used smart pointers and interior mutability to bypass
//! that - don't take me wrong, this code is still safe but will panic under the situation I explained
//! above through runtime borrow checking.
//! The cost is that this code will be harder to read - especially for those who are not
//! familiar with the Rust language.

use std::{
    cell::{Cell, RefCell},
    cmp::Reverse,
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, Read},
    panic,
    rc::Rc,
};

// ================ core logic ===============

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum EventKind {
    Run,
    Load,
    Start,
    Word,
    ValidWord,
    Eof,
    Print,
}
#[derive(Debug, Clone)]
enum Event {
    /// run the application
    Run {
        filename: String,
    },
    /// load the file (both the text file and the stop words)
    Load {
        filename: String,
    },
    /// start counting
    Start,
    /// count word
    Word(String),
    /// word that is not filtered out
    ValidWord(String),
    /// process & counting finished
    Eof,
    Print,
}
impl Event {
    pub fn kind(&self) -> EventKind {
        match &self {
            Event::Run { .. } => EventKind::Run,
            Event::Load { .. } => EventKind::Load,
            Event::Start => EventKind::Start,
            Event::Word(_) => EventKind::Word,
            Event::ValidWord(_) => EventKind::ValidWord,
            Event::Eof => EventKind::Eof,
            Event::Print => EventKind::Print,
        }
    }
}

/// This is the event handler. Ideally it should be immutable when called so ensure safety.
/// If you do need to modify the handler it self, use `RefCell` to bypass that.
trait EventHandler {
    // handle registered event
    fn handle(&self, event: Event);
}

#[derive(Default)]
struct EventManager {
    /// registered handlers
    handlers: HashMap<EventKind, Vec<Rc<dyn EventHandler>>>,
}
impl EventManager {
    pub fn publish(&self, event: Event) {
        // println!("event {:?}", event);
        if let Some(handlers) = self.handlers.get(&event.kind()) {
            handlers.iter().for_each(|handler| {
                handler.handle(event.clone());
            })
        }
    }
    /// register a handler for a given event kind.
    ///
    /// The `Rc` here stands for Reference Count smart pointer. A handler may be registered
    /// in multiple places and thus need a smart pointer to be shared.
    pub fn subscribe(&mut self, event_kind: EventKind, handler: Rc<dyn EventHandler>) {
        self.handlers.entry(event_kind).or_default().push(handler);
    }
}

// =========== exercise related logic ==============
struct Application {
    manager: Rc<RefCell<EventManager>>,
}
impl Application {
    pub fn new(manager: Rc<RefCell<EventManager>>) -> Rc<dyn EventHandler> {
        let me = Rc::new(Self {
            manager: manager.clone(),
        });
        manager.borrow_mut().subscribe(EventKind::Run, me.clone());
        manager.borrow_mut().subscribe(EventKind::Eof, me.clone());
        me
    }
}
impl EventHandler for Application {
    fn handle(&self, event: Event) {
        match event {
            Event::Run { filename } => {
                self.manager.borrow().publish(Event::Load { filename });
                self.manager.borrow().publish(Event::Start);
            }
            Event::Eof => self.manager.borrow().publish(Event::Print),
            _ => panic!("Unregistered event"),
        }
    }
}

/// Stores the data (words) and publishes them
struct DataStorage {
    manager: Rc<RefCell<EventManager>>,
    words: RefCell<Vec<String>>,
}
impl DataStorage {
    pub fn new(manager: Rc<RefCell<EventManager>>) -> Rc<dyn EventHandler> {
        let me = Rc::new(Self {
            manager: manager.clone(),
            words: RefCell::new(vec![]),
        });
        manager.borrow_mut().subscribe(EventKind::Load, me.clone());
        manager.borrow_mut().subscribe(EventKind::Start, me.clone());
        me
    }
}
impl EventHandler for DataStorage {
    fn handle(&self, event: Event) {
        match event {
            Event::Load { filename } => {
                let f = File::open(filename).expect("Failed to open file.");
                let reader = BufReader::new(f);
                let mut words = self.words.borrow_mut();
                for line in reader.lines() {
                    let line = line.unwrap();
                    words.extend(
                        line.split(|ch: char| !ch.is_ascii_alphabetic())
                            .filter(|s| s.len() > 1)
                            .map(|s| s.to_lowercase()),
                    );
                }
            }
            Event::Start => {
                for word in self.words.borrow().iter().cloned() {
                    self.manager.borrow().publish(Event::Word(word));
                }
                self.manager.borrow().publish(Event::Eof);
            }
            _ => panic!("Unregistered event"),
        }
    }
}

struct StopWordsFilter {
    manager: Rc<RefCell<EventManager>>,
    stop_words: RefCell<HashSet<String>>,
}
impl StopWordsFilter {
    pub fn new(manager: Rc<RefCell<EventManager>>) -> Rc<dyn EventHandler> {
        let me = Rc::new(Self {
            manager: manager.clone(),
            stop_words: RefCell::new(HashSet::new()),
        });
        manager.borrow_mut().subscribe(EventKind::Load, me.clone());
        manager.borrow_mut().subscribe(EventKind::Word, me.clone());
        me
    }
}
impl EventHandler for StopWordsFilter {
    fn handle(&self, event: Event) {
        match event {
            Event::Load { .. } => {
                let f = File::open("../stop_words.txt").expect("failed to open stop words file.");
                let mut reader = BufReader::new(f);
                let mut line = String::new();
                reader.read_to_string(&mut line).unwrap();
                self.stop_words
                    .borrow_mut()
                    .extend(line.split(',').map(|s| s.to_string()));
            }
            Event::Word(word) => {
                if !self.stop_words.borrow().contains(&word) {
                    self.manager.borrow().publish(Event::ValidWord(word));
                }
            }
            _ => panic!("Unregistered event"),
        }
    }
}

struct WordCounter {
    counter: RefCell<HashMap<String, usize>>,
}
impl WordCounter {
    pub fn new(manager: Rc<RefCell<EventManager>>) -> Rc<dyn EventHandler> {
        let me = Rc::new(Self {
            counter: RefCell::new(HashMap::new()),
        });
        manager
            .borrow_mut()
            .subscribe(EventKind::ValidWord, me.clone());
        manager.borrow_mut().subscribe(EventKind::Print, me.clone());
        me
    }
}
impl EventHandler for WordCounter {
    fn handle(&self, event: Event) {
        match event {
            Event::ValidWord(word) => {
                *self.counter.borrow_mut().entry(word).or_default() += 1;
            }
            Event::Print => {
                let mut entries: Vec<(String, usize)> = self
                    .counter
                    .borrow()
                    .iter()
                    .map(|(w, t)| (w.to_string(), *t))
                    .collect();
                entries.sort_unstable_by_key(|(_w, t)| Reverse(*t));
                entries.truncate(25);
                for (index, (word, times)) in entries.into_iter().enumerate() {
                    println!("[{:>2}] {:>20} - {:>4}", index + 1, word, times);
                }
            }
            _ => panic!("Unregistered event"),
        }
    }
}

struct ZWordHolic {
    zwords_count: Cell<usize>,
}
impl ZWordHolic {
    pub fn new(manager: Rc<RefCell<EventManager>>) -> Rc<dyn EventHandler> {
        let me = Rc::new(Self {
            zwords_count: Cell::new(0),
        });
        manager
            .borrow_mut()
            .subscribe(EventKind::ValidWord, me.clone());
        manager.borrow_mut().subscribe(EventKind::Print, me.clone());
        me
    }
}
impl EventHandler for ZWordHolic {
    fn handle(&self, event: Event) {
        match event {
            Event::ValidWord(word) => {
                if word.contains('z') {
                    self.zwords_count.set(self.zwords_count.get() + 1);
                }
            }
            Event::Print => {
                println!(
                    "Number of non-stop words with z: {}",
                    self.zwords_count.get()
                );
            }
            _ => panic!("Unregistered event"),
        }
    }
}

// ================= main ====================

fn main() {
    let event_manager = Rc::new(RefCell::new(EventManager::default()));
    let _data_storage = DataStorage::new(event_manager.clone());
    let _application = Application::new(event_manager.clone());
    let _stopwords_filter = StopWordsFilter::new(event_manager.clone());
    let _word_counter = WordCounter::new(event_manager.clone());
    let _z_word_holic = ZWordHolic::new(event_manager.clone());

    event_manager.borrow().publish(Event::Run {
        filename: std::env::args().skip(1).next().expect("Usage: ./16 <file>"),
    });
}
