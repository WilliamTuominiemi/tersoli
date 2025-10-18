use crate::{foundation::Foundation, suit::Suit};

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

pub fn check_win(foundation: &Foundation) -> bool {
    foundation
        .cards
        .iter()
        .all(|pile| pile.iter().filter(|card| card.is_some()).count() == 13)
}

#[cfg(test)]
mod tests {
    use crate::card::Card;

    use super::*;

    #[test]
    fn test_get_card_suit_index() {
        assert_eq!(get_card_suit_index(Suit::Hearts), 1);
    }

    #[test]
    fn test_get_suit_by_card_suit_index() {
        assert_eq!(get_suit_by_card_suit_index(2), Suit::Clubs);
    }

    #[test]
    fn test_check_win() {
        let mut mock_foundation = Foundation::new();

        assert!(!check_win(&mock_foundation));

        let mut index = 0;

        while index < 13 {
            let rank = index + 1;
            let spade = Card::new(Suit::Spades, rank);
            let heart = Card::new(Suit::Hearts, rank);
            let club = Card::new(Suit::Clubs, rank);
            let diamond = Card::new(Suit::Diamonds, rank);

            mock_foundation.add_card(spade, Suit::Spades);
            mock_foundation.add_card(heart, Suit::Hearts);
            mock_foundation.add_card(club, Suit::Clubs);
            mock_foundation.add_card(diamond, Suit::Diamonds);
            index += 1;
        }

        assert!(check_win(&mock_foundation));
    }
}
