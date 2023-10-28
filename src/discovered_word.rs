use rusqlite::{params, Connection};

pub struct DiscoveredWord {
    entry_number: u32,
    success_reviews: u32,
    failed_reviews: u32,
    total_reviews: u32,
}
impl DiscoveredWord {
    pub fn jmdict_entry(&self) -> jmdict::Entry {
        jmdict::entries()
            .find(|e| e.number == self.entry_number)
            .expect("Entry not found in jmdict")
    }
    pub fn entry_number(&self) -> u32 {
        self.entry_number
    }
    pub fn success_rate(&self) -> f64 {
        self.success_reviews as f64 / self.total_reviews as f64
    }
    pub fn success_reviews(&self) -> u32 {
        self.success_reviews
    }
    pub fn failed_reviews(&self) -> u32 {
        self.failed_reviews
    }
    pub fn total_reviews(&self) -> u32 {
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
            UPDATE discovered_words SET success_reviews = ?, failed_reviews = ? WHERE entry_number = ?
            "#,
            params![self.success_reviews, self.failed_reviews, self.entry_number],
        )?;
        Ok(())
    }
    pub fn exists(word: impl Into<String>, db: &Connection) -> color_eyre::Result<bool> {
        let word = word.into();
        let entry = crate::dictutils::find_entry_by_word(word.as_str())
            .ok_or_else(|| color_eyre::eyre::eyre!("Word not found in the dictionary"))?;
        let mut stmt = db.prepare(
            r#"
            SELECT EXISTS(SELECT 1 FROM discovered_words WHERE entry_number = ?)
            "#,
        )?;
        let mut rows = stmt.query(params![entry.number])?;
        Ok(rows.next()?.is_some())
    }
    pub fn add(word: impl Into<String>, db: &Connection) -> color_eyre::Result<DiscoveredWord> {
        let word = word.into();
        let entry = crate::dictutils::find_entry_by_word(word.as_str())
            .ok_or_else(|| color_eyre::eyre::eyre!("Word not found in the dictionary"))?;
        db.execute(
            r#"
            INSERT INTO discovered_words (entry_number) VALUES (?)
            "#,
            params![entry.number],
        )?;
        Ok(DiscoveredWord {
            entry_number: entry.number,
            success_reviews: 0,
            failed_reviews: 0,
            total_reviews: 0,
        })
    }
    pub fn remove(word: impl Into<String>, db: &Connection) -> color_eyre::Result<()> {
        let word = word.into();
        let entry = crate::dictutils::find_entry_by_word(word.as_str())
            .ok_or_else(|| color_eyre::eyre::eyre!("Word not found in the dictionary"))?;
        db.execute(
            r#"
            DELETE FROM discovered_words WHERE entry_number = ?
            "#,
            params![entry.number],
        )?;
        Ok(())
    }
    pub fn list(db: &Connection) -> color_eyre::Result<Vec<DiscoveredWord>> {
        let mut stmt = db.prepare(
            r#"
            SELECT entry_number, success_reviews, failed_reviews, total_reviews FROM discovered_words
            "#,
        )?;
        let rows = stmt.query_map(params![], |row| {
            Ok(DiscoveredWord {
                entry_number: row.get(0)?,
                success_reviews: row.get(1)?,
                failed_reviews: row.get(2)?,
                total_reviews: row.get(3)?,
            })
        })?;

        rows.map(|r| r.map_err(color_eyre::eyre::Error::from))
            .collect()
    }
}
