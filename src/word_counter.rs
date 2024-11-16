use std::collections::HashMap;
use std::io::{self, BufRead};

pub fn run() {
    let mut bigram_count = HashMap::new();
    let mut unigram_count = HashMap::new();

    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = line.unwrap();

        let mut words: Vec<String> = vec!["BOS".to_string()];
        words.extend(line.split(" ").map(|s| s.to_string()));
        words.push("EOS".to_string());

        if words.len() > 1 {
            for i in 0..words.len() - 1 {
                let first_word = words[i].clone();
                let second_word = words[i + 1].clone();

                *bigram_count
                    .entry((first_word.clone(), second_word))
                    .or_insert(0) += 1;
                *unigram_count.entry(first_word).or_insert(0) += 1;
            }
        }
    }

    for ((first, second), count) in bigram_count {
        if count > 0 {
            let first_count = *unigram_count.get(&first).unwrap_or(&0);
            let probability = count as f64 / first_count as f64;
            println!("{} {} {:.4}", second, first, probability);
        }
    }
}
