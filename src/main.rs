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
use crate::sim::{Overlay, SimSpeed, SimState};

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

/// Target frame duration (~30 FPS).
const FRAME_DURATION: Duration = Duration::from_millis(33);

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    mut sim: SimState,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut frame_count: u64 = 0;

    loop {
        let frame_start = Instant::now();
        frame_count += 1;

        // Draw the current state
        terminal.draw(|frame| {
            ui::layout::draw_main_layout(frame, &sim);
        })?;

        // Run simulation ticks (only when no overlay is active)
        if sim.overlay == Overlay::None {
            sim.step_frame(frame_count);
        }

        // Process input — poll with remaining frame time budget
        let elapsed = frame_start.elapsed();
        let poll_time = FRAME_DURATION.saturating_sub(elapsed);

        if event::poll(poll_time)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if handle_input(&mut sim, key.code) {
                        return Ok(());
                    }
                }
            }
        }
    }
}

/// Route input based on the current overlay state.
/// Returns true if the app should quit.
fn handle_input(sim: &mut SimState, key: KeyCode) -> bool {
    match &sim.overlay {
        Overlay::None => handle_main_input(sim, key),
        Overlay::InspectAgent(_) => { handle_inspect_input(sim, key); false }
        Overlay::AgentSearch(_) => { handle_search_input(sim, key); false }
        Overlay::ExportMenu => { handle_export_menu_input(sim, key); false }
        Overlay::ExportInput(_) => { handle_export_input(sim, key); false }
    }
}

/// Input handling for the main simulation view. Returns true if should quit.
fn handle_main_input(sim: &mut SimState, key: KeyCode) -> bool {
    match key {
        KeyCode::Char('q') => {
            return true;
        }
        KeyCode::Char(' ') => {
            sim.speed = if sim.speed == SimSpeed::Paused {
                SimSpeed::Run1x
            } else {
                SimSpeed::Paused
            };
        }
        KeyCode::Char('.') => {
            if sim.speed == SimSpeed::Paused {
                sim.tick();
            }
        }
        KeyCode::Char('1') => sim.speed = SimSpeed::Run1x,
        KeyCode::Char('5') => sim.speed = SimSpeed::Run5x,
        KeyCode::Char('2') => sim.speed = SimSpeed::Run20x,
        KeyCode::PageUp => sim.scroll_log_up(5),
        KeyCode::PageDown => sim.scroll_log_down(5),
        KeyCode::Char('i') => {
            // Open agent search overlay
            sim.overlay = Overlay::AgentSearch(String::new());
        }
        KeyCode::Char('e') => {
            sim.overlay = Overlay::ExportMenu;
        }
        _ => {}
    }
    false
}

/// Input handling when inspecting an agent.
fn handle_inspect_input(sim: &mut SimState, key: KeyCode) {
    match key {
        KeyCode::Esc => sim.overlay = Overlay::None,
        _ => {}
    }
}

/// Input handling for the agent search overlay.
fn handle_search_input(sim: &mut SimState, key: KeyCode) {
    // We need to extract the current query to work with it
    let query = if let Overlay::AgentSearch(ref q) = sim.overlay {
        q.clone()
    } else {
        return;
    };

    match key {
        KeyCode::Esc => {
            sim.overlay = Overlay::None;
        }
        KeyCode::Enter => {
            // Find first match and inspect it
            if query.len() >= 2 {
                let matches = sim.search_agents(&query);
                if let Some(&idx) = matches.first() {
                    sim.overlay = Overlay::InspectAgent(idx);
                } else {
                    sim.set_status_message("No matching agents found.".to_string());
                    sim.overlay = Overlay::None;
                }
            }
        }
        KeyCode::Backspace => {
            let mut q = query;
            q.pop();
            sim.overlay = Overlay::AgentSearch(q);
        }
        KeyCode::Char(c) => {
            let mut q = query;
            q.push(c);
            sim.overlay = Overlay::AgentSearch(q);
        }
        _ => {}
    }
}

/// Input handling for the export menu.
fn handle_export_menu_input(sim: &mut SimState, key: KeyCode) {
    match key {
        KeyCode::Esc => sim.overlay = Overlay::None,
        KeyCode::Char('1') => {
            sim.overlay = Overlay::ExportInput(String::new());
        }
        _ => {}
    }
}

/// Input handling for the export filename input.
fn handle_export_input(sim: &mut SimState, key: KeyCode) {
    let input = if let Overlay::ExportInput(ref s) = sim.overlay {
        s.clone()
    } else {
        return;
    };

    match key {
        KeyCode::Esc => {
            sim.overlay = Overlay::None;
        }
        KeyCode::Enter => {
            let prefix = if input.is_empty() { "log" } else { &input };
            match export::export_log(&sim.events, prefix) {
                Ok(path) => {
                    sim.set_status_message(format!("Exported to {}", path));
                }
                Err(e) => {
                    sim.set_status_message(format!("Export failed: {}", e));
                }
            }
            sim.overlay = Overlay::None;
        }
        KeyCode::Backspace => {
            let mut s = input;
            s.pop();
            sim.overlay = Overlay::ExportInput(s);
        }
        KeyCode::Char(c) => {
            let mut s = input;
            s.push(c);
            sim.overlay = Overlay::ExportInput(s);
        }
        _ => {}
    }
}
