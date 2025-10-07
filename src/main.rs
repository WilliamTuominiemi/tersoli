use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, Widget, canvas::Canvas},
    *,
};
use std::cmp;
use std::time::{Duration, Instant};

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
}

impl App {
    const fn new() -> Self {
        Self {
            exit: false,
            tick_count: 0,
            selected: (0, 0),
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
        let tick_rate = Duration::from_millis(16);
        let mut last_tick = Instant::now();

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

    fn on_tick(&mut self) {
        self.tick_count += 1;
    }

    fn draw(&self, frame: &mut Frame) {
        let horizontal_constraints: [Constraint; 7] = [Constraint::Percentage(14); 7];
        let horizontal = Layout::horizontal(horizontal_constraints);

        let vertical_constraints: [Constraint; 2] = [Constraint::Percentage(50); 2];
        let vertical = Layout::vertical(vertical_constraints);

        let [top, bottom] = vertical.areas(frame.area());
        let [
            stock,
            first_empty,
            second_empty,
            spades,
            hearts,
            clubs,
            diamonds,
        ] = horizontal.areas(top);

        let [first, second, third, fourth, fifth, sixth, seventh] = horizontal.areas(bottom);

        frame.render_widget(self.deck_canvas((0, 0)), stock);
        frame.render_widget(self.empty_canvas((1, 0)), first_empty);
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
        let selected = pos == self.selected;

        Canvas::default()
            .block(
                Block::bordered()
                    .title("Card")
                    .border_style(Style::default().fg(if selected {
                        Color::Red
                    } else {
                        Color::White
                    })),
            )
            .paint(|_ctx| {})
    }

    fn deck_canvas(&self, pos: (i8, i8)) -> impl Widget + '_ {
        let selected = pos == self.selected;

        Canvas::default()
            .block(Block::bordered().title("Stock").border_style(
                Style::default().fg(if selected { Color::Red } else { Color::White }),
            ))
            .paint(|_ctx| {})
    }

    fn empty_canvas(&self, pos: (i8, i8)) -> impl Widget + '_ {
        Canvas::default().paint(|_ctx| {})
    }

    fn foundation_canvas(&self, pos: (i8, i8)) -> impl Widget + '_ {
        let selected = pos == self.selected;

        Canvas::default()
            .block(Block::bordered().title("Foundation").border_style(
                Style::default().fg(if selected { Color::Red } else { Color::White }),
            ))
            .paint(|_ctx| {})
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
                    self.selected = (0, 0);
                }
            }
            KeyCode::Right => {
                self.selected.0 = cmp::min(cmp::max(0, self.selected.0 + 1), 6);
                if self.selected == (1, 0) {
                    self.selected = (3, 0);
                }
            }
            KeyCode::Up => {
                self.selected.1 = cmp::min(cmp::max(0, self.selected.1 - 1), 1);
                if self.selected == (1, 0) {
                    self.selected = (0, 0);
                } else if self.selected == (2, 0) {
                    self.selected = (3, 0);
                }
            }
            KeyCode::Down => self.selected.1 = cmp::min(cmp::max(0, self.selected.1 + 1), 1),
            _ => {}
        }
    }
}
