use std::{
    io,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{backend::CrosstermBackend, Terminal};

use crate::{state::State, ui};

pub const QUIT_CHAR: char = 'q';

pub struct RunnerOptions {
    pub fps: u8,
}

impl RunnerOptions {
    pub fn default() -> Self {
        Self { fps: 15 }
    }
}

pub fn start(initial_state: &State, options: &RunnerOptions) -> io::Result<()> {
    let mut state = initial_state.clone();

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let tick_rate = Duration::from_secs_f64(1.0 / options.fps as f64);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|frame| ui::draw(frame, &state))?;

        let poll_interval = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or(Duration::from_secs(0));

        if event::poll(poll_interval)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char(QUIT_CHAR) = key.code {
                    break;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            state = state.next();
            last_tick = Instant::now();
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
