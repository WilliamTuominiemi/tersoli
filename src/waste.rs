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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::suit::Suit;

    fn mock_waste() -> Waste {
        Waste::new()
    }

    #[test]
    fn test_add() {
        let mut waste = mock_waste();

        waste.add(Card::new(Suit::Spades, 5));

        assert_eq!(waste.cards[0].suit, Suit::Spades);
        assert_eq!(waste.cards[0].rank, 5);
    }

    #[test]
    fn test_remove() {
        let mut waste = mock_waste();
        waste.add(Card::new(Suit::Spades, 5));
        waste.remove();
        assert_eq!(waste.cards.len(), 0);
    }

    #[test]
    fn test_get_top_card() {
        let mut waste = mock_waste();

        let waste_card = Card::new(Suit::Clubs, 8);
        waste.add(waste_card);

        match waste.get_top_card() {
            Some(card) => {
                assert_eq!(card.rank, waste_card.rank);
                assert_eq!(card.suit, waste_card.suit);
            }
            _ => panic!("Couldn't get top card"),
        }
    }

    #[test]
    fn test_get_last_cards() {
        let mut waste = mock_waste();

        waste.add(Card::new(Suit::Clubs, 8));
        waste.add(Card::new(Suit::Hearts, 11));
        waste.add(Card::new(Suit::Hearts, 6));
        waste.add(Card::new(Suit::Spades, 1));

        let last_cards = waste.get_last_cards();

        assert_eq!(last_cards.len(), 3);

        assert_eq!(last_cards[0].suit, Suit::Spades);
        assert_eq!(last_cards[0].rank, 1);

        assert_eq!(last_cards[1].suit, Suit::Hearts);
        assert_eq!(last_cards[1].rank, 6);

        assert_eq!(last_cards[2].suit, Suit::Hearts);
        assert_eq!(last_cards[2].rank, 11);
    }

    #[test]
    fn test_reset() {
        let mut waste = mock_waste();

        waste.add(Card::new(Suit::Clubs, 8));
        waste.add(Card::new(Suit::Hearts, 11));

        waste.reset();

        assert_eq!(waste.cards.len(), 0);
    }
}
