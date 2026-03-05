mod sim;
mod gen;
mod ui;
mod export;

use std::io;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};
use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal;

use crate::gen::world_gen;
use crate::sim::world::World;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Generate a world with a default seed (will be player-configurable later)
    let seed = 42;
    let world = world_gen::generate_world(seed);

    // Run the app loop; catch errors so we always clean up the terminal
    let result = run_app(&mut terminal, world);

    // Restore terminal no matter what
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    // Now propagate any error from the app loop
    result
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    world: World,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        terminal.draw(|frame| {
            ui::layout::draw_main_layout(frame, &world);
        })?;

        // Poll for input — blocks until an event arrives
        if let Event::Key(key) = event::read()? {
            // crossterm on Windows fires both Press and Release; only act on Press
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    _ => {}
                }
            }
        }
    }
}
