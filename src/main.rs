use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Widget, canvas::Canvas},
    *,
};
use std::cmp;
use std::time::{Duration, Instant};

mod utils;
use utils::{canvas_style, card_text_style, get_card};

mod card;
use card::Card;

mod stock;
use stock::Stock;

mod tableau;
use tableau::Tableau;

mod waste;
use waste::Waste;

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
    foundations: Vec<u8>,
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
            foundations: vec![0, 0, 0, 0],
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

    fn draw_new(&mut self) {
        if self.stock.cards.len() == 0 {
            self.stock.reset(&self.waste);
            self.waste.reset();
        }

        let new_card = self.stock.deal();
        self.waste.add(new_card);
    }

    fn take_from_waste(&mut self) {
        let card_to_place = match self.waste.get_top_card() {
            Some(card) => card,
            _ => return,
        };

        let card_to_add_to = match self.tableau.get_top_card(self.selected) {
            Some(card) => card,
            _ => {
                if card_to_place.rank == 13 {
                    self.tableau.add_card(self.selected, card_to_place);
                    self.waste.cards.pop();
                    self.active = None;
                    return;
                } else {
                    return;
                }
            }
        };

        if card_to_add_to.suit % 2 == card_to_place.suit % 2 {
            self.active = Some(self.selected);
            return;
        }

        if card_to_add_to.rank - 1 != card_to_place.rank {
            self.active = Some(self.selected);
            return;
        }

        self.tableau.add_card(self.selected, card_to_place);

        self.waste.cards.pop();

        self.active = None;
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

        let selected_foundation = self.selected.0 - 2;

        if selected_foundation != card.suit as i8 {
            self.active = Some(self.selected);
            return;
        };

        let current_value = self.foundations[(card.suit - 1) as usize];

        if card.rank != current_value + 1 {
            self.active = Some(self.selected);
            return;
        };

        self.foundations[(card.suit - 1) as usize] = card.rank;

        if active_position.1 == 0 && active_position.0 == 1 {
            self.waste.remove();
        } else {
            self.tableau.update_cutoffs(active_position);
            self.tableau.cards[active_position.0 as usize].pop();
        }

        self.active = None;
    }

    fn move_between_tableau(&mut self) {
        let active_position;
        let active_card = match self.active {
            Some(active) => {
                active_position = active;
                match self.tableau.get_top_card(active) {
                    Some(tableau_card) => tableau_card,
                    _ => return,
                }
            }
            _ => return,
        };

        if self.tableau.cards[self.selected.0 as usize].len() == 0 {
            if active_card.rank == 13 {
                self.tableau.add_card(self.selected, active_card);
                self.tableau.update_cutoffs(active_position);
                self.tableau.cards[active_position.0 as usize].pop();
                self.active = None;
            } else {
                return;
            }
        }

        let selected_card: Card = match self.tableau.get_top_card(self.selected) {
            Some(tableau_card) => tableau_card,
            _ => return,
        };

        if active_card.suit % 2 == selected_card.suit % 2 {
            self.active = Some(self.selected);
            return;
        }

        if active_card.rank != selected_card.rank - 1 {
            self.active = Some(self.selected);
            return;
        }

        self.tableau.add_card(self.selected, active_card);
        self.tableau.update_cutoffs(active_position);
        self.tableau.cards[active_position.0 as usize].pop();

        self.active = None;
    }

    fn on_tick(&mut self) {
        self.tick_count += 1;
    }

    fn draw(&self, frame: &mut Frame) {
        let horizontal_constraints: [Constraint; 7] = [Constraint::Percentage(14); 7];
        let horizontal = Layout::horizontal(horizontal_constraints);

        let vertical_constraints: [Constraint; 2] = [Constraint::Percentage(50); 2];
        let vertical = Layout::vertical(vertical_constraints);

        let [top, bottom] = vertical.areas(frame.area());
        let [stock, waste, second_empty, spades, hearts, clubs, diamonds] = horizontal.areas(top);

        let [first, second, third, fourth, fifth, sixth, seventh] = horizontal.areas(bottom);

        frame.render_widget(self.stock_canvas((0, 0)), stock);
        frame.render_widget(self.waste_canvas((1, 0)), waste);
        frame.render_widget(self.empty_canvas(), second_empty);
        frame.render_widget(self.foundation_canvas((3, 0)), spades);
        frame.render_widget(self.foundation_canvas((4, 0)), hearts);
        frame.render_widget(self.foundation_canvas((5, 0)), clubs);
        frame.render_widget(self.foundation_canvas((6, 0)), diamonds);

        frame.render_widget(self.card_canvas((0, 1)), first);
        frame.render_widget(self.card_canvas((1, 1)), second);
        frame.render_widget(self.card_canvas((2, 1)), third);
        frame.render_widget(self.card_canvas((3, 1)), fourth);
        frame.render_widget(self.card_canvas((4, 1)), fifth);
        frame.render_widget(self.card_canvas((5, 1)), sixth);
        frame.render_widget(self.card_canvas((6, 1)), seventh);
    }

    fn card_canvas(&self, pos: (i8, i8)) -> impl Widget + '_ {
        let visible_cards: Vec<Card> = self.tableau.get_visible_cards(pos.0);

        let card_text = format!("Hidden cards: {}", self.tableau.cutoffs[pos.0 as usize]);

        Canvas::default()
            .block(
                Block::bordered()
                    .title(card_text)
                    .border_style(canvas_style(pos, self.selected, self.active)),
            )
            .x_bounds([0.0, 100.0])
            .y_bounds([0.0, 100.0])
            .paint(move |ctx| {
                ctx.layer();
                for (i, card) in visible_cards.iter().enumerate() {
                    let card_name = get_card(card.suit, card.rank);

                    ctx.print(
                        10.0,
                        i as f64 * 10.0,
                        Span::styled(format!("{}", card_name), card_text_style(Some(*card))),
                    );
                }
            })
    }

    fn stock_canvas(&self, pos: (i8, i8)) -> impl Widget + '_ {
        let card_amount = self.stock.cards.len() + 1;

        let card_text = format!("Cards in stock: {}", card_amount);

        Canvas::default()
            .block(
                Block::bordered()
                    .title(card_text)
                    .border_style(canvas_style(pos, self.selected, self.active)),
            )
            .x_bounds([0.0, 100.0])
            .y_bounds([0.0, 100.0])
            .paint(|_ctx| {})
    }

    fn waste_canvas(&self, pos: (i8, i8)) -> impl Widget + '_ {
        let cards = self.waste.get_last_cards();

        Canvas::default()
            .block(
                Block::bordered()
                    .title("Waste pile")
                    .border_style(canvas_style(pos, self.selected, self.active)),
            )
            .x_bounds([0.0, 100.0])
            .y_bounds([0.0, 100.0])
            .paint(move |ctx| {
                ctx.layer();
                if cards.len() == 0 {
                    ctx.print(10.0, 50.0, Span::styled("Empty", card_text_style(None)));
                } else {
                    for (i, card) in cards.iter().enumerate() {
                        let card_name = get_card(card.suit, card.rank);

                        ctx.print(
                            10.0,
                            i as f64 * 10.0,
                            Span::styled(format!("{}", card_name), card_text_style(Some(*card))),
                        );
                    }
                }
            })
    }

    fn empty_canvas(&self) -> impl Widget + '_ {
        Canvas::default().paint(|_ctx| {})
    }

    fn foundation_canvas(&self, pos: (i8, i8)) -> impl Widget + '_ {
        let suit_index = pos.0 - 3;

        let card_name = get_card(
            (suit_index + 1) as u8,
            self.foundations[suit_index as usize],
        );

        Canvas::default()
            .block(
                Block::bordered()
                    .title("Foundation")
                    .border_style(canvas_style(pos, self.selected, self.active)),
            )
            .x_bounds([0.0, 100.0])
            .y_bounds([0.0, 100.0])
            .paint(move |ctx| {
                ctx.layer();
                ctx.print(
                    10.0,
                    50.0,
                    Span::styled(
                        format!("{}", card_name),
                        Style::default().fg(if pos.0 % 2 == 0 {
                            Color::Red
                        } else {
                            Color::Blue
                        }),
                    ),
                )
            })
    }

    fn handle_key_press(&mut self, key: event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Left => {
                self.selected.0 = cmp::min(cmp::max(0, self.selected.0 - 1), 6);
                if self.selected == (2, 0) {
                    self.selected = (1, 0);
                }
            }
            KeyCode::Right => {
                self.selected.0 = cmp::min(cmp::max(0, self.selected.0 + 1), 6);
                if self.selected == (2, 0) {
                    self.selected = (3, 0);
                }
            }
            KeyCode::Up => {
                self.selected.1 = cmp::min(cmp::max(0, self.selected.1 - 1), 1);
                if self.selected == (2, 0) {
                    self.selected = (3, 0);
                }
            }
            KeyCode::Down => self.selected.1 = cmp::min(cmp::max(0, self.selected.1 + 1), 1),
            KeyCode::Enter => match self.active {
                Some(active_card) => {
                    if active_card == self.selected {
                        self.active = None;
                    } else {
                        if self.selected == (0, 0) {
                            self.draw_new();
                        } else if active_card.1 == 0 && active_card.0 == 1 && self.selected.1 == 1 {
                            self.take_from_waste();
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
            _ => {}
        }
    }
}
