use crate::discovered_word::DiscoveredWord;
use rusqlite::Connection;
pub fn add(word: String, connection: &Connection) -> Result<(), color_eyre::eyre::Error> {
    if jmdict::entries().all(|e| e.kanji_elements().all(|k| k.text != word)) {
        return Err(color_eyre::eyre::eyre!("Word not found in JMdict"));
    }
    DiscoveredWord::add(word, connection)?;
    Ok(())
}
