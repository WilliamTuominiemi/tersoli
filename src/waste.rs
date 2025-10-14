use std::cmp::min;

use crate::card::Card;

pub struct Waste {
    pub cards: Vec<Card>,
}

impl Waste {
    pub fn new() -> Self {
        Self { cards: vec![] }
    }

    pub fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn remove(&mut self) {
        self.cards.pop();
    }

    pub fn get_top_card(&self) -> Option<Card> {
        if self.cards.is_empty() {
            return None;
        }

        self.cards.last().copied()
    }

    pub fn get_last_cards(&self) -> Vec<Card> {
        if self.cards.is_empty() {
            return vec![];
        }

        let amount_to_take = min(self.cards.len(), 3);

        self.cards
            .iter()
            .rev()
            .take(amount_to_take)
            .copied()
            .collect()
    }

    pub fn reset(&mut self) {
        self.cards = vec![];
    }
}
