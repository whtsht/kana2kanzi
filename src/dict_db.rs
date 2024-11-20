use std::{
    collections::HashMap,
    fs,
    io::{self, BufRead},
};

#[derive(Debug, Clone, PartialEq)]
pub struct DictEntry {
    kana: String,
    kanzi: String,
}

#[derive(Debug, Default)]
pub struct DictDB {
    map: HashMap<String, Vec<String>>,
}

impl DictDB {
    const TXT_PATH: &'static str = "./data/dict.txt";

    pub fn new() -> Self {
        let mut map = HashMap::new();
        let file = fs::File::open(Self::TXT_PATH).unwrap();
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            let line = line.unwrap();
            let line = line.split(" ").map(|s| s.to_string()).collect::<Vec<_>>();
            let (kana, kanzi) = (line[0].clone(), line[1].clone());
            map.entry(kana).or_insert_with(Vec::new).push(kanzi);
        }
        Self { map }
    }

    pub fn get_kanzis(&self, kana: &str) -> Vec<String> {
        self.map.get(kana).cloned().unwrap_or_default()
    }
}
