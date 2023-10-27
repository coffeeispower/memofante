use crate::discovered_word::DiscoveredWord;
use rusqlite::Connection;

pub fn remove(word: String, connection: &Connection) -> color_eyre::Result<()> {
    DiscoveredWord::remove(&word, connection)?;
    Ok(println!("Removed word '{word}' from discovered words"))
}
