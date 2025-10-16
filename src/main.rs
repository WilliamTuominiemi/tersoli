use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout},
    *,
};
use std::cmp;
use std::time::{Duration, Instant};

mod renderer;
mod utils;

mod card;
use card::Card;

mod stock;
use stock::Stock;

mod tableau;
use tableau::Tableau;

mod waste;
use waste::Waste;

mod foundation;
use foundation::Foundation;

use crate::renderer::render;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}

struct App {
    exit: bool,
    tick_count: u64,
    selected: (i8, i8),
    active: Option<(i8, i8)>,
    stock: Stock,
    waste: Waste,
    tableau: Tableau,
    foundations: Foundation,
}

impl App {
    fn new() -> Self {
        Self {
            exit: false,
            tick_count: 0,
            selected: (0, 0),
            active: None,
            stock: Stock::new(),
            waste: Waste::new(),
            tableau: Tableau::new(),
            foundations: Foundation::new(),
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
        let tick_rate = Duration::from_millis(16);
        let mut last_tick = Instant::now();

        self.tableau.initialize(&mut self.stock);

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());

            if event::poll(timeout)? {
                match event::read()? {
                    Event::Key(key) => self.handle_key_press(key),
                    _ => (),
                }
            }

            if last_tick.elapsed() >= tick_rate {
                self.on_tick();
                last_tick = Instant::now();
            }
        }
        Ok(())
    }

    fn reset_selection(&mut self) {
        self.active = None
    }

    fn draw_new(&mut self) {
        if self.stock.cards.is_empty() && self.waste.cards.is_empty() {
            return;
        }

        if self.stock.cards.is_empty() {
            self.stock.reset(&self.waste);
            self.waste.reset();
        }

        self.waste.add(self.stock.deal());
    }

    fn take_from_waste(&mut self) {
        let card_to_place = match self.waste.get_top_card() {
            Some(card) => card,
            _ => return,
        };

        if self.tableau.add_card(self.selected, card_to_place) {
            self.waste.cards.pop();
        }

        self.reset_selection();
    }

    fn place_in_foundation(&mut self) {
        let active_position: (i8, i8);
        let card: Card = match self.active {
            Some(active) => {
                active_position = active;
                if active.1 == 1 {
                    match self.tableau.get_top_card(active) {
                        Some(tableau_card) => tableau_card,
                        _ => return,
                    }
                } else if active.1 == 0 && active.0 == 1 {
                    if let Some(card) = self.waste.get_top_card() {
                        card
                    } else {
                        return;
                    }
                } else {
                    return;
                }
            }
            _ => return,
        };

        if self.foundations.add_card(card, (self.selected.0 - 2) as u8) {
            if active_position.1 == 0 && active_position.0 == 1 {
                self.waste.remove();
            } else {
                self.tableau.update_cutoffs(active_position);
                self.tableau.cards[active_position.0 as usize].pop();
            }
        }

        self.reset_selection();
    }

    fn take_from_foundation(&mut self) {
        let active_position: (i8, i8);
        let foundation_card = match self.active {
            Some(position) => {
                active_position = position;
                match self.foundations.get_top_card(position) {
                    Some(card) => card,
                    _ => return,
                }
            }
            _ => return,
        };

        if self.tableau.add_card(self.selected, foundation_card) {
            self.foundations.remove_card(active_position);
        }

        self.reset_selection();
    }

    fn move_between_tableau(&mut self) {
        let active_position = match self.active {
            Some(active) => active,
            _ => return,
        };

        self.tableau
            .try_to_move_between_tableu(active_position, self.selected);

        self.reset_selection();
    }

    fn try_to_place_in_foundation(&mut self) {
        let card_to_place = if self.selected.1 == 1 {
            match self.tableau.get_top_card(self.selected) {
                Some(tableau_card) => tableau_card,
                _ => return,
            }
        } else if self.selected.1 == 0 && self.selected.0 == 1 {
            match self.waste.get_top_card() {
                Some(card) => card,
                _ => return,
            }
        } else {
            return;
        };

        if self.foundations.snap_add(card_to_place) {
            if self.selected.1 == 1 {
                self.tableau.update_cutoffs(self.selected);
                self.tableau.cards[self.selected.0 as usize].pop();
            } else {
                self.waste.cards.pop();
            }
        }

        self.reset_selection();
    }

    fn on_tick(&mut self) {
        self.tick_count += 1;
    }

    fn draw(&self, frame: &mut Frame) {
        let horizontal_constraints: [Constraint; 7] = [Constraint::Percentage(14); 7];
        let horizontal = Layout::horizontal(horizontal_constraints);

        let vertical_constraints: [Constraint; 2] = [Constraint::Percentage(50); 2];
        let vertical = Layout::vertical(vertical_constraints);

        render(
            frame,
            horizontal,
            vertical,
            &self.tableau,
            &self.stock,
            &self.waste,
            &self.foundations,
            self.selected,
            self.active,
        );
    }

    fn handle_key_press(&mut self, key: event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Left | KeyCode::Char('a') => {
                self.selected.0 = cmp::min(cmp::max(0, self.selected.0 - 1), 6);
                if self.selected == (2, 0) {
                    self.selected = (1, 0);
                }
            }
            KeyCode::Right | KeyCode::Char('d') => {
                self.selected.0 = cmp::min(cmp::max(0, self.selected.0 + 1), 6);
                if self.selected == (2, 0) {
                    self.selected = (3, 0);
                }
            }
            KeyCode::Up | KeyCode::Char('w') => {
                self.selected.1 = cmp::min(cmp::max(0, self.selected.1 - 1), 1);
                if self.selected == (2, 0) {
                    self.selected = (3, 0);
                }
            }
            KeyCode::Down | KeyCode::Char('s') => {
                self.selected.1 = cmp::min(cmp::max(0, self.selected.1 + 1), 1)
            }
            KeyCode::Enter => match self.active {
                Some(active_card) => {
                    if active_card == self.selected {
                        self.reset_selection();
                    } else {
                        if self.selected == (0, 0) {
                            self.draw_new();
                        } else if active_card.1 == 0 && active_card.0 == 1 && self.selected.1 == 1 {
                            self.take_from_waste();
                        } else if active_card.1 == 0 && active_card.0 > 2 && self.selected.1 == 1 {
                            self.take_from_foundation();
                        } else if self.selected.1 == 0 && self.selected.0 > 2 {
                            self.place_in_foundation();
                        } else if self.selected.1 == 1 && active_card.1 == 1 {
                            self.move_between_tableau();
                        } else {
                            self.active = Some(self.selected);
                        }
                    }
                }
                _ => {
                    self.active = if self.selected == (0, 0) {
                        self.draw_new();
                        Some((1, 0))
                    } else {
                        Some(self.selected)
                    }
                }
            },
            KeyCode::Char(' ') => self.try_to_place_in_foundation(),
            _ => {}
        }
    }
}
