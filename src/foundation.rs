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

#[cfg(test)]
mod tests {
    use crate::foundation;

    use super::*;

    fn mock_foundation() -> Foundation {
        let mut mock = Foundation::new();

        mock.add_card(Card::new(2, 1), 2);
        mock.add_card(Card::new(2, 2), 2);
        mock.add_card(Card::new(3, 1), 3);

        mock
    }

    #[test]
    fn test_get_top_card() {
        let foundation = mock_foundation();

        let top_card = match foundation.get_top_card((4, 0)) {
            Some(card) => card,
            _ => panic!("No card found at position"),
        };

        assert_eq!(top_card.rank, 2);
        assert_eq!(top_card.suit, 2);
    }

    #[test]
    fn test_get_top_card_by_suit() {
        let foundation = mock_foundation();

        let top_card = match foundation.get_top_card_by_suit(3) {
            Some(card) => card,
            None => panic!("No card found for suit 3"),
        };

        assert_eq!(top_card.rank, 1);
        assert_eq!(top_card.suit, 3);
    }

    #[test]
    fn test_get_top_value() {
        let foundation = mock_foundation();

        assert_eq!(foundation.get_top_value((4, 0)), 2);
    }

    #[test]
    fn test_add_card() {
        let mut foundation = mock_foundation();

        // Cards which shouldn't be accepted & added
        let wrong_rank_card = Card::new(2, 8);
        foundation.add_card(wrong_rank_card, wrong_rank_card.suit);
        match foundation.get_top_card_by_suit(wrong_rank_card.suit) {
            Some(card) => assert_ne!(card.rank, wrong_rank_card.rank),
            _ => panic!("No card found for suit"),
        }

        let wrong_suit_card = Card::new(1, 2);
        foundation.add_card(wrong_suit_card, 3);
        match foundation.get_top_card_by_suit(3) {
            Some(card) => assert_ne!(card.rank, wrong_suit_card.rank),
            _ => panic!("No card found for suit"),
        }

        // Card which is added to parent card
        let add_to_parent_card = Card::new(2, 2);
        foundation.add_card(add_to_parent_card, add_to_parent_card.suit);
        match foundation.get_top_card_by_suit(add_to_parent_card.suit) {
            Some(card) => assert_eq!(card.rank, add_to_parent_card.rank),
            _ => panic!("No card found for suit"),
        }

        // Ace added as a first card
        let first_card = Card::new(1, 1);
        foundation.add_card(first_card, first_card.suit);
        match foundation.get_top_card_by_suit(first_card.suit) {
            Some(card) => assert_eq!(card.rank, first_card.rank),
            _ => panic!("No card found for suit"),
        }
    }

    #[test]
    fn test_remove() {
        let mut foundation = mock_foundation();

        let before = foundation.cards[0].len();

        foundation.remove_card((3, 0));

        assert_eq!(before - 1, foundation.cards[0].len());
    }

    #[test]
    fn test_snap_add() {
        let mut foundation = mock_foundation();

        assert!(!foundation.snap_add(Card::new(2, 9)));
        assert!(foundation.snap_add(Card::new(1, 1)));
        assert!(foundation.snap_add(Card::new(1, 2)));
    }
}
