use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout},
    *,
};
use std::time::{Duration, Instant};

mod renderer;
mod utils;
use utils::*;

mod location;
use location::Location;
mod command;
use command::Command;
mod suit;

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
    selected: Location,
    active: Option<Location>,
    stock: Stock,
    waste: Waste,
    tableau: Tableau,
    foundation: Foundation,
    won: bool,
}

impl App {
    fn new() -> Self {
        Self {
            exit: false,
            tick_count: 0,
            selected: Location::Stock,
            active: None,
            stock: Stock::new(),
            waste: Waste::new(),
            tableau: Tableau::new(),
            foundation: Foundation::new(),
            won: false,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
        let tick_rate = Duration::from_millis(16);
        let mut last_tick = Instant::now();

        self.tableau.initialize(&mut self.stock);

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());

            if event::poll(timeout)?
                && let Event::Key(key) = event::read()?
            {
                self.handle_key_press(key)
            }

            if last_tick.elapsed() >= tick_rate {
                self.on_tick();
                self.won = check_win(&self.foundation);
                last_tick = Instant::now();
            }
        }
        Ok(())
    }

    fn reset_selection(&mut self) {
        self.active = None
    }

    fn deal_from_stock(&mut self) {
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
        let card: Card = match self.active {
            Some(active) => match active {
                Location::Tableau(_) => match self.tableau.get_top_card(active) {
                    Some(tableau_card) => tableau_card,
                    _ => return,
                },
                Location::Waste => match self.waste.get_top_card() {
                    Some(waste_card) => waste_card,
                    _ => return,
                },
                _ => return,
            },
            _ => return,
        };

        if let Location::Foundation(index) = self.selected
            && self
                .foundation
                .add_card(card, get_suit_by_card_suit_index(index))
        {
            match self.active {
                Some(Location::Waste) => self.waste.remove(),
                Some(Location::Tableau(index)) => {
                    self.tableau.update_cutoffs(index);
                    self.tableau.cards[index].pop();
                }
                _ => return,
            }
        }

        self.reset_selection();
    }

    fn take_from_foundation(&mut self) {
        let active_location: Location;
        let foundation_card = match self.active {
            Some(position) => {
                active_location = position;
                match self.foundation.get_top_card(position) {
                    Some(card) => card,
                    _ => return,
                }
            }
            _ => return,
        };

        if self.tableau.add_card(self.selected, foundation_card) {
            self.foundation.remove_card(active_location);
        }

        self.reset_selection();
    }

    fn move_between_tableau(&mut self) {
        let active_location = match self.active {
            Some(active) => active,
            _ => return,
        };

        self.tableau
            .try_to_move_between_tableau(active_location, self.selected);

        self.reset_selection();
    }

    fn try_to_place_in_foundation(&mut self) {
        let card_to_place = match self.selected {
            Location::Tableau(_) => match self.tableau.get_top_card(self.selected) {
                Some(tableau_card) => tableau_card,
                _ => return,
            },
            Location::Waste => match self.waste.get_top_card() {
                Some(card) => card,
                _ => return,
            },
            _ => return,
        };

        if self.foundation.add_card(card_to_place, card_to_place.suit) {
            match self.selected {
                Location::Tableau(index) => {
                    self.tableau.update_cutoffs(index);
                    self.tableau.cards[index].pop();
                }
                Location::Waste => {
                    self.waste.cards.pop();
                }
                _ => unreachable!("Can't add other than waste or tableau card"),
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
            &self.foundation,
            self.selected,
            self.active,
            self.won,
        );
    }

    fn handle_key_press(&mut self, key: event::KeyEvent) {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Char('q') => self.apply_command(Command::Quit),
                KeyCode::Left | KeyCode::Char('a') => {
                    self.apply_command(Command::MoveLeft);
                }
                KeyCode::Right | KeyCode::Char('d') => {
                    self.apply_command(Command::MoveRight);
                }
                KeyCode::Up | KeyCode::Char('w') => {
                    self.apply_command(Command::MoveUp);
                }
                KeyCode::Down | KeyCode::Char('s') => {
                    self.apply_command(Command::MoveDown);
                }
                KeyCode::Enter => self.apply_command(Command::Select),
                KeyCode::Char(' ') => self.apply_command(Command::AutoPlace),
                _ => {}
            }
        }
    }

    fn apply_command(&mut self, cmd: Command) {
        match cmd {
            Command::AutoPlace => self.try_to_place_in_foundation(),
            Command::Quit => self.exit = true,
            Command::Select => match self.active {
                Some(active) => {
                    if active == self.selected {
                        self.reset_selection();
                    } else {
                        match (self.selected, active) {
                            (Location::Stock, _) => {
                                self.deal_from_stock();
                            }
                            (Location::Tableau(_), Location::Waste) => {
                                self.take_from_waste();
                            }
                            (Location::Tableau(_), Location::Foundation(_)) => {
                                self.take_from_foundation();
                            }
                            (Location::Foundation(_), _) => {
                                self.place_in_foundation();
                            }
                            (Location::Tableau(_), Location::Tableau(_)) => {
                                self.move_between_tableau();
                            }
                            _ => {
                                self.active = Some(self.selected);
                            }
                        }
                    }
                }
                _ => {
                    self.active = if self.selected == Location::Stock {
                        self.deal_from_stock();
                        Some(Location::Waste)
                    } else {
                        Some(self.selected)
                    }
                }
            },
            Command::MoveDown => match self.selected {
                Location::Stock => self.selected = Location::Tableau(0),
                Location::Waste => self.selected = Location::Tableau(1),
                Location::Foundation(index) => self.selected = Location::Tableau(3 + index),
                Location::Tableau(_) => (),
            },
            Command::MoveLeft => match self.selected {
                Location::Stock => (),
                Location::Waste => self.selected = Location::Stock,
                Location::Foundation(index) => {
                    if index == 0 {
                        self.selected = Location::Waste
                    } else {
                        self.selected = Location::Foundation(index - 1)
                    }
                }
                Location::Tableau(index) => {
                    if index != 0 {
                        self.selected = Location::Tableau(index - 1)
                    }
                }
            },
            Command::MoveRight => match self.selected {
                Location::Stock => self.selected = Location::Waste,
                Location::Waste => self.selected = Location::Foundation(0),
                Location::Foundation(index) => {
                    if index != 3 {
                        self.selected = Location::Foundation(index + 1)
                    }
                }
                Location::Tableau(index) => {
                    if index != 6 {
                        self.selected = Location::Tableau(index + 1)
                    }
                }
            },
            Command::MoveUp => match self.selected {
                Location::Stock => (),
                Location::Waste => (),
                Location::Foundation(_) => (),
                Location::Tableau(index) => {
                    if index == 0 {
                        self.selected = Location::Stock
                    } else if index == 1 || index == 2 {
                        self.selected = Location::Waste
                    } else {
                        self.selected = Location::Foundation(index - 3)
                    }
                }
            },
        }
    }
}
