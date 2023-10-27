use rusqlite::{params, Connection};

pub struct DiscoveredWord {
    word: String,
    success_reviews: i32,
    failed_reviews: i32,
    total_reviews: i32,
}
impl DiscoveredWord {
    pub fn word(&self) -> &str {
        &self.word
    }
    pub fn success_rate(&self) -> f64 {
        self.success_reviews as f64 / self.total_reviews as f64
    }
    pub fn success_reviews(&self) -> i32 {
        self.success_reviews
    }
    pub fn failed_reviews(&self) -> i32 {
        self.failed_reviews
    }
    pub fn total_reviews(&self) -> i32 {
        self.total_reviews
    }
    pub fn reviewed(&mut self, success: bool, db: &Connection) -> color_eyre::Result<()> {
        if success {
            self.success_reviews += 1;
        } else {
            self.failed_reviews += 1;
        }
        self.total_reviews += 1;
        db.execute(
            r#"
            UPDATE discovered_words SET success_reviews = ?, failed_reviews = ? WHERE word = ?
            "#,
            params![self.success_reviews, self.failed_reviews, self.word],
        )?;
        Ok(())
    }
    pub fn exists(word: impl Into<String>, db: &Connection) -> color_eyre::Result<bool> {
        let word = word.into();
        let mut stmt = db.prepare(
            r#"
            SELECT EXISTS(SELECT 1 FROM discovered_words WHERE word = ?)
            "#,
        )?;
        let mut rows = stmt.query(params![word])?;
        Ok(rows.next()?.is_some())
    }
    pub fn add(word: impl Into<String>, db: &Connection) -> color_eyre::Result<DiscoveredWord> {
        let word = word.into();
        db.execute(
            r#"
            INSERT INTO discovered_words (word) VALUES (?)
            "#,
            params![word],
        )?;
        Ok(DiscoveredWord {
            word,
            success_reviews: 0,
            failed_reviews: 0,
            total_reviews: 0,
        })
    }
    pub fn remove(word: impl Into<String>, db: &Connection) -> color_eyre::Result<()> {
        let word = word.into();
        db.execute(
            r#"
            DELETE FROM discovered_words WHERE word = ?
            "#,
            params![word],
        )?;
        Ok(())
    }
    pub fn list(db: &Connection) -> color_eyre::Result<Vec<DiscoveredWord>> {
        let mut stmt = db.prepare(
            r#"
            SELECT word, success_reviews, failed_reviews, total_reviews FROM discovered_words
            "#,
        )?;
        let rows = stmt.query_map(params![], |row| {
            Ok(DiscoveredWord {
                word: row.get(0)?,
                success_reviews: row.get(1)?,
                failed_reviews: row.get(2)?,
                total_reviews: row.get(3)?,
            })
        })?;

        rows.map(|r| r.map_err(color_eyre::eyre::Error::from))
            .collect()
    }
}
