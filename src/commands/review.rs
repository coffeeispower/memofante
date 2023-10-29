mod spaced_repetition;

use std::{cell::RefCell, collections::HashSet, io::Write, rc::Rc};

use crate::{dictutils::EntryExt, discovered_word::DiscoveredWord};
use jmdict::GlossLanguage;
use rusqlite::Connection;

use self::spaced_repetition::SpacedRepetition;

use super::ReviewMode;

pub fn review(connection: &Connection, mode: ReviewMode) -> color_eyre::Result<()> {
    let words = DiscoveredWord::list(connection)?
        .into_iter()
        .map(|w| Rc::new(RefCell::new(w)))
        .collect::<Vec<_>>();
    let mut correct_answered_questions = HashSet::<u32>::new();
    for word in SpacedRepetition::new(words.clone()) {
        match mode {
            ReviewMode::Meaning => {
                review_word_meaning(word, &mut correct_answered_questions, connection)?
            }
            ReviewMode::Reading => {
                review_word_reading(word, &mut correct_answered_questions, connection)?
            }
        }
        if correct_answered_questions.len() == words.len() {
            break;
        }
    }
    println!("You finished reviewing all the ✨{}{}discovered words{}✨, go take a break, drink a coffee, do what you do best.", termion::style::Bold, termion::color::Fg(termion::color::Yellow), termion::style::Reset);
    Ok(())
}

fn review_word_meaning(
    word: Rc<RefCell<DiscoveredWord>>,
    correct_answered_questions: &mut HashSet<u32>,
    connection: &Connection,
) -> Result<(), color_eyre::eyre::Error> {
    let jmdict_entry = word.borrow().jmdict_entry();
    let correct_answers = jmdict_entry
        .senses()
        .map(|s| {
            s.glosses()
                .filter(|g| g.language == GlossLanguage::English)
                .map(|g| g.text.to_string())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    if jmdict_entry.usually_written_using_kana() {
        println!(
            "{}{}--- MEANING EXERCISE ---\nWord: {}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            jmdict_entry.common_text_form()
        );
    } else {
        println!(
            "{}{}--- MEANING EXERCISE ---\nWord: {}\n      {}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            jmdict_entry.common_text_form(),
            jmdict_entry.word_in_kana()
        );
    }
    println!("Type the meaning of this word:");
    print!("Answer: ");
    std::io::stdout().lock().flush()?;
    let mut answer = String::new();
    std::io::stdin().read_line(&mut answer)?;

    if correct_answers.iter().any(|ca| {
        ca.iter().any(|ca| {
            match strsim::normalized_levenshtein(
                ca.split_once('(')
                    .map(|(ca, _)| ca)
                    .unwrap_or(ca.as_str())
                    .trim(),
                answer.trim(),
            ) {
                f if (0.0..0.8).contains(&f) => false,
                f if (0.8..1.0).contains(&f) => {
                    println!("You made a typo, did you mean '{ca}'?");
                    true
                }
                _ => true,
            }
        })
    }) {
        correct_answered_questions.insert(word.borrow().entry_number());
        println!(
            "{}✓ Nailed it{}",
            termion::color::Fg(termion::color::Green),
            termion::style::Reset
        );
        if correct_answers.len() > 1 {
            println!("Other possible correct answers:");
            for gloss in correct_answers {
                if gloss.is_empty() {
                    continue;
                }
                println!(
                    "- {}",
                    gloss
                        .iter()
                        .map(|g| g.split_once('(').map(|(g, _)| g).unwrap_or(g).trim())
                        .collect::<Vec<&str>>()
                        .join(", ")
                );
            }
        }
        word.borrow_mut().reviewed(true, connection)?;
    } else {
        correct_answered_questions.remove(&word.borrow().entry_number());
        println!(
            "{}✘ Wrong answer{}",
            termion::color::Fg(termion::color::Red),
            termion::style::Reset
        );
        println!("Correct answers:");
        for gloss in correct_answers {
            if gloss.is_empty() {
                continue;
            }
            println!(
                "- {}",
                gloss
                    .iter()
                    .map(|g| g.split_once('(').map(|(g, _)| g).unwrap_or(g).trim())
                    .collect::<Vec<&str>>()
                    .join(", ")
            );
        }
        word.borrow_mut().reviewed(false, connection)?;
    }

    println!("Press enter to go to the next word");
    pause();
    Ok(())
}

fn review_word_reading(
    word: Rc<RefCell<DiscoveredWord>>,
    correct_answered_questions: &mut HashSet<u32>,
    connection: &Connection,
) -> Result<(), color_eyre::eyre::Error> {
    let jmdict_entry = word.borrow().jmdict_entry();
    if jmdict_entry.usually_written_using_kana() {
        return Ok(());
    }
    let correct_answers = jmdict_entry
        .reading_elements()
        .map(|r| r.text)
        .collect::<Vec<_>>();
    println!(
        "{}{}--- READING EXERCISE ---\nWord: {}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        jmdict_entry.common_text_form()
    );

    println!("Type the reading of this word:");
    print!("Answer: ");
    std::io::stdout().lock().flush()?;
    let mut answer = String::new();
    std::io::stdin().read_line(&mut answer)?;
    if correct_answers.iter().any(|ca| ca.trim() == answer.trim()) {
        correct_answered_questions.insert(word.borrow().entry_number());
        println!(
            "{}✓ Nailed it{}",
            termion::color::Fg(termion::color::Green),
            termion::style::Reset
        );
        if correct_answers.len() > 1 {
            println!("Other possible correct answers:");
            for reading in correct_answers {
                println!("- {reading}");
            }
        }
        word.borrow_mut().reviewed(true, connection)?;
    } else {
        correct_answered_questions.remove(&word.borrow().entry_number());
        println!(
            "{}✘ Wrong answer{}",
            termion::color::Fg(termion::color::Red),
            termion::style::Reset
        );
        println!("Correct answers:");
        for reading in correct_answers {
            println!("- {reading}");
        }
        word.borrow_mut().reviewed(false, connection)?;
    }

    println!("Press enter to go to the next word");
    pause();
    Ok(())
}
fn pause() {
    let mut _a = String::new();
    std::io::stdin()
        .read_line(&mut _a)
        .expect("Failed to read line");
}
