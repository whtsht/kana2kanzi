use std::io::{self, BufRead};

use rusqlite::Connection;

#[derive(Debug, Clone, PartialEq)]
pub struct DictEntry {
    kana: String,
    kanzi: String,
}

#[derive(Debug)]
pub struct DictDB {
    conn: Connection,
}

impl DictDB {
    const DB_PATH: &'static str = "./data/dict.db";

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
            "CREATE TABLE IF NOT EXISTS dict (
               id      INTEGER PRIMARY KEY AUTOINCREMENT,
               kana    TEXT NOT NULL,
               kanzi   TEXT NOT NULL
            )",
            (),
        )?;
        conn.execute(
            "CREATE INDEX IF NOT EXISTS index_dict_kana ON dict(kana)",
            (),
        )?;

        Self::insert_data()?;

        Ok(())
    }

    fn insert_data() -> rusqlite::Result<()> {
        let mut conn = Connection::open(Self::DB_PATH)?;
        let tx = conn.transaction()?;
        {
            let stdin = io::stdin();

            let mut stmt =
                tx.prepare("INSERT OR IGNORE INTO dict (kana, kanzi) VALUES (?1, ?2)")?;

            for line in stdin.lock().lines() {
                let line = line.unwrap();
                let line = line.split(" ").map(|s| s.to_string()).collect::<Vec<_>>();
                let (kana, kanzi) = (line[0].clone(), line[1].clone());
                stmt.execute([kana, kanzi])?;
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

    pub fn get_kanzis(&self, kana: &str) -> rusqlite::Result<Vec<String>> {
        let mut stmt = self
            .conn
            .prepare("SELECT kanzi FROM dict WHERE kana = ?1")?;

        let kanzis = stmt
            .query_map([kana], |row| row.get::<usize, String>(0))?
            .flatten()
            .collect();

        Ok(kanzis)
    }
}
