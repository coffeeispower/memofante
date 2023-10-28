use crate::discovered_word::DiscoveredWord;
use rusqlite::Connection;
pub fn add(word: String, connection: &Connection) -> Result<(), color_eyre::eyre::Error> {
    DiscoveredWord::add(word, connection)?;
    Ok(())
}
