use rand::prelude::*;

use crate::card::Card;

pub struct Stock {
    pub cards: Vec<Card>,
    rng: ThreadRng,
}

impl Stock {
    pub fn new() -> Self {
        let mut new_stock = Vec::with_capacity(52);
        for i in 1..=4 {
            for j in 1..=13 {
                new_stock.push(Card::new(i, j));
            }
        }
        Self {
            cards: new_stock,
            rng: rand::rng(),
        }
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut self.rng);
    }

    pub fn deal(&mut self) -> Card {
        self.cards.pop().expect("No more cards in stock")
    }
}
