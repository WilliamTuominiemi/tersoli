use crate::card::Card;
use crate::location::Location;
use crate::stock::Stock;

pub struct Tableau {
    pub cards: Vec<Vec<Card>>,
    pub cutoffs: Vec<u8>,
}

impl Tableau {
    pub fn new() -> Self {
        Self {
            cards: vec![],
            cutoffs: vec![0, 1, 2, 3, 4, 5, 6],
        }
    }

    pub fn initialize(&mut self, stock: &mut Stock) {
        self.cards.clear();
        for i in 0..7 {
            let mut row = Vec::new();
            for _j in 0..=i {
                row.push(stock.deal());
            }
            self.cards.push(row);
        }
    }

    pub fn get_top_card(&mut self, location: Location) -> Option<Card> {
        if let Location::Tableau(index) = location {
            if self.cards[index].is_empty() {
                return None;
            }

            self.cards[index].last().copied()
        } else {
            unreachable!("Expected Location::tableau, got something else :(");
        }
    }

    fn find_card(&self, location: Location, rank: u8, suit: Option<u8>) -> Option<usize> {
        match location {
            Location::Tableau(index) => {
                let visible = self.cutoffs[index] as usize;
                self.cards[index]
                    .iter()
                    .enumerate()
                    .skip(visible)
                    .position(|(_, &card)| {
                        card.rank == rank
                            && match suit {
                                Some(s) => card.suit % 2 == s,
                                None => true,
                            }
                    })
                    .map(|index| index + visible)
            }
            _ => unreachable!("can only find tableau cards"),
        }
    }

    fn take_cards_at_index(&mut self, location: Location, index: usize) -> Vec<Card> {
        match location {
            Location::Tableau(column) => {
                if column >= self.cards.len() || index > self.cards[column].len() {
                    return vec![];
                }

                if self.cutoffs[column] > 0 && (index as u8) <= self.cutoffs[column] {
                    self.cutoffs[column] -= 1;
                }

                self.cards[column].split_off(index)
            }
            _ => unreachable!("can only take tableau cards"),
        }
    }

    pub fn add_card(&mut self, to: Location, card: Card) -> bool {
        if let Location::Tableau(index) = to {
            let parent_card = match self.get_top_card(to) {
                Some(parent) => parent,
                _ => {
                    if card.rank == 13 {
                        self.cards[index].push(card);
                        return true;
                    }
                    return false;
                }
            };

            if card.suit % 2 == parent_card.suit % 2 {
                return false;
            }

            if card.rank != parent_card.rank - 1 {
                return false;
            }

            self.cards[index].push(card);
            return true;
        }
        return false;
    }

    pub fn get_visible_cards(&self, column: usize) -> Vec<Card> {
        self.cards[column][(self.cutoffs[column] as usize)..].to_vec()
    }

    pub fn update_cutoffs(&mut self, index: usize) {
        if index >= self.cutoffs.len() || index >= self.cards.len() {
            return;
        }

        let cutoff = self.cutoffs[index];
        let card_index = self.cards[index].len() as u8;

        if cutoff > 0 && card_index > 0 && cutoff == card_index - 1 {
            self.cutoffs[index] -= 1;
        }
    }

    fn move_cards(
        &mut self,
        needed_rank: u8,
        needed_suit: Option<u8>,
        from: Location,
        to: Location,
    ) {
        let card_index = match self.find_card(from, needed_rank, needed_suit) {
            Some(index) => index,
            _ => return,
        };

        let cards_to_move = self.take_cards_at_index(from, card_index);

        for card in cards_to_move {
            self.add_card(to, card);
        }

        return;
    }

    pub fn try_to_move_between_tableau(&mut self, from: Location, to: Location) {
        let to_card: Card = match self.get_top_card(to) {
            Some(card) => card,
            _ => {
                self.move_cards(13, None, from, to);
                return;
            }
        };

        let needed_rank = to_card.rank - 1;
        let needed_suit = (to_card.suit + 1) % 2;

        self.move_cards(needed_rank, Some(needed_suit), from, to);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_tableau() -> Tableau {
        let mut stock = Stock::new();
        let mut tableau = Tableau::new();

        tableau.initialize(&mut stock);

        tableau
    }

    #[test]
    fn test_get_top_card() {
        let mut tableau = mock_tableau();
        assert!(tableau.get_top_card(Location::Tableau(1)).is_some());
    }

    #[test]
    fn test_add_card() {
        let mut tableau = mock_tableau();

        let mut current_card = match tableau.get_top_card(Location::Tableau(2)) {
            Some(card) => card,
            _ => panic!("Top card not found"),
        };

        while current_card.rank < 5 {
            tableau = mock_tableau();
            current_card = match tableau.get_top_card(Location::Tableau(2)) {
                Some(card) => card,
                _ => panic!("Top card not found"),
            };
        }

        let first_card = Card::new((current_card.suit % 2) + 1, current_card.rank - 1);
        assert!(tableau.add_card(Location::Tableau(2), first_card));
        assert_eq!(tableau.cards[2].len(), 4);

        let wrong_number_card = Card::new((first_card.suit % 2) + 1, 12);
        assert!(!tableau.add_card(Location::Tableau(2), wrong_number_card));
        assert_eq!(tableau.cards[2].len(), 4);

        let wrong_suit_card = Card::new(first_card.suit % 2, first_card.rank - 1);
        assert!(!tableau.add_card(Location::Tableau(2), wrong_suit_card));
        assert_eq!(tableau.cards[2].len(), 4);

        tableau.take_cards_at_index(Location::Tableau(0), 0);
        assert_eq!(tableau.cards[0].len(), 0);

        assert!(!tableau.add_card(Location::Tableau(0), first_card));
        assert_eq!(tableau.cards[0].len(), 0);

        let king_card = Card::new(1, 13);
        assert!(tableau.add_card(Location::Tableau(0), king_card));
        assert_eq!(tableau.cards[0].len(), 1);
    }

    #[test]
    fn test_try_to_move_between_tableau() {
        let mut tableau = Tableau::new();

        tableau.cards = vec![
            vec![Card::new(1, 5)],
            vec![Card::new(1, 6), Card::new(2, 4)],
            vec![Card::new(1, 7)],
            vec![],
            vec![Card::new(2, 13)],
            vec![],
            vec![],
        ];

        tableau.try_to_move_between_tableau(Location::Tableau(1), Location::Tableau(0));
        assert_eq!(tableau.cards[0].len(), 2);
        assert_eq!(tableau.cards[0][1].rank, 4);
        assert_eq!(tableau.cards[1].len(), 1);
    }
}
