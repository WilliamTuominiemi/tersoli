use crate::card::Card;
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

    pub fn get_top_card(&mut self, position: (i8, i8)) -> Option<Card> {
        if self.cards[position.0 as usize].len() == 0 {
            return None;
        }

        self.cards[position.0 as usize].last().copied()
    }

    pub fn find_card(&self, position: (i8, i8), rank: u8, suit: Option<u8>) -> Option<usize> {
        let visible = self.cutoffs[position.0 as usize] as usize;
        self.cards[position.0 as usize]
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

    pub fn take_cards_at_index(&mut self, position: (i8, i8), index: usize) -> Vec<Card> {
        let column = position.0 as usize;
        if column >= self.cards.len() || index > self.cards[column].len() {
            return vec![];
        }

        if self.cutoffs[column] > 0 && (index as u8) <= self.cutoffs[column] {
            self.cutoffs[column] -= 1;
        }

        self.cards[position.0 as usize].split_off(index)
    }

    pub fn add_card(&mut self, to: (i8, i8), card: Card) -> bool {
        let parent_card = match self.get_top_card(to) {
            Some(parent) => parent,
            _ => {
                if card.rank == 13 {
                    self.cards[to.0 as usize].push(card);
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

        self.cards[to.0 as usize].push(card);
        return true;
    }

    pub fn get_visible_cards(&self, column: i8) -> Vec<Card> {
        self.cards[column as usize][(self.cutoffs[column as usize] as usize)..].to_vec()
    }

    pub fn update_cutoffs(&mut self, position: (i8, i8)) {
        let index = position.0 as usize;
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
        from: (i8, i8),
        to: (i8, i8),
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

    pub fn try_to_move_between_tableu(&mut self, from: (i8, i8), to: (i8, i8)) {
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
