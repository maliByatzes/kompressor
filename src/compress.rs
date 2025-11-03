use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Symbol {
    value: char,
    count: i32,
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

        println!("counter: {:#?}", counter);
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
