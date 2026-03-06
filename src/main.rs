mod sim;
mod gen;
mod ui;
mod export;

use std::io;
use std::time::{Duration, Instant};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};
use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal;

use crate::export::SaveFileInfo;
use crate::gen::world_gen;
use crate::sim::{Overlay, SimSpeed, SimState};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Ensure saves directory exists at startup
    let _ = export::ensure_saves_dir();

    // Set up terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run the app loop; catch errors so we always clean up the terminal
    let result = run_app(&mut terminal);

    // Restore terminal no matter what
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

/// Target frame duration (~30 FPS).
const FRAME_DURATION: Duration = Duration::from_millis(33);

/// The top-level application mode (menu vs. in-game).
enum AppMode {
    MainMenu {
        selected: usize,
        has_autosave: bool,
    },
    NewWorld {
        selected_preset: usize,
        seed_input: String,
        editing_seed: bool,
    },
    LoadWorld {
        saves: Vec<SaveFileInfo>,
        selected: usize,
        confirm_delete: bool,
    },
    /// Shown when all 10 save slots are full and player tries to create a new world.
    SavesFull,
    /// Brief "Generating..." screen shown for a few frames before sim starts.
    Generating {
        seed: u64,
        flavor: usize,
        frames_shown: u32,
    },
    /// World Assessment Report shown after generation, before sim starts. (scroll offset, flavor index)
    WorldReport {
        scroll: usize,
        flavor: usize,
    },
    InGame,
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut frame_count: u64 = 0;
    let mut mode = AppMode::MainMenu {
        selected: 0,
        has_autosave: export::most_recent_save().is_some(),
    };
    // sim lives here, populated when entering InGame mode
    let mut sim: Option<SimState> = None;

    loop {
        let frame_start = Instant::now();
        frame_count += 1;

        // Draw based on current mode
        terminal.draw(|frame| {
            match &mode {
                AppMode::MainMenu { selected, has_autosave } => {
                    ui::menu::draw_main_menu(frame, *selected, *has_autosave);
                }
                AppMode::NewWorld { selected_preset, seed_input, editing_seed } => {
                    ui::menu::draw_new_world(frame, *selected_preset, seed_input, *editing_seed);
                }
                AppMode::LoadWorld { saves, selected, confirm_delete } => {
                    ui::menu::draw_load_world(frame, saves, *selected, *confirm_delete);
                }
                AppMode::SavesFull => {
                    ui::menu::draw_saves_full(frame);
                }
                AppMode::Generating { .. } => {
                    ui::menu::draw_generating(frame);
                }
                AppMode::WorldReport { scroll, .. } => {
                    if let Some(ref s) = sim {
                        ui::overlays::draw_world_report_fullscreen(frame, s, *scroll, true);
                    }
                }
                AppMode::InGame => {
                    if let Some(ref s) = sim {
                        ui::layout::draw_main_layout(frame, s);
                    }
                }
            }
        })?;

        // Handle Generating state transition (show the screen for a few frames first)
        if let AppMode::Generating { seed, flavor, ref mut frames_shown } = mode {
            *frames_shown += 1;
            if *frames_shown >= 3 {
                let wf = world_gen::WorldFlavor::from_index(flavor);
                let (world, agents, institutions, sites, artifacts) = world_gen::generate_world(seed, wf);
                sim = Some(SimState::new(world, agents, institutions, sites, artifacts));
                mode = AppMode::WorldReport { scroll: 0, flavor };
                continue;
            }
        }

        // Run simulation ticks when in-game and no overlay is active
        if let AppMode::InGame = &mode {
            if let Some(ref mut s) = sim {
                if s.overlay == Overlay::None || matches!(s.overlay, Overlay::SiteView(_, _)) {
                    s.step_frame(frame_count);
                }

                // Autosave every 500 ticks
                if s.world.tick > 0 && s.world.tick - s.last_autosave_tick >= 500 {
                    s.last_autosave_tick = s.world.tick;
                    match export::save_world(s, "autosave") {
                        Ok(_) => { s.status_message = Some(("~ autosaved".to_string(), 40)); }
                        Err(e) => s.set_status_message(format!("Autosave failed: {}", e)),
                    }
                }
            }
        }

        // Process input
        let elapsed = frame_start.elapsed();
        let poll_time = FRAME_DURATION.saturating_sub(elapsed);

        if event::poll(poll_time)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match handle_input(&mut mode, &mut sim, key.code, key.modifiers) {
                        InputResult::Continue => {}
                        InputResult::Quit => return Ok(()),
                        InputResult::ReturnToMenu => {
                            sim = None;
                            mode = AppMode::MainMenu {
                                selected: 0,
                                has_autosave: export::most_recent_save().is_some(),
                            };
                        }
                    }
                }
            }
        }
    }
}

enum InputResult {
    Continue,
    Quit,
    ReturnToMenu,
}

/// Route input based on the current app mode.
fn handle_input(
    mode: &mut AppMode,
    sim: &mut Option<SimState>,
    key: KeyCode,
    modifiers: KeyModifiers,
) -> InputResult {
    match mode {
        AppMode::MainMenu { .. } => {
            handle_menu_input(mode, sim, key)
        }
        AppMode::NewWorld { .. } => {
            handle_new_world_input(mode, key);
            InputResult::Continue
        }
        AppMode::LoadWorld { .. } => {
            handle_load_world_input(mode, sim, key);
            InputResult::Continue
        }
        AppMode::SavesFull => {
            if matches!(key, KeyCode::Esc) {
                *mode = AppMode::MainMenu {
                    selected: 0,
                    has_autosave: export::most_recent_save().is_some(),
                };
            }
            InputResult::Continue
        }
        AppMode::WorldReport { .. } => {
            handle_world_report_input(mode, sim, key)
        }
        AppMode::Generating { .. } => InputResult::Continue,
        AppMode::InGame => {
            if let Some(ref mut s) = sim {
                handle_game_input(s, key, modifiers)
            } else {
                InputResult::Continue
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Main Menu Input
// ---------------------------------------------------------------------------

fn handle_menu_input(
    mode: &mut AppMode,
    sim: &mut Option<SimState>,
    key: KeyCode,
) -> InputResult {
    let (selected, has_autosave) = if let AppMode::MainMenu { selected, has_autosave } = mode {
        (selected, *has_autosave)
    } else {
        return InputResult::Continue;
    };

    match key {
        KeyCode::Up => {
            if *selected > 0 {
                *selected -= 1;
            }
        }
        KeyCode::Down => {
            if *selected < 3 {
                *selected += 1;
            }
        }
        KeyCode::Enter => {
            match *selected {
                0 => {
                    // New World — check if save slots are full
                    if export::named_save_count() >= export::MAX_SAVE_SLOTS {
                        *mode = AppMode::SavesFull;
                    } else {
                        *mode = AppMode::NewWorld {
                            selected_preset: 4, // Default to Unguided
                            seed_input: String::new(),
                            editing_seed: false,
                        };
                    }
                }
                1 => {
                    // Continue — load the most recently modified save (any type)
                    if has_autosave {
                        if let Some(recent) = export::most_recent_save() {
                            match export::load_world(&recent.path) {
                                Ok(mut loaded) => {
                                    let label = if recent.is_autosave {
                                        "Resumed from autosave.".to_string()
                                    } else {
                                        format!("Resumed: {}", recent.world_name)
                                    };
                                    loaded.set_status_message(label);
                                    *sim = Some(loaded);
                                    *mode = AppMode::InGame;
                                }
                                Err(_) => {
                                    // Can't load — stay on menu
                                }
                            }
                        }
                    }
                }
                2 => {
                    // Load World
                    let saves = export::list_saves();
                    *mode = AppMode::LoadWorld {
                        saves,
                        selected: 0,
                        confirm_delete: false,
                    };
                }
                3 => {
                    // Quit
                    return InputResult::Quit;
                }
                _ => {}
            }
        }
        KeyCode::Char('q') => return InputResult::Quit,
        _ => {}
    }
    InputResult::Continue
}

// ---------------------------------------------------------------------------
// New World Input
// ---------------------------------------------------------------------------

fn handle_new_world_input(mode: &mut AppMode, key: KeyCode) {
    let (selected_preset, seed_input, editing_seed) =
        if let AppMode::NewWorld { selected_preset, seed_input, editing_seed } = mode {
            (selected_preset, seed_input, editing_seed)
        } else {
            return;
        };

    if *editing_seed {
        match key {
            KeyCode::Esc => {
                *editing_seed = false;
            }
            KeyCode::Enter => {
                // Generate world with chosen preset and seed
                let seed = if seed_input.is_empty() {
                    // Random seed from system time
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .map(|d| d.as_secs())
                        .unwrap_or(42)
                } else {
                    // Hash the seed string to a u64
                    hash_seed_string(seed_input)
                };
                *mode = AppMode::Generating { seed, flavor: *selected_preset, frames_shown: 0 };
            }
            KeyCode::Backspace => {
                seed_input.pop();
            }
            KeyCode::Char(c) => {
                if seed_input.len() < 40 {
                    seed_input.push(c);
                }
            }
            _ => {}
        }
    } else {
        match key {
            KeyCode::Esc => {
                *mode = AppMode::MainMenu {
                    selected: 0,
                    has_autosave: export::most_recent_save().is_some(),
                };
            }
            KeyCode::Up => {
                if *selected_preset > 0 {
                    *selected_preset -= 1;
                }
            }
            KeyCode::Down => {
                if *selected_preset < ui::menu::FLAVOR_PRESETS.len() - 1 {
                    *selected_preset += 1;
                }
            }
            KeyCode::Tab => {
                *editing_seed = true;
            }
            KeyCode::Enter => {
                // Generate with selected preset and current seed input
                let seed = if seed_input.is_empty() {
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .map(|d| d.as_secs())
                        .unwrap_or(42)
                } else {
                    hash_seed_string(seed_input)
                };
                *mode = AppMode::Generating { seed, flavor: *selected_preset, frames_shown: 0 };
            }
            _ => {}
        }
    }
}

/// Hash a seed string to a u64 using a simple FNV-1a-like hash.
fn hash_seed_string(s: &str) -> u64 {
    let mut hash: u64 = 14695981039346656037;
    for byte in s.bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(1099511628211);
    }
    hash
}

// ---------------------------------------------------------------------------
// Load World Input
// ---------------------------------------------------------------------------

fn handle_load_world_input(mode: &mut AppMode, sim: &mut Option<SimState>, key: KeyCode) {
    let (saves, selected, confirm_delete) = if let AppMode::LoadWorld { saves, selected, confirm_delete } = mode {
        (saves, selected, confirm_delete)
    } else {
        return;
    };

    // If we're in delete confirmation mode, handle Y/N
    if *confirm_delete {
        match key {
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                if !saves.is_empty() {
                    let path = saves[*selected].path.clone();
                    let _ = export::delete_save(&path);
                    // Refresh the save list
                    *saves = export::list_saves();
                    if *selected >= saves.len() && !saves.is_empty() {
                        *selected = saves.len() - 1;
                    }
                }
                *confirm_delete = false;
            }
            _ => {
                // Any other key cancels delete
                *confirm_delete = false;
            }
        }
        return;
    }

    match key {
        KeyCode::Esc => {
            *mode = AppMode::MainMenu {
                selected: 2,
                has_autosave: export::most_recent_save().is_some(),
            };
        }
        KeyCode::Up => {
            if *selected > 0 {
                *selected -= 1;
            }
        }
        KeyCode::Down => {
            if !saves.is_empty() && *selected < saves.len() - 1 {
                *selected += 1;
            }
        }
        KeyCode::Enter => {
            if !saves.is_empty() {
                let path = saves[*selected].path.clone();
                match export::load_world(&path) {
                    Ok(mut loaded) => {
                        let name = saves[*selected].world_name.clone();
                        loaded.set_status_message(format!("Loaded: {}", name));
                        *sim = Some(loaded);
                        *mode = AppMode::InGame;
                    }
                    Err(_) => {
                        // Stay on load screen — could show error in future
                    }
                }
            }
        }
        KeyCode::Char('d') | KeyCode::Char('D') | KeyCode::Delete => {
            if !saves.is_empty() {
                *confirm_delete = true;
            }
        }
        _ => {}
    }
}

// ---------------------------------------------------------------------------
// World Report Input (pre-sim)
// ---------------------------------------------------------------------------

fn handle_world_report_input(
    mode: &mut AppMode,
    sim: &mut Option<SimState>,
    key: KeyCode,
) -> InputResult {
    let (scroll, flavor) = if let AppMode::WorldReport { scroll, flavor } = mode {
        (scroll, *flavor)
    } else {
        return InputResult::Continue;
    };

    match key {
        KeyCode::Enter => {
            // Begin simulation
            *mode = AppMode::InGame;
        }
        KeyCode::Char('r') | KeyCode::Char('R') => {
            // Reroll: generate a new random seed with same flavor and regenerate
            let new_seed = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos() as u64)
                .unwrap_or(42);
            let wf = world_gen::WorldFlavor::from_index(flavor);
            let (world, agents, institutions, sites, artifacts) = world_gen::generate_world(new_seed, wf);
            *sim = Some(SimState::new(world, agents, institutions, sites, artifacts));
            *mode = AppMode::WorldReport { scroll: 0, flavor };
        }
        KeyCode::Up => {
            *scroll = scroll.saturating_sub(1);
        }
        KeyCode::Down => {
            *scroll += 1;
        }
        KeyCode::PageUp => {
            *scroll = scroll.saturating_sub(10);
        }
        KeyCode::PageDown => {
            *scroll += 10;
        }
        KeyCode::Esc => {
            // Return to main menu, discard world
            *sim = None;
            *mode = AppMode::MainMenu {
                selected: 0,
                has_autosave: export::most_recent_save().is_some(),
            };
        }
        _ => {}
    }
    InputResult::Continue
}

// ---------------------------------------------------------------------------
// In-Game Input
// ---------------------------------------------------------------------------

/// Handle input when the simulation is running. Returns Quit if app should exit.
fn handle_game_input(sim: &mut SimState, key: KeyCode, modifiers: KeyModifiers) -> InputResult {
    match &sim.overlay {
        Overlay::None => handle_main_game_input(sim, key, modifiers),
        Overlay::InspectAgent(_) => { handle_inspect_input(sim, key); InputResult::Continue }
        Overlay::AgentSearch(_, _) => { handle_search_input(sim, key); InputResult::Continue }
        Overlay::AgentList(_) => { handle_agent_list_input(sim, key); InputResult::Continue }
        Overlay::FactionList(_) => { handle_faction_list_input(sim, key); InputResult::Continue }
        Overlay::Help => { if matches!(key, KeyCode::Esc | KeyCode::Char('?')) { sim.overlay = Overlay::None; } InputResult::Continue }
        Overlay::SiteList(_) => { handle_site_list_input(sim, key); InputResult::Continue }
        Overlay::WorldReport(_) => { handle_world_report_overlay_input(sim, key); InputResult::Continue }
        Overlay::SiteView(_, _) => { handle_site_view_input(sim, key); InputResult::Continue }
        Overlay::FollowSelect(_) => { handle_follow_select_input(sim, key); InputResult::Continue }
        Overlay::FollowAgentPick(_) => { handle_follow_agent_pick_input(sim, key); InputResult::Continue }
        Overlay::FollowInstitutionPick(_) => { handle_follow_institution_pick_input(sim, key); InputResult::Continue }
        Overlay::Annals(_) => { handle_annals_input(sim, key); InputResult::Continue }
        Overlay::ExportMenu => { handle_export_menu_input(sim, key); InputResult::Continue }
        Overlay::ExportInput(_) => { handle_export_input(sim, key); InputResult::Continue }
        Overlay::SaveNameInput(_) => { handle_save_name_input(sim, key); InputResult::Continue }
        Overlay::QuitConfirm(_) => handle_quit_confirm_input(sim, key),
    }
}

/// Input handling for the main simulation view.
fn handle_main_game_input(sim: &mut SimState, key: KeyCode, modifiers: KeyModifiers) -> InputResult {
    // Check for Ctrl+Shift+S first (Save As — always prompt)
    if modifiers.contains(KeyModifiers::CONTROL) && modifiers.contains(KeyModifiers::SHIFT)
        && (key == KeyCode::Char('S') || key == KeyCode::Char('s'))
    {
        let default_name = sim.save_name.clone().unwrap_or_default();
        sim.overlay = Overlay::SaveNameInput(default_name);
        return InputResult::Continue;
    }

    // Check for Ctrl+S (Save — silent re-save if name exists, prompt if new)
    if modifiers.contains(KeyModifiers::CONTROL) && key == KeyCode::Char('s') {
        if let Some(ref name) = sim.save_name {
            // Already has a save name — save silently
            let name = name.clone();
            match export::save_world(sim, &name) {
                Ok(path) => sim.set_status_message(format!("Saved to {}", path)),
                Err(e) => sim.set_status_message(format!("Save failed: {}", e)),
            }
        } else {
            // No save name yet — prompt for one
            sim.overlay = Overlay::SaveNameInput(String::new());
        }
        return InputResult::Continue;
    }

    match key {
        KeyCode::Char('q') => {
            sim.overlay = Overlay::QuitConfirm(0);
            return InputResult::Continue;
        }
        KeyCode::Tab => {
            sim.overlay = Overlay::AgentList(0);
            return InputResult::Continue;
        }
        KeyCode::Char(' ') => {
            if sim.speed == SimSpeed::Paused {
                // Unpause: restore previous speed, default to 1x
                sim.speed = sim.pre_pause_speed.take().unwrap_or(SimSpeed::Run1x);
            } else {
                // Pause: remember current speed
                sim.pre_pause_speed = Some(sim.speed);
                sim.speed = SimSpeed::Paused;
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
            sim.overlay = Overlay::AgentSearch(String::new(), 0);
        }
        KeyCode::Char('f') => {
            // Toggle follow mode: if following, stop; otherwise open selection
            if sim.follow_target.is_some() {
                sim.follow_target = None;
                sim.set_status_message("Stopped following.".to_string());
            } else {
                sim.overlay = Overlay::FollowSelect(0);
            }
        }
        KeyCode::Char('F') => {
            sim.overlay = Overlay::FactionList(0);
        }
        KeyCode::Char('e') => {
            sim.overlay = Overlay::ExportMenu;
        }
        KeyCode::Char('?') => {
            sim.overlay = Overlay::Help;
        }
        KeyCode::Char('W') => {
            sim.overlay = Overlay::WorldReport(0);
        }
        KeyCode::Char('a') => {
            sim.overlay = Overlay::Annals(0);
        }
        KeyCode::Char('s') => {
            if !sim.sites.is_empty() {
                sim.overlay = Overlay::SiteList(0);
            }
        }
        _ => {}
    }
    InputResult::Continue
}

/// Input handling when inspecting an agent.
fn handle_inspect_input(sim: &mut SimState, key: KeyCode) {
    match key {
        KeyCode::Esc => sim.overlay = Overlay::None,
        _ => {}
    }
}

/// Input handling for the agent search overlay with selectable results.
fn handle_search_input(sim: &mut SimState, key: KeyCode) {
    let (query, selected) = if let Overlay::AgentSearch(ref q, sel) = sim.overlay {
        (q.clone(), sel)
    } else {
        return;
    };

    match key {
        KeyCode::Esc => {
            sim.overlay = Overlay::None;
        }
        KeyCode::Enter => {
            if query.len() >= 2 {
                let matches = sim.search_agents(&query);
                if let Some(&idx) = matches.get(selected) {
                    sim.overlay = Overlay::InspectAgent(idx);
                } else {
                    sim.set_status_message("No matching agents found.".to_string());
                    sim.overlay = Overlay::None;
                }
            }
        }
        KeyCode::Up => {
            let new_sel = selected.saturating_sub(1);
            sim.overlay = Overlay::AgentSearch(query, new_sel);
        }
        KeyCode::Down => {
            let matches = sim.search_agents(&query);
            let max = matches.len().min(15).saturating_sub(1);
            let new_sel = (selected + 1).min(max);
            sim.overlay = Overlay::AgentSearch(query, new_sel);
        }
        KeyCode::Backspace => {
            let mut q = query;
            q.pop();
            sim.overlay = Overlay::AgentSearch(q, 0);
        }
        KeyCode::Char(c) => {
            let mut q = query;
            q.push(c);
            sim.overlay = Overlay::AgentSearch(q, 0);
        }
        _ => {}
    }
}

/// Input handling for the agent list overlay (Tab).
fn handle_agent_list_input(sim: &mut SimState, key: KeyCode) {
    let selected = if let Overlay::AgentList(sel) = sim.overlay {
        sel
    } else {
        return;
    };

    let living = sim.living_agent_indices();
    let max_idx = living.len().saturating_sub(1);

    match key {
        KeyCode::Esc => {
            sim.overlay = Overlay::None;
        }
        KeyCode::Up => {
            sim.overlay = Overlay::AgentList(selected.saturating_sub(1));
        }
        KeyCode::Down => {
            sim.overlay = Overlay::AgentList((selected + 1).min(max_idx));
        }
        KeyCode::Enter => {
            if let Some(&idx) = living.get(selected) {
                sim.overlay = Overlay::InspectAgent(idx);
            }
        }
        _ => {}
    }
}

/// Input handling for the quit confirm overlay (Q key).
fn handle_quit_confirm_input(sim: &mut SimState, key: KeyCode) -> InputResult {
    let selected = if let Overlay::QuitConfirm(sel) = sim.overlay {
        sel
    } else {
        return InputResult::Continue;
    };

    match key {
        KeyCode::Esc => {
            sim.overlay = Overlay::None;
        }
        KeyCode::Up => {
            sim.overlay = Overlay::QuitConfirm(selected.saturating_sub(1));
        }
        KeyCode::Down => {
            sim.overlay = Overlay::QuitConfirm((selected + 1).min(2));
        }
        KeyCode::Enter => {
            match selected {
                0 => {
                    // Save and return to menu
                    let name = sim.save_name.clone()
                        .unwrap_or_else(|| sim.world.name.clone());
                    let _ = export::save_world(sim, &name);
                    sim.overlay = Overlay::None;
                    return InputResult::ReturnToMenu;
                }
                1 => {
                    // Return without saving
                    sim.overlay = Overlay::None;
                    return InputResult::ReturnToMenu;
                }
                2 => {
                    // Cancel
                    sim.overlay = Overlay::None;
                }
                _ => {}
            }
        }
        _ => {}
    }
    InputResult::Continue
}

/// Input handling for the follow select overlay (f key — pick agent or institution).
fn handle_follow_select_input(sim: &mut SimState, key: KeyCode) {
    let selected = if let Overlay::FollowSelect(sel) = sim.overlay { sel } else { return; };
    match key {
        KeyCode::Esc => { sim.overlay = Overlay::None; }
        KeyCode::Up => { sim.overlay = Overlay::FollowSelect(selected.saturating_sub(1)); }
        KeyCode::Down => { sim.overlay = Overlay::FollowSelect((selected + 1).min(1)); }
        KeyCode::Enter => {
            match selected {
                0 => sim.overlay = Overlay::FollowAgentPick(0),
                1 => sim.overlay = Overlay::FollowInstitutionPick(0),
                _ => {}
            }
        }
        _ => {}
    }
}

/// Input handling for the follow agent picker.
fn handle_follow_agent_pick_input(sim: &mut SimState, key: KeyCode) {
    let selected = if let Overlay::FollowAgentPick(sel) = sim.overlay { sel } else { return; };
    let living = sim.living_agent_indices();
    let max_idx = living.len().saturating_sub(1);
    match key {
        KeyCode::Esc => { sim.overlay = Overlay::FollowSelect(0); }
        KeyCode::Up => { sim.overlay = Overlay::FollowAgentPick(selected.saturating_sub(1)); }
        KeyCode::Down => { sim.overlay = Overlay::FollowAgentPick((selected + 1).min(max_idx)); }
        KeyCode::Enter => {
            if let Some(&idx) = living.get(selected) {
                let agent_id = sim.agents[idx].id;
                sim.follow_target = Some(crate::sim::FollowTarget::Agent(agent_id));
                sim.overlay = Overlay::None;
                sim.set_status_message(format!("Following {}.", sim.agents[idx].display_name()));
            }
        }
        _ => {}
    }
}

/// Input handling for the follow institution picker.
fn handle_follow_institution_pick_input(sim: &mut SimState, key: KeyCode) {
    let selected = if let Overlay::FollowInstitutionPick(sel) = sim.overlay { sel } else { return; };
    let living = sim.living_institution_indices();
    let max_idx = living.len().saturating_sub(1);
    match key {
        KeyCode::Esc => { sim.overlay = Overlay::FollowSelect(1); }
        KeyCode::Up => { sim.overlay = Overlay::FollowInstitutionPick(selected.saturating_sub(1)); }
        KeyCode::Down => { sim.overlay = Overlay::FollowInstitutionPick((selected + 1).min(max_idx)); }
        KeyCode::Enter => {
            if let Some(&idx) = living.get(selected) {
                let inst_id = sim.institutions[idx].id;
                let inst_name = sim.institutions[idx].name.clone();
                sim.follow_target = Some(crate::sim::FollowTarget::Institution(inst_id));
                sim.overlay = Overlay::None;
                sim.set_status_message(format!("Following {}.", inst_name));
            }
        }
        _ => {}
    }
}

/// Input handling for the site list overlay (s key).
fn handle_site_list_input(sim: &mut SimState, key: KeyCode) {
    let selected = if let Overlay::SiteList(sel) = sim.overlay { sel } else { return; };
    let max_idx = sim.sites.len().saturating_sub(1);
    match key {
        KeyCode::Esc => { sim.overlay = Overlay::None; }
        KeyCode::Up => { sim.overlay = Overlay::SiteList(selected.saturating_sub(1)); }
        KeyCode::Down => { sim.overlay = Overlay::SiteList((selected + 1).min(max_idx)); }
        KeyCode::Enter => {
            if selected < sim.sites.len() {
                sim.overlay = Overlay::SiteView(selected, 0);
            }
        }
        _ => {}
    }
}

/// Input handling for the site view (viewing a dungeon floor).
fn handle_site_view_input(sim: &mut SimState, key: KeyCode) {
    let (site_idx, floor_idx) = if let Overlay::SiteView(si, fi) = sim.overlay {
        (si, fi)
    } else {
        return;
    };
    match key {
        KeyCode::Esc => { sim.overlay = Overlay::None; }
        // Navigate floors with < and >
        KeyCode::Char('<') | KeyCode::Char(',') => {
            if floor_idx > 0 {
                sim.overlay = Overlay::SiteView(site_idx, floor_idx - 1);
            }
        }
        KeyCode::Char('>') => {
            if let Some(site) = sim.sites.get(site_idx) {
                if floor_idx + 1 < site.floors.len() {
                    sim.overlay = Overlay::SiteView(site_idx, floor_idx + 1);
                }
            }
        }
        // Allow simulation speed controls while viewing a site
        KeyCode::Char(' ') => {
            if sim.speed == SimSpeed::Paused {
                sim.speed = sim.pre_pause_speed.take().unwrap_or(SimSpeed::Run1x);
            } else {
                sim.pre_pause_speed = Some(sim.speed);
                sim.speed = SimSpeed::Paused;
            }
        }
        KeyCode::Char('1') => sim.speed = SimSpeed::Run1x,
        KeyCode::Char('5') => sim.speed = SimSpeed::Run5x,
        KeyCode::Char('2') => sim.speed = SimSpeed::Run20x,
        KeyCode::Char('.') => {
            if sim.speed == SimSpeed::Paused {
                sim.tick();
            }
        }
        _ => {}
    }
}

/// Input handling for the in-game world report overlay (W key).
fn handle_world_report_overlay_input(sim: &mut SimState, key: KeyCode) {
    let scroll = if let Overlay::WorldReport(s) = sim.overlay { s } else { return; };
    match key {
        KeyCode::Esc => { sim.overlay = Overlay::None; }
        KeyCode::Up => { sim.overlay = Overlay::WorldReport(scroll.saturating_sub(1)); }
        KeyCode::Down => { sim.overlay = Overlay::WorldReport(scroll + 1); }
        KeyCode::PageUp => { sim.overlay = Overlay::WorldReport(scroll.saturating_sub(10)); }
        KeyCode::PageDown => { sim.overlay = Overlay::WorldReport(scroll + 10); }
        _ => {}
    }
}

/// Input handling for the faction list overlay (Shift+F).
fn handle_faction_list_input(sim: &mut SimState, key: KeyCode) {
    let selected = if let Overlay::FactionList(sel) = sim.overlay {
        sel
    } else {
        return;
    };

    let living = sim.living_institution_indices();
    let max_idx = living.len().saturating_sub(1);

    match key {
        KeyCode::Esc => {
            sim.overlay = Overlay::None;
        }
        KeyCode::Up => {
            sim.overlay = Overlay::FactionList(selected.saturating_sub(1));
        }
        KeyCode::Down => {
            sim.overlay = Overlay::FactionList((selected + 1).min(max_idx));
        }
        _ => {}
    }
}

/// Input handling for the World Annals overlay.
fn handle_annals_input(sim: &mut SimState, key: KeyCode) {
    let scroll = if let Overlay::Annals(s) = sim.overlay { s } else { return; };
    match key {
        KeyCode::Esc => { sim.overlay = Overlay::None; }
        KeyCode::Up => { sim.overlay = Overlay::Annals(scroll.saturating_sub(1)); }
        KeyCode::Down => { sim.overlay = Overlay::Annals(scroll + 1); }
        KeyCode::PageUp => { sim.overlay = Overlay::Annals(scroll.saturating_sub(10)); }
        KeyCode::PageDown => { sim.overlay = Overlay::Annals(scroll + 10); }
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
        KeyCode::Char('2') => {
            match export::export_faction_record(sim, "factions") {
                Ok(path) => sim.set_status_message(format!("Exported to {}", path)),
                Err(e) => sim.set_status_message(format!("Export failed: {}", e)),
            }
            sim.overlay = Overlay::None;
        }
        KeyCode::Char('3') => {
            match export::export_character_chronicle(sim, "chronicles") {
                Ok(path) => sim.set_status_message(format!("Exported to {}", path)),
                Err(e) => sim.set_status_message(format!("Export failed: {}", e)),
            }
            sim.overlay = Overlay::None;
        }
        KeyCode::Char('4') => {
            match export::export_world_annals(sim, "annals") {
                Ok(path) => sim.set_status_message(format!("Exported to {}", path)),
                Err(e) => sim.set_status_message(format!("Export failed: {}", e)),
            }
            sim.overlay = Overlay::None;
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

/// Input handling for the save name input overlay (Ctrl+S first save, or Ctrl+Shift+S Save As).
fn handle_save_name_input(sim: &mut SimState, key: KeyCode) {
    let input = if let Overlay::SaveNameInput(ref s) = sim.overlay {
        s.clone()
    } else {
        return;
    };

    match key {
        KeyCode::Esc => {
            sim.overlay = Overlay::None;
        }
        KeyCode::Enter => {
            let name = if input.is_empty() {
                sim.world.name.clone()
            } else {
                input
            };
            match export::save_world(sim, &name) {
                Ok(path) => {
                    sim.save_name = Some(name);
                    sim.set_status_message(format!("Saved to {}", path));
                }
                Err(e) => {
                    sim.set_status_message(format!("Save failed: {}", e));
                }
            }
            sim.overlay = Overlay::None;
        }
        KeyCode::Backspace => {
            let mut s = input;
            s.pop();
            sim.overlay = Overlay::SaveNameInput(s);
        }
        KeyCode::Char(c) => {
            let mut s = input;
            s.push(c);
            sim.overlay = Overlay::SaveNameInput(s);
        }
        _ => {}
    }
}
