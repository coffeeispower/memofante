use crate::{discovered_word::DiscoveredWord, dictutils::EntryExt};
use rusqlite::Connection;
use std::io::Write;

pub fn list(connection: &Connection) -> Result<(), color_eyre::eyre::Error> {
    let discovered_words = DiscoveredWord::list(connection)?;
    let mut stdout = std::io::stdout().lock();
    writeln!(&mut stdout, "Discovered words:")?;
    for word in discovered_words {
        if word.total_reviews() == 0 {
            writeln!(
                &mut stdout,
                "- {}{}{} NEW {} {}",
                termion::style::Bold,
                termion::color::Bg(termion::color::Yellow),
                termion::color::Fg(termion::color::Black),
                termion::style::Reset,
                word.jmdict_entry().common_text_form(),
            )?;
            continue;
        }
        writeln!(
            &mut stdout,
            "- {} (Successes: {}, Fails: {}, Success Rate: {:.1}%)",
            word.jmdict_entry().common_text_form(),
            word.success_reviews(),
            word.failed_reviews(),
            word.success_rate() * 100f64
        )?;
    }
    Ok(())
}
