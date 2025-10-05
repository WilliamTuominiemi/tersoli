use ratatui::{
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
        frame.render_widget(self.card_canvas(), frame.area());
    }

    fn card_canvas(&self) -> impl Widget + '_ {
        Canvas::default()
            .block(Block::bordered().title("Card"))
            .paint(|_ctx| {})
    }
}
