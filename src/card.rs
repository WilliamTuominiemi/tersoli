#[derive(Copy, Clone)]
pub struct Card {
    pub suit: u8,
    pub rank: u8,
}

impl Card {
    pub fn new(suit: u8, rank: u8) -> Self {
        Self { suit, rank }
    }
}
