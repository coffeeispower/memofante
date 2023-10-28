//! Implements spaced repetition with a iterator
//! 
//! The next word to review is the word with the lowest success rate after the current one, after all words have been reviewed, the iterator starts again.

use crate::discovered_word::DiscoveredWord;
use std::cell::RefCell;
use std::rc::Rc;

pub struct SpacedRepetition {
    words: Vec<Rc<RefCell<DiscoveredWord>>>,
    reviewed_words: Vec<String>,
}

impl SpacedRepetition {
    pub fn new(words: Vec<Rc<RefCell<DiscoveredWord>>>) -> Self {
        Self {
            words,
            reviewed_words: Vec::new(),
        }
    }
}

impl Iterator for SpacedRepetition {
    type Item = Rc<RefCell<DiscoveredWord>>;

    fn next(&mut self) -> Option<Self::Item> {
        // Get the next word based on your spaced repetition algorithm
        let next_word: Option<Self::Item> = self.words
            .iter()
            .filter(|word| !self.reviewed_words.iter().any(|w| w == word.borrow().word()))
            .min_by(|a, b| a.borrow().success_rate().partial_cmp(&b.borrow().success_rate()).unwrap_or(std::cmp::Ordering::Equal))
            .cloned();  // Clone the Rc<RefCell<DiscoveredWord>>

        // Mark the word as reviewed
        if let Some(word) = next_word.as_ref() {
            self.reviewed_words.push(word.borrow().word().to_string());
        }

        // Return the next word
        next_word
    }
}
