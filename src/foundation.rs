use crate::{card::Card, foundation};

pub struct Foundation {
    pub cards: Vec<Vec<Option<Card>>>,
}

impl Foundation {
    pub fn new() -> Self {
        Self {
            cards: vec![vec![None], vec![None], vec![None], vec![None]],
        }
    }

    pub fn get_top_card(&self, position: (i8, i8)) -> Option<Card> {
        match self.cards[(position.0 - 3) as usize].last().copied() {
            Some(card) => card,
            _ => None,
        }
    }

    pub fn get_top_value(&self, position: (i8, i8)) -> u8 {
        match self.get_top_card(position) {
            Some(card) => card.rank,
            _ => 0,
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards[(card.suit - 1) as usize].push(Some(card))
    }

    pub fn remove_card(&mut self, position: (i8, i8)) {
        self.cards[(position.0 - 3) as usize].pop();
    }
}
