use crate::suit::Suit;

#[derive(Copy, Clone, Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: u8,
}

impl Card {
    pub fn new(suit: Suit, rank: u8) -> Self {
        Self { suit, rank }
    }
}
