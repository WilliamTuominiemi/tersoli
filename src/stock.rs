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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deal() {
        let mut mock_stock = Stock::new();
        let mock_stock_size = mock_stock.cards.len();
        let _card = mock_stock.deal();
        assert_eq!(mock_stock_size - 1, mock_stock.cards.len());
    }

    #[test]
    fn test_reset() {
        let mut mock_stock = Stock::new();
        let mut mock_waste = Waste::new();

        let mock_stock_card = mock_stock.deal();
        mock_waste.add(mock_stock_card);

        mock_stock.reset(&mock_waste);
        let reversed_cards: Vec<_> = mock_stock.cards.clone().into_iter().rev().collect();

        assert_eq!(mock_stock.cards[0].rank, reversed_cards[0].rank);
        assert_eq!(mock_stock.cards[0].suit, reversed_cards[0].suit);
    }
}
