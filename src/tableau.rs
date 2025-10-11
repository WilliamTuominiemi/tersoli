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

    pub fn add_card(&mut self, position: (i8, i8), card: Card) {
        self.cards[position.0 as usize].push(card);
    }

    pub fn get_visible_cards(&self, column: i8) -> Vec<Card> {
        self.cards[column as usize][(self.cutoffs[column as usize] as usize)..].to_vec()
    }

    pub fn update_cutoffs(&mut self, position: (i8, i8)) {
        let cutoff = self.cutoffs[position.0 as usize];
        let card_index = self.cards[position.0 as usize].len() as u8;

        if cutoff > 0 && card_index > 0 && cutoff < card_index {
            self.cutoffs[position.0 as usize] -= 1;
        }
    }
}
