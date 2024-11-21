use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::{
    fs,
    io::{self, BufRead},
};

use bincode::{deserialize, serialize};

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
    const DB_PATH: &'static str = "./data/dict.bin";

    fn remove_db() -> std::io::Result<()> {
        use std::fs;
        if fs::metadata(Self::DB_PATH).is_ok() {
            fs::remove_file(Self::DB_PATH)?;
        }
        Ok(())
    }

    pub fn build() {
        Self::remove_db().unwrap();

        let mut map = HashMap::new();
        let file = fs::File::open(Self::TXT_PATH).unwrap();
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            let line = line.unwrap();
            let line = line.split(" ").map(|s| s.to_string()).collect::<Vec<_>>();
            let (kana, kanzi) = (line[0].clone(), line[1].clone());
            map.entry(kana).or_insert_with(Vec::new).push(kanzi);
        }

        let encoded: Vec<u8> = serialize(&map).unwrap();
        let mut file = File::create(Self::DB_PATH).unwrap();
        file.write_all(&encoded).unwrap();
    }

    pub fn new() -> Self {
        let mut file = File::open(Self::DB_PATH).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        let map = deserialize(&buffer).unwrap();
        Self { map }
    }

    pub fn get_kanzis(&self, kana: &str) -> Vec<String> {
        self.map.get(kana).cloned().unwrap_or_default()
    }
}
