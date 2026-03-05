mod sim;
mod gen;
mod ui;
mod export;

use std::io;
use std::time::{Duration, Instant};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};
use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal;

use crate::gen::world_gen;
use crate::sim::{SimSpeed, SimState};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Generate a world with a default seed (will be player-configurable later)
    let seed = 42;
    let (world, agents) = world_gen::generate_world(seed);
    let sim = SimState::new(world, agents);

    // Run the app loop; catch errors so we always clean up the terminal
    let result = run_app(&mut terminal, sim);

    // Restore terminal no matter what
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    // Now propagate any error from the app loop
    result
}

/// Target frame duration (~30 FPS) — controls how often we redraw and process ticks.
const FRAME_DURATION: Duration = Duration::from_millis(33);

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    mut sim: SimState,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let frame_start = Instant::now();

        // Draw the current state
        terminal.draw(|frame| {
            ui::layout::draw_main_layout(frame, &sim);
        })?;

        // Run simulation ticks for this frame
        sim.step_frame();

        // Process input — poll with remaining frame time budget
        let elapsed = frame_start.elapsed();
        let poll_time = FRAME_DURATION.saturating_sub(elapsed);

        if event::poll(poll_time)? {
            if let Event::Key(key) = event::read()? {
                // crossterm on Windows fires both Press and Release; only act on Press
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char(' ') => {
                            // Toggle pause/run
                            sim.speed = if sim.speed == SimSpeed::Paused {
                                SimSpeed::Run1x
                            } else {
                                SimSpeed::Paused
                            };
                        }
                        KeyCode::Char('.') => {
                            // Step one tick (only when paused)
                            if sim.speed == SimSpeed::Paused {
                                sim.tick();
                            }
                        }
                        KeyCode::Char('1') => {
                            sim.speed = SimSpeed::Run1x;
                        }
                        KeyCode::Char('5') => {
                            sim.speed = SimSpeed::Run5x;
                        }
                        KeyCode::Char('2') => {
                            sim.speed = SimSpeed::Run20x;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
