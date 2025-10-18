use crate::{card::Card, location::Location, suit::Suit, utils::get_card_suit_index};

pub struct Foundation {
    pub cards: Vec<Vec<Option<Card>>>,
}

impl Foundation {
    pub fn new() -> Self {
        Self {
            cards: vec![vec![None], vec![None], vec![None], vec![None]],
        }
    }

    pub fn get_top_card(&self, location: Location) -> Option<Card> {
        if let Location::Foundation(index) = location {
            self.cards[index].last().copied().unwrap_or_default()
        } else {
            None
        }
    }

    pub fn get_top_card_by_suit(&self, suit: Suit) -> Option<Card> {
        self.cards[get_card_suit_index(suit)]
            .last()
            .copied()
            .unwrap_or_default()
    }

    pub fn get_top_value(&self, location: Location) -> u8 {
        match self.get_top_card(location) {
            Some(card) => card.rank,
            _ => 0,
        }
    }

    pub fn add_card(&mut self, card: Card, to_suit: Suit) -> bool {
        if card.suit != to_suit {
            return false;
        }

        let card_suit_index = get_card_suit_index(card.suit);

        let parent_card = match self.get_top_card_by_suit(card.suit) {
            Some(parent) => parent,
            _ => {
                if card.rank == 1 {
                    self.cards[card_suit_index].push(Some(card));
                    return true;
                }
                return false;
            }
        };

        if card.rank != parent_card.rank + 1 {
            return false;
        }

        self.cards[card_suit_index].push(Some(card));
        true
    }

    pub fn remove_card(&mut self, location: Location) {
        if let Location::Foundation(index) = location {
            self.cards[index].pop();
        } else {
            unreachable!("Can only have foundation location enum in foundation")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_foundation() -> Foundation {
        let mut mock = Foundation::new();

        mock.add_card(Card::new(Suit::Hearts, 1), Suit::Hearts);
        mock.add_card(Card::new(Suit::Hearts, 2), Suit::Hearts);
        mock.add_card(Card::new(Suit::Clubs, 1), Suit::Clubs);

        mock
    }

    #[test]
    fn test_get_top_card() {
        let foundation = mock_foundation();

        println!("{:?}", foundation.cards);

        let top_card = match foundation.get_top_card(Location::Foundation(1)) {
            Some(card) => card,
            _ => panic!("No card found at position"),
        };

        assert_eq!(top_card.rank, 2);
        assert_eq!(top_card.suit, Suit::Hearts);
    }

    #[test]
    fn test_get_top_card_by_suit() {
        let foundation = mock_foundation();

        let top_card = match foundation.get_top_card_by_suit(Suit::Clubs) {
            Some(card) => card,
            None => panic!("No card found for suit 3"),
        };

        assert_eq!(top_card.rank, 1);
        assert_eq!(top_card.suit, Suit::Clubs);
    }

    #[test]
    fn test_get_top_value() {
        let foundation = mock_foundation();

        assert_eq!(foundation.get_top_value(Location::Foundation(1)), 2);
    }

    #[test]
    fn test_add_card() {
        let mut foundation = mock_foundation();

        // Cards which shouldn't be accepted & added
        let wrong_rank_card = Card::new(Suit::Hearts, 8);
        foundation.add_card(wrong_rank_card, wrong_rank_card.suit);
        match foundation.get_top_card_by_suit(wrong_rank_card.suit) {
            Some(card) => assert_ne!(card.rank, wrong_rank_card.rank),
            _ => panic!("No card found for suit"),
        }

        let wrong_suit_card = Card::new(Suit::Spades, 2);
        foundation.add_card(wrong_suit_card, Suit::Clubs);
        match foundation.get_top_card_by_suit(Suit::Clubs) {
            Some(card) => assert_ne!(card.rank, wrong_suit_card.rank),
            _ => panic!("No card found for suit"),
        }

        // Card which is added to parent card
        let add_to_parent_card = Card::new(Suit::Hearts, 2);
        foundation.add_card(add_to_parent_card, add_to_parent_card.suit);
        match foundation.get_top_card_by_suit(add_to_parent_card.suit) {
            Some(card) => assert_eq!(card.rank, add_to_parent_card.rank),
            _ => panic!("No card found for suit"),
        }

        // Ace added as a first card
        let first_card = Card::new(Suit::Spades, 1);
        foundation.add_card(first_card, Suit::Spades);
        match foundation.get_top_card_by_suit(first_card.suit) {
            Some(card) => assert_eq!(card.rank, first_card.rank),
            _ => panic!("No card found for suit"),
        }
    }

    #[test]
    fn test_remove() {
        let mut foundation = mock_foundation();

        let before = foundation.cards[0].len();

        foundation.remove_card(Location::Foundation(0));

        assert_eq!(before - 1, foundation.cards[0].len());
    }
}
