use crate::suit::Suit;

pub fn get_card_suit_index(suit: Suit) -> usize {
    match suit {
        Suit::Spades => 0,
        Suit::Hearts => 1,
        Suit::Clubs => 2,
        Suit::Diamonds => 3,
    }
}

pub fn get_suit_by_card_suit_index(index: usize) -> Suit {
    match index {
        0 => Suit::Spades,
        1 => Suit::Hearts,
        2 => Suit::Clubs,
        3 => Suit::Diamonds,
        _ => panic!("Suit index not mapped to any existing suit"),
    }
}
