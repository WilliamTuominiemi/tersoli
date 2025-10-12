use rand::prelude::*;

use crate::{card::Card, waste::Waste};

pub struct Stock {
    pub cards: Vec<Card>,
}

impl Stock {
    pub fn new() -> Self {
        let mut new_stock = Vec::with_capacity(52);
        for i in 1..=4 {
            for j in 1..=13 {
                new_stock.push(Card::new(i, j));
            }
        }

        new_stock.shuffle(&mut rand::rng());

        Self { cards: new_stock }
    }

    pub fn deal(&mut self) -> Card {
        self.cards.pop().expect("No more cards in stock")
    }

    pub fn reset(&mut self, waste: &Waste) {
        self.cards = waste.cards.iter().rev().cloned().collect();
    }
}
