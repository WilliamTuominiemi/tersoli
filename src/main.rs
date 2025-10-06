use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout},
    widgets::{Block, Widget, canvas::Canvas},
    *,
};
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
}

impl App {
    const fn new() -> Self {
        Self {
            exit: false,
            tick_count: 0,
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

        frame.render_widget(self.deck_canvas(), stock);
        frame.render_widget(self.empty_canvas(), first_empty);
        frame.render_widget(self.empty_canvas(), second_empty);
        frame.render_widget(self.foundation_canvas(), spades);
        frame.render_widget(self.foundation_canvas(), hearts);
        frame.render_widget(self.foundation_canvas(), clubs);
        frame.render_widget(self.foundation_canvas(), diamonds);

        frame.render_widget(self.card_canvas(), first);
        frame.render_widget(self.card_canvas(), second);
        frame.render_widget(self.card_canvas(), third);
        frame.render_widget(self.card_canvas(), fourth);
        frame.render_widget(self.card_canvas(), fifth);
        frame.render_widget(self.card_canvas(), sixth);
        frame.render_widget(self.card_canvas(), seventh);
    }

    fn card_canvas(&self) -> impl Widget + '_ {
        Canvas::default()
            .block(Block::bordered().title("Card"))
            .paint(|_ctx| {})
    }

    fn deck_canvas(&self) -> impl Widget + '_ {
        Canvas::default()
            .block(Block::bordered().title("Stock"))
            .paint(|_ctx| {})
    }

    fn empty_canvas(&self) -> impl Widget + '_ {
        Canvas::default().paint(|_ctx| {})
    }

    fn foundation_canvas(&self) -> impl Widget + '_ {
        Canvas::default()
            .block(Block::bordered().title("Foundation"))
            .paint(|_ctx| {})
    }

    fn handle_key_press(&mut self, key: event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('q') => self.exit = true,
            _ => {}
        }
    }
}
