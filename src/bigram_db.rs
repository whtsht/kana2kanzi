use std::collections::HashMap;
use std::io::BufRead;

use crate::bin_loader::{load_from_file, save_to_file};

#[derive(Debug, Clone, PartialEq)]
pub struct Bigram {
    first: String,
    second: String,
    probability: f64,
}

#[derive(Debug)]
pub struct BigramDB {
    map: HashMap<(String, String), f64>,
}

impl BigramDB {
    const DB_PATH: &'static str = "./data/bigrams.bin";
    const TXT_PATH: &'static str = "./data/wakati.txt";
    pub const BOS: &'static str = "__BOS__";
    pub const EOS: &'static str = "__EOS__";

    fn remove_db() -> std::io::Result<()> {
        use std::fs;
        if fs::metadata(Self::DB_PATH).is_ok() {
            fs::remove_file(Self::DB_PATH)?;
        }
        Ok(())
    }

    pub fn gen_insert_data() -> (HashMap<(String, String), u64>, HashMap<String, u64>) {
        let mut bigram_count = HashMap::new();
        let mut unigram_count = HashMap::new();

        let file = std::fs::File::open(Self::TXT_PATH).unwrap();
        let reader = std::io::BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();

            let mut words: Vec<String> = vec!["__BOS__".to_string()];
            words.extend(line.split(" ").map(|s| s.to_string()));
            words.push("__EOS__".to_string());

            if words.len() > 1 {
                for i in 0..words.len() - 1 {
                    let first_word = words[i].clone();
                    let second_word = words[i + 1].clone();

                    *bigram_count
                        .entry((first_word.clone(), second_word))
                        .or_insert(0) += 1;
                    *unigram_count.entry(first_word).or_insert(0) += 1;
                }
                *unigram_count.entry(Self::EOS.to_string()).or_insert(0) += 1;
            }
        }

        (bigram_count, unigram_count)
    }

    pub fn build() {
        Self::remove_db().unwrap();

        let (bigram_count, unigram_count) = Self::gen_insert_data();

        let mut map = HashMap::new();
        for ((first, second), count) in bigram_count {
            if count > 0 {
                let first_count = *unigram_count.get(&first).unwrap_or(&0);
                let probability = count as f64 / first_count as f64;
                map.insert((first, second), probability);
            }
        }

        save_to_file(&map, Self::DB_PATH).unwrap();
    }

    pub fn new() -> Self {
        Self {
            map: load_from_file(Self::DB_PATH, &mut Vec::new()).unwrap(),
        }
    }

    pub fn get_probability(&self, first: &str, second: &str) -> f64 {
        self.map
            .get(&(first.to_string(), second.to_string()))
            .cloned()
            .unwrap_or(0.0)
    }
}

impl Default for BigramDB {
    fn default() -> Self {
        Self::new()
    }
}
