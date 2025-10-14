use crate::card::Card;

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

    pub fn get_top_card_by_suit(&self, suit: u8) -> Option<Card> {
        match self.cards[(suit - 1) as usize].last().copied() {
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

    pub fn add_card(&mut self, card: Card, to_suit: u8) -> bool {
        if card.suit != to_suit {
            return false;
        }

        let parent_card = match self.get_top_card_by_suit(card.suit) {
            Some(parent) => parent,
            _ => {
                if card.rank == 1 {
                    self.cards[(card.suit - 1) as usize].push(Some(card));
                    return true;
                }
                return false;
            }
        };

        if card.rank != parent_card.rank + 1 {
            return false;
        }

        self.cards[(card.suit - 1) as usize].push(Some(card));
        return true;
    }

    pub fn remove_card(&mut self, position: (i8, i8)) {
        self.cards[(position.0 - 3) as usize].pop();
    }

    pub fn snap_add(&mut self, card: Card) -> bool {
        let idx = (card.suit - 1) as usize;
        let foundation_card_rank = match self.cards[idx].last().copied() {
            Some(foundation_card) => match foundation_card {
                Some(foundation_card) => foundation_card.rank,
                _ => 0,
            },
            _ => 0,
        };

        if foundation_card_rank != card.rank - 1 {
            return false;
        } else {
            self.add_card(card, card.suit);
            return true;
        }
    }
}
