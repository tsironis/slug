use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;
use std::io::{self, stdout};
use storage::Storage;

mod app;
mod storage;
mod ui;
use app::{App, Mode};

fn main() -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    // Create app state
    let mut app = App::new();
    let storage = Storage::new();

    // Main loop
    loop {
        terminal.draw(|f| ui::draw(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match app.mode {
                Mode::Normal => match key.code {
                    KeyCode::Char('m') => {
                        app.mode = Mode::Command;
                        app.input_buffer.clear();
                    }
                    KeyCode::Char('q') => break,
                    KeyCode::Char('f') => app.mode = Mode::Future,
                    KeyCode::Char('i') => app.mode = Mode::Insert,
                    KeyCode::Char('h') => app.prev_week(),
                    KeyCode::Char('j') => app.next_day(),
                    KeyCode::Char('k') => app.prev_day(),
                    KeyCode::Char('l') => app.next_week(),
                    _ => {}
                },
                Mode::Future => match key.code {
                    KeyCode::Esc => app.mode = Mode::Normal,
                    KeyCode::Char('q') => break,
                    _ => app.handle_input(key),
                },
                Mode::Command => match key.code {
                    KeyCode::Esc => app.mode = Mode::Normal,
                    _ => app.handle_input(key),
                },
                Mode::Insert => match key.code {
                    KeyCode::Esc => app.mode = Mode::Normal,
                    _ => app.handle_input(key),
                },
                _ => {}
            }
        }
        if app.should_quit {
            break;
        }
    }

    // Cleanup
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
