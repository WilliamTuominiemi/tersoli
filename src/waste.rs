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
        if self.cards.len() == 0 {
            return None;
        }

        self.cards.last().copied()
    }

    pub fn reset(&mut self) {
        self.cards = vec![];
    }
}
