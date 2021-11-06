use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{prelude::BufRead, BufReader};

struct WordCounter(HashMap<String, usize>);

impl WordCounter {
    fn new() -> WordCounter {
        WordCounter(HashMap::new())
    }

    fn increment(&mut self, word: &str) {
        let key = word.to_string();
        let count = self.0.entry(key).or_insert(0);
        *count += 1;
    }

    fn display(&self, filter_count: &String) {
        for (key, value) in self.0.iter() {
            println!("{}: {}", key, value)
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let filter_count = &args[2];
    println!("Processing file: {}", filename);
    let file = File::open(filename).expect("Could not find file");
    let reader = BufReader::new(file);
    let mut word_counter = WordCounter::new();

    for line in reader.lines() {
        let line = line.expect("Can't read line");
        let words = line.split(" ");
        for word in words {
            if word == " " {
                continue;
            } else {
                word_counter.increment(word);
            }
        }
    }

    word_counter.display(filter_count);
}
