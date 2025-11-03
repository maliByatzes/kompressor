use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Symbol {
    value: char,
    count: i32,
}

impl Ord for Symbol {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .count
            .cmp(&self.count)
            .then_with(|| self.value.cmp(&other.value))
    }
}

impl PartialOrd for Symbol {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Compress {
    contents: String,
}

impl Compress {
    pub fn new(s: &str) -> Self {
        Compress {
            contents: s.to_owned(),
        }
    }

    pub fn compress(&self) {
        let counter = self.freq_counter();

        let heap = BinaryHeap::from(counter);

        for sym in &heap {
            println!("{:?}", sym);
        }
    }

    fn freq_counter(&self) -> Vec<Symbol> {
        let mut temp_counter = HashMap::new();

        for chr in self.contents.chars() {
            temp_counter.entry(chr).and_modify(|e| *e += 1).or_insert(1);
        }

        let counter: Vec<Symbol> = temp_counter
            .iter()
            .map(|(&key, &val)| Symbol {
                value: key,
                count: val,
            })
            .collect();

        counter
    }
}
