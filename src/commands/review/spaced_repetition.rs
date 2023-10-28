use rand::distributions::WeightedIndex;
use rand::prelude::*;
use crate::discovered_word::DiscoveredWord;
use std::cell::RefCell;
use std::rc::Rc;

pub struct SpacedRepetition {
    words: Vec<Rc<RefCell<DiscoveredWord>>>,
}

impl SpacedRepetition {
    pub fn new(words: Vec<Rc<RefCell<DiscoveredWord>>>) -> Self {
        Self { words }
    }
}

impl Iterator for SpacedRepetition {
    type Item = Rc<RefCell<DiscoveredWord>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut rng = rand::thread_rng();
        let rand_weights_iter = self
            .words
            .iter()
            .map(|word| calculate_score(&word.borrow()));
        let dist = WeightedIndex::new(rand_weights_iter).ok()?;
        let chosen_word_index = dist.sample(&mut rng);
        Some(self.words[chosen_word_index].clone())
    }
}
fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}
fn calculate_score(word: &DiscoveredWord) -> f64 {
    const FAILURE_WEIGHT: f64 = 1.0;
    const FEW_REVIEWS_WEIGHT: f64 = 5.0;
    let failure_score = FAILURE_WEIGHT * word.failure_rate() as f64;
    let few_reviews_score = FEW_REVIEWS_WEIGHT * (1.0 - sigmoid(word.total_reviews() as f64));
    failure_score + few_reviews_score
}