use rusqlite::Connection;
use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Debug, Clone, PartialEq)]
pub struct Bigram {
    first: String,
    second: String,
    probability: f64,
}

#[derive(Debug)]
pub struct BigramDB {
    conn: Connection,
}

impl BigramDB {
    const DB_PATH: &'static str = "./data/bigrams.db";
    pub const BOS: &'static str = "__BOS__";
    pub const EOS: &'static str = "__EOS__";

    fn remove_db() -> std::io::Result<()> {
        use std::fs;
        if fs::metadata(Self::DB_PATH).is_ok() {
            fs::remove_file(Self::DB_PATH)?;
        }
        Ok(())
    }

    fn new_db() -> rusqlite::Result<()> {
        Self::remove_db().unwrap();
        let conn = Connection::open(Self::DB_PATH)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS bigrams (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            first       TEXT NOT NULL,
            second      TEXT NOT NULL,
            probability REAL NOT NULL,
            UNIQUE(second, first)
        )",
            (),
        )?;
        conn.execute(
            "CREATE INDEX IF NOT EXISTS index_bigrams_first_second ON bigrams(first, second)",
            (),
        )?;

        Self::insert_data()?;

        Ok(())
    }

    pub fn gen_insert_data() -> (HashMap<(String, String), u64>, HashMap<String, u64>) {
        let mut bigram_count = HashMap::new();
        let mut unigram_count = HashMap::new();

        let stdin = io::stdin();

        for line in stdin.lock().lines() {
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

    fn insert_data() -> rusqlite::Result<()> {
        let mut conn = Connection::open(Self::DB_PATH)?;
        let (bigram_count, unigram_count) = Self::gen_insert_data();

        let tx = conn.transaction()?;

        {
            let mut stmt = tx.prepare(
                "INSERT OR IGNORE INTO bigrams (first, second, probability) VALUES (?1, ?2, ?3)",
            )?;
            for ((first, second), count) in bigram_count {
                if count > 0 {
                    let first_count = *unigram_count.get(&first).unwrap_or(&0);
                    let probability = count as f64 / first_count as f64;
                    stmt.execute((first, second, probability))?;
                }
            }
        }

        tx.commit()?;

        Ok(())
    }

    pub fn new() -> rusqlite::Result<Self> {
        Ok(Self {
            conn: Connection::open(Self::DB_PATH)?,
        })
    }

    pub fn new_with_build() -> rusqlite::Result<Self> {
        Self::new_db()?;
        Self::new()
    }

    pub fn new_with_conn(conn: Connection) -> Self {
        Self { conn }
    }

    pub fn get_probability(&self, first: &str, second: &str) -> rusqlite::Result<f64> {
        let mut stmt = self.conn.prepare(
            "SELECT
                probability 
             FROM
                bigrams 
             WHERE
                first = ?1 AND second = ?2",
        )?;

        let probability = stmt
            .query_row((first, second), |row| row.get(0))
            .unwrap_or(0.0);

        Ok(probability)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn setup_test_db() -> rusqlite::Result<BigramDB> {
        let conn = Connection::open_in_memory()?;
        let db = BigramDB { conn };

        db.conn.execute(
            "CREATE TABLE IF NOT EXISTS bigrams (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                first       TEXT NOT NULL,
                second      TEXT NOT NULL,
                probability REAL NOT NULL,
                UNIQUE(first, second)
            )",
            (),
        )?;

        db.conn.execute(
            "CREATE INDEX IF NOT EXISTS index_bigrams_first_second ON bigrams(first, second)",
            (),
        )?;

        db.conn.execute(
            "INSERT INTO bigrams (first, second, probability) VALUES ('さ', 'は', 0.5)",
            (),
        )?;
        db.conn.execute(
            "INSERT INTO bigrams (first, second, probability) VALUES ('さ', 'に', 0.8)",
            (),
        )?;

        Ok(db)
    }

    #[test]
    fn test_get_probability_existing_bigram() {
        let db = setup_test_db().unwrap();
        let probability = db.get_probability("さ", "は").unwrap();
        assert_eq!(probability, 0.5);
    }

    #[test]
    fn test_get_probability_non_existing_bigram() {
        let db = setup_test_db().unwrap();
        let probability = db.get_probability("ない", "言葉").unwrap();
        assert_eq!(probability, 0.0);
    }
}
