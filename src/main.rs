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
use utils::get_card;

mod card;
use card::Card;

mod stock;
use stock::Stock;

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
    drawn: Stock,
    stock_face: Option<Card>,
    tableau: Vec<Vec<Option<Card>>>,
    tableau_cutoffs: Vec<u8>,
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
            drawn: Stock::new(),
            stock_face: None,
            tableau: vec![],
            tableau_cutoffs: vec![0, 1, 2, 3, 4, 5, 6],
            foundations: vec![0, 0, 0, 0],
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
        let tick_rate = Duration::from_millis(16);
        let mut last_tick = Instant::now();

        self.first_deal();

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

    fn first_deal(&mut self) {
        let dealt_card = self.stock.deal();
        self.stock_face = Some(dealt_card);
        self.drawn.cards = vec![];
        self.drawn.cards.push(dealt_card);

        self.tableau.clear();
        for i in 0..7 {
            let mut row = Vec::new();
            for _j in 0..=i {
                row.push(Some(self.stock.deal()));
            }
            self.tableau.push(row);
        }
    }

    fn draw_new(&mut self) {
        if self.stock.cards.len() == 0 {
            self.stock.cards = self.drawn.cards.clone();
            self.drawn.cards = vec![];
        }

        let new_card = self.stock.deal();
        self.stock_face = Some(new_card);
        self.drawn.cards.push(new_card);
    }

    fn take_from_drawn(&mut self) {
        let card_to_place = match self.stock_face {
            Some(card) => card,
            _ => return,
        };

        let selected_stack = self.selected.0;
        let card_to_add_to = match self.tableau[selected_stack as usize]
            [self.tableau[selected_stack as usize].len() - 1]
        {
            Some(card) => card,
            _ => return,
        };

        if card_to_add_to.suit % 2 == card_to_place.suit % 2 {
            self.active = Some(self.selected);
            return;
        }

        if card_to_add_to.rank - 1 != card_to_place.rank {
            self.active = Some(self.selected);
            return;
        }

        self.tableau[selected_stack as usize].push(Some(card_to_place));
        self.drawn.cards.pop();

        self.stock_face = Some(self.stock.deal());

        self.active = Some(self.selected);
    }

    fn place_in_foundation(&mut self) {
        let mut active_position: (i8, i8);
        let card: Card = match self.active {
            Some(active) => {
                active_position = active;
                if active.1 == 1 {
                    if let Some(card) =
                        self.tableau[active.0 as usize][self.tableau[active.0 as usize].len() - 1]
                    {
                        card
                    } else {
                        return;
                    }
                } else if active.1 == 0 && active.0 == 1 {
                    if let Some(card) = self.stock_face {
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
            self.stock_face = Some(self.stock.deal());
        } else {
            if self.tableau_cutoffs[active_position.0 as usize] > 0 {
                self.tableau_cutoffs[active_position.0 as usize] -= 1;
            }
            self.tableau[active_position.0 as usize].pop();
        }

        self.active = Some(self.selected);
    }

    fn move_between_tableau(&mut self) {
        // From
        let active_tableau;
        let active_card = match self.active {
            Some(active) => {
                active_tableau = active.0;
                match self.tableau[active.0 as usize][self.tableau[active.0 as usize].len() - 1] {
                    Some(card) => card,
                    _ => return,
                }
            }
            _ => return,
        };

        // To

        if self.tableau[self.selected.0 as usize].len() == 0 {
            if active_card.rank != 13 {
                return;
            } else {
                self.tableau[self.selected.0 as usize].push(Some(active_card));
                if self.tableau_cutoffs[active_tableau as usize] > 0
                    && self.tableau_cutoffs[active_tableau as usize] as usize
                        == self.tableau[active_tableau as usize].len() - 1
                {
                    self.tableau_cutoffs[active_tableau as usize] -= 1;
                }
                self.tableau[active_tableau as usize].pop();
                self.active = Some(self.selected);
                return;
            }
        }

        let selected_card: Card = match self.tableau[self.selected.0 as usize]
            [self.tableau[self.selected.0 as usize].len() - 1]
        {
            Some(card) => card,
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

        self.tableau[self.selected.0 as usize].push(Some(active_card));
        if self.tableau_cutoffs[active_tableau as usize] > 0
            && self.tableau_cutoffs[active_tableau as usize] as usize
                == self.tableau[active_tableau as usize].len() - 1
        {
            self.tableau_cutoffs[active_tableau as usize] -= 1;
        }
        self.tableau[active_tableau as usize].pop();

        self.active = Some(self.selected);
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
        let [stock, drawn, second_empty, spades, hearts, clubs, diamonds] = horizontal.areas(top);

        let [first, second, third, fourth, fifth, sixth, seventh] = horizontal.areas(bottom);

        frame.render_widget(self.stock_canvas((0, 0)), stock);
        frame.render_widget(self.drawn_canvas((1, 0)), drawn);
        frame.render_widget(self.empty_canvas((2, 0)), second_empty);
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
        let visible_cards: Vec<Option<Card>> = self.tableau[pos.0 as usize]
            [(self.tableau_cutoffs[pos.0 as usize] as usize)..]
            .to_vec();

        let card_text = format!("Hidden cards: {}", self.tableau_cutoffs[pos.0 as usize]);

        Canvas::default()
            .block(
                Block::bordered()
                    .title(card_text)
                    .border_style(self.canvas_style(pos)),
            )
            .x_bounds([0.0, 100.0])
            .y_bounds([0.0, 100.0])
            .paint(move |ctx| {
                ctx.layer();
                for (i, card) in visible_cards.iter().enumerate() {
                    let card_name: String = match card {
                        Some(card) => get_card(card.suit, card.rank),
                        None => "Stock empty".to_string(),
                    };

                    ctx.print(
                        10.0,
                        i as f64 * 10.0,
                        Span::styled(format!("{}", card_name), self.card_text_style(*card)),
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
                    .border_style(self.canvas_style(pos)),
            )
            .x_bounds([0.0, 100.0])
            .y_bounds([0.0, 100.0])
            .paint(|_ctx| {})
    }

    fn drawn_canvas(&self, pos: (i8, i8)) -> impl Widget + '_ {
        let card_name: String = match self.stock_face {
            Some(card) => get_card(card.suit, card.rank),
            None => "Stock empty".to_string(),
        };

        Canvas::default()
            .block(
                Block::bordered()
                    .title("Drawn cards")
                    .border_style(self.canvas_style(pos)),
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
                        self.card_text_style(self.stock_face),
                    ),
                );
            })
    }

    fn empty_canvas(&self, pos: (i8, i8)) -> impl Widget + '_ {
        Canvas::default().paint(|_ctx| {})
    }

    fn foundation_canvas(&self, pos: (i8, i8)) -> impl Widget + '_ {
        let suit_index = pos.0 - 3;

        let suit = match suit_index {
            0 => "♠".to_string(),
            1 => "♥".to_string(),
            2 => "♣".to_string(),
            3 => "♦".to_string(),
            _ => "Error".to_string(),
        };

        let number = match self.foundations[suit_index as usize] {
            0 => "".to_string(),
            1 => "Ace".to_string(),
            2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 => self.foundations[suit_index as usize].to_string(),
            11 => "Jack".to_string(),
            12 => "Queen".to_string(),
            13 => "King".to_string(),
            _ => "Error".to_string(),
        };

        let card_name = format!("{} {}", suit, number);

        Canvas::default()
            .block(
                Block::bordered()
                    .title("Foundation")
                    .border_style(self.canvas_style(pos)),
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
                            self.take_from_drawn();
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

    fn canvas_style(&self, pos: (i8, i8)) -> Style {
        let selected = pos == self.selected;
        let active = match self.active {
            Some(position) => position == pos,
            _ => false,
        };

        Style::default().fg(if selected && active {
            Color::Green
        } else if active {
            Color::Blue
        } else if selected {
            Color::Red
        } else {
            Color::White
        })
    }

    fn card_text_style(&self, card: Option<Card>) -> Style {
        Style::default().fg(match card {
            Some(c) => {
                if c.suit % 2 == 0 {
                    Color::Red
                } else {
                    Color::Blue
                }
            }
            _ => Color::Yellow,
        })
    }
}
