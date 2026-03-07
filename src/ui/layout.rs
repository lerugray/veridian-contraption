use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::sim::{Overlay, SimState};
use crate::sim::world::{MAP_HEIGHT, MAP_WIDTH};
use crate::sim::site::{FLOOR_HEIGHT, FLOOR_WIDTH};
use crate::ui::overlays;

/// Colors assigned to agents based on their people_id (bright, legible on dark bg).
const PEOPLE_COLORS: [Color; 8] = [
    Color::Rgb(230, 120, 220), // orchid
    Color::Rgb(100, 220, 230), // teal
    Color::Rgb(240, 220, 100), // gold
    Color::Rgb(120, 230, 130), // mint
    Color::Rgb(240, 130, 110), // coral
    Color::Rgb(130, 170, 255), // periwinkle
    Color::Rgb(255, 180, 100), // amber
    Color::Rgb(180, 140, 255), // lavender
];

/// Draw the main two-panel layout: world map (left) and live log (right).
/// Then draw any active overlay on top.
pub fn draw_main_layout(frame: &mut Frame, sim: &SimState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),    // main panels
            Constraint::Length(1), // status bar
        ])
        .split(frame.area());

    let panels = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(60),
            Constraint::Percentage(40),
        ])
        .split(chunks[0]);

    // If viewing a site, draw site floor instead of world map
    if let Overlay::SiteView(site_idx, floor_idx) = &sim.overlay {
        draw_site_panel(frame, panels[0], sim, *site_idx, *floor_idx);
    } else {
        draw_map_panel(frame, panels[0], sim);
    }

    // If following, split the right pane: top = log, bottom = chronicle
    if sim.follow_target.is_some() {
        let right_split = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(55),
                Constraint::Percentage(45),
            ])
            .split(panels[1]);
        draw_log_panel(frame, right_split[0], sim);
        draw_follow_panel(frame, right_split[1], sim);
    } else {
        draw_log_panel(frame, panels[1], sim);
    }

    draw_status_bar(frame, chunks[1], sim);

    // Draw overlays on top of the main layout
    match &sim.overlay {
        Overlay::None => {}
        Overlay::InspectAgent(idx) => {
            overlays::draw_inspect_overlay(frame, sim, *idx);
        }
        Overlay::AgentSearch(query, selected) => {
            overlays::draw_search_overlay(frame, sim, query, *selected);
        }
        Overlay::AgentList(selected) => {
            overlays::draw_agent_list(frame, sim, *selected);
        }
        Overlay::FactionList(selected) => {
            overlays::draw_faction_list(frame, sim, *selected);
        }
        Overlay::FactionDetail(inst_idx, scroll) => {
            overlays::draw_faction_detail(frame, sim, *inst_idx, *scroll);
        }
        Overlay::FollowSelect(selected) => {
            overlays::draw_follow_select(frame, *selected);
        }
        Overlay::FollowAgentPick(selected) => {
            overlays::draw_follow_agent_pick(frame, sim, *selected);
        }
        Overlay::FollowInstitutionPick(selected) => {
            overlays::draw_follow_institution_pick(frame, sim, *selected);
        }
        Overlay::Help => {
            overlays::draw_help(frame);
        }
        Overlay::WorldReport(scroll) => {
            overlays::draw_world_report_fullscreen(frame, sim, *scroll, false);
        }
        Overlay::SiteList(selected) => {
            overlays::draw_site_list(frame, sim, *selected);
        }
        Overlay::SiteView(_, _) => {
            // Site view replaces the map panel, handled above — no popup overlay needed
        }
        Overlay::Annals(scroll) => {
            overlays::draw_annals(frame, sim, *scroll);
        }
        Overlay::ExportMenu => {
            overlays::draw_export_menu(frame);
        }
        Overlay::ExportInput(input) => {
            overlays::draw_export_input(frame, input);
        }
        Overlay::SaveNameInput(input) => {
            overlays::draw_save_name_input(frame, input);
        }
        Overlay::QuitConfirm(selected) => {
            overlays::draw_quit_confirm(frame, *selected);
        }
        Overlay::EschatonConfirm(selected) => {
            overlays::draw_eschaton_confirm(frame, sim, *selected);
        }
        Overlay::MapLegend => {
            overlays::draw_map_legend(frame);
        }
    }
}

fn draw_map_panel(frame: &mut Frame, area: Rect, sim: &SimState) {
    let block = Block::default()
        .title(format!(" {} ", sim.world.name))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(60, 60, 70)));

    let mut rendered = sim.world.render_map();

    // Count agents per tile and track people_id for coloring
    let mut agent_counts = vec![vec![0u32; MAP_WIDTH]; MAP_HEIGHT];
    let mut agent_people = vec![vec![0usize; MAP_WIDTH]; MAP_HEIGHT];

    for agent in &sim.agents {
        if !agent.alive {
            continue;
        }
        let ax = agent.x as usize;
        let ay = agent.y as usize;
        if ay < MAP_HEIGHT && ax < MAP_WIDTH {
            agent_counts[ay][ax] += 1;
            agent_people[ay][ax] = agent.people_id;
        }
    }

    // Overlay sites on the map (before agents so agents on top of sites show as @)
    for site in &sim.sites {
        let sx = site.grid_x as usize;
        let sy = site.grid_y as usize;
        if sy < MAP_HEIGHT && sx < MAP_WIDTH {
            rendered[sy][sx] = ('Ω', site.kind.map_color());
        }
    }

    // Overlay agents on the map — pulse between @ and • for liveness
    let pulse_bright = (sim.frame_count / 15) % 2 == 0; // toggles every ~0.5s at 30fps
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let count = agent_counts[y][x];
            if count == 0 {
                continue;
            }
            let base_color = PEOPLE_COLORS[agent_people[y][x] % PEOPLE_COLORS.len()];
            // Dim the color slightly on the off-pulse for a breathing effect
            let color = if pulse_bright {
                base_color
            } else {
                dim_color(base_color)
            };
            if count == 1 {
                let ch = if pulse_bright { '@' } else { '\u{2022}' }; // @ or •
                rendered[y][x] = (ch, color);
            } else if count < 10 {
                let ch = char::from_digit(count, 10).unwrap_or('*');
                rendered[y][x] = (ch, color);
            } else {
                rendered[y][x] = ('*', color);
            }
        }
    }

    // Highlight followed agent with a distinct pulsing marker
    if let Some((fx, fy)) = sim.follow_agent_pos() {
        let fx = fx as usize;
        let fy = fy as usize;
        if fy < MAP_HEIGHT && fx < MAP_WIDTH {
            let follow_color = if pulse_bright {
                Color::Rgb(255, 100, 100)
            } else {
                Color::Rgb(200, 70, 70)
            };
            rendered[fy][fx] = ('X', follow_color);
        }
    }

    // Scale the map to fill the panel by distributing extra rows/columns
    // evenly across map tiles (some tiles get one extra char/row).
    let inner_w = area.width.saturating_sub(2) as usize;
    let inner_h = area.height.saturating_sub(2) as usize;

    // Per-column widths: base width + 1 extra char for the first `remainder` columns
    let col_base = inner_w / MAP_WIDTH;
    let col_extra = inner_w % MAP_WIDTH;

    // Per-row heights: base height + 1 extra row for the first `remainder` rows
    let row_base = inner_h / MAP_HEIGHT;
    let row_extra = inner_h % MAP_HEIGHT;

    let mut lines: Vec<Line> = Vec::new();
    for y in 0..MAP_HEIGHT {
        let row_repeats = if y < row_extra { row_base + 1 } else { row_base };
        if row_repeats == 0 {
            continue;
        }

        let spans: Vec<Span> = rendered[y]
            .iter()
            .enumerate()
            .map(|(x, &(ch, color))| {
                let w = if x < col_extra { col_base + 1 } else { col_base };
                let s: String = std::iter::repeat(ch).take(w.max(1)).collect();
                Span::styled(s, Style::default().fg(color))
            })
            .collect();
        let line = Line::from(spans);
        for _ in 0..row_repeats {
            lines.push(line.clone());
        }
    }

    let map_widget = Paragraph::new(lines).block(block);
    frame.render_widget(map_widget, area);
}

fn draw_log_panel(frame: &mut Frame, area: Rect, sim: &SimState) {
    let scrolled = sim.log_scroll > 0;
    let title = if scrolled {
        format!(" LIVE LOG [scrolled +{}] ", sim.log_scroll)
    } else {
        " LIVE LOG ".to_string()
    };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(if scrolled {
            Color::Rgb(200, 170, 80)
        } else {
            Color::Rgb(60, 60, 70)
        }));

    let inner_height = area.height.saturating_sub(2) as usize;
    let inner_width = area.width.saturating_sub(2) as usize;

    if inner_height == 0 || inner_width == 0 || sim.events.is_empty() {
        let empty = Paragraph::new("No events yet.")
            .style(Style::default().fg(Color::Gray))
            .block(block);
        frame.render_widget(empty, area);
        return;
    }

    // Slice events: use frozen view length if scrolled, otherwise live length.
    let pool_len = sim.log_frozen_len.unwrap_or(sim.events.len());
    let end_event = pool_len.saturating_sub(sim.log_scroll).min(sim.events.len());
    let visible_events = &sim.events[..end_event];

    // Build formatted lines from the visible events (events may wrap to multiple display lines).
    let mut all_lines: Vec<Line> = Vec::new();

    // Track the current tick for "new entry" highlighting
    let current_tick = sim.world.tick;

    for (ei, event) in visible_events.iter().enumerate() {
        let tick_str = format!("[{}] ", event.tick);
        let desc = &event.description;
        let category_prefix = event.event_type.category_prefix();
        let full_prefix = format!("{}{}", tick_str, category_prefix);
        let prefix_len = full_prefix.len();
        let body_width = inner_width.saturating_sub(prefix_len);
        let text_color = event.event_type.log_color();
        // Highlight entries from the last 3 ticks with brighter text
        let is_recent = current_tick.saturating_sub(event.tick) < 3 && !scrolled;
        let tick_color = Color::Rgb(70, 70, 80);
        let cat_color = event.event_type.log_color();
        let body_color = if is_recent { brighten_color(text_color) } else { text_color };

        // Add a blank line between entries (not before the first one)
        if ei > 0 {
            all_lines.push(Line::from(""));
        }

        if body_width < 10 {
            all_lines.push(Line::from(vec![
                Span::styled(tick_str.clone(), Style::default().fg(tick_color)),
                Span::styled(category_prefix.clone(), Style::default().fg(cat_color)),
                Span::styled(desc.clone(), Style::default().fg(body_color)),
            ]));
        } else {
            let words: Vec<&str> = desc.split_whitespace().collect();
            let mut line_buf = String::new();
            let mut first = true;

            for word in &words {
                let space = if line_buf.is_empty() { 0 } else { 1 };
                let limit = body_width;

                if line_buf.len() + space + word.len() > limit && !line_buf.is_empty() {
                    if first {
                        all_lines.push(Line::from(vec![
                            Span::styled(tick_str.clone(), Style::default().fg(tick_color)),
                            Span::styled(category_prefix.clone(), Style::default().fg(cat_color)),
                            Span::styled(line_buf.clone(), Style::default().fg(body_color)),
                        ]));
                        first = false;
                    } else {
                        let indent = " ".repeat(prefix_len);
                        all_lines.push(Line::from(vec![
                            Span::styled(indent, Style::default().fg(tick_color)),
                            Span::styled(line_buf.clone(), Style::default().fg(body_color)),
                        ]));
                    }
                    line_buf.clear();
                }

                if !line_buf.is_empty() {
                    line_buf.push(' ');
                }
                line_buf.push_str(word);
            }

            if !line_buf.is_empty() {
                if first {
                    all_lines.push(Line::from(vec![
                        Span::styled(tick_str.clone(), Style::default().fg(tick_color)),
                        Span::styled(category_prefix.clone(), Style::default().fg(cat_color)),
                        Span::styled(line_buf, Style::default().fg(body_color)),
                    ]));
                } else {
                    let indent = " ".repeat(prefix_len);
                    all_lines.push(Line::from(vec![
                        Span::styled(indent, Style::default().fg(tick_color)),
                        Span::styled(line_buf, Style::default().fg(body_color)),
                    ]));
                }
            }
        }
    }

    // Show the last inner_height lines of the visible events
    let total = all_lines.len();
    let start = total.saturating_sub(inner_height);
    let visible: Vec<Line> = all_lines[start..].to_vec();

    let log_widget = Paragraph::new(visible).block(block);
    frame.render_widget(log_widget, area);
}

fn draw_follow_panel(frame: &mut Frame, area: Rect, sim: &SimState) {
    let label = sim.follow_label().unwrap_or_else(|| "???".to_string());
    let title = format!(" FOLLOWING: {} ", label);

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(200, 100, 90)));

    let inner_height = area.height.saturating_sub(2) as usize;
    let inner_width = area.width.saturating_sub(2) as usize;

    let follow_events = sim.follow_events();

    if follow_events.is_empty() || inner_height == 0 || inner_width == 0 {
        let empty = Paragraph::new(" No events recorded for this entity.")
            .style(Style::default().fg(Color::DarkGray))
            .block(block);
        frame.render_widget(empty, area);
        return;
    }

    // Build wrapped lines from follow events (same style as main log)
    let mut all_lines: Vec<Line> = Vec::new();
    for event in &follow_events {
        let tick_str = format!("[{}] ", event.tick);
        let desc = &event.description;
        let category_prefix = event.event_type.category_prefix();
        let full_prefix = format!("{}{}", tick_str, category_prefix);
        let prefix_len = full_prefix.len();
        let body_width = inner_width.saturating_sub(prefix_len);
        let text_color = event.event_type.log_color();
        let tick_color = Color::Rgb(70, 70, 80);
        let cat_color = event.event_type.log_color();

        if body_width < 10 {
            all_lines.push(Line::from(vec![
                Span::styled(tick_str.clone(), Style::default().fg(tick_color)),
                Span::styled(category_prefix.clone(), Style::default().fg(cat_color)),
                Span::styled(desc.clone(), Style::default().fg(text_color)),
            ]));
        } else {
            let words: Vec<&str> = desc.split_whitespace().collect();
            let mut line_buf = String::new();
            let mut first = true;

            for word in &words {
                let space = if line_buf.is_empty() { 0 } else { 1 };
                if line_buf.len() + space + word.len() > body_width && !line_buf.is_empty() {
                    if first {
                        all_lines.push(Line::from(vec![
                            Span::styled(tick_str.clone(), Style::default().fg(tick_color)),
                            Span::styled(category_prefix.clone(), Style::default().fg(cat_color)),
                            Span::styled(line_buf.clone(), Style::default().fg(text_color)),
                        ]));
                        first = false;
                    } else {
                        let indent = " ".repeat(prefix_len);
                        all_lines.push(Line::from(vec![
                            Span::styled(indent, Style::default().fg(tick_color)),
                            Span::styled(line_buf.clone(), Style::default().fg(text_color)),
                        ]));
                    }
                    line_buf.clear();
                }
                if !line_buf.is_empty() { line_buf.push(' '); }
                line_buf.push_str(word);
            }
            if !line_buf.is_empty() {
                if first {
                    all_lines.push(Line::from(vec![
                        Span::styled(tick_str.clone(), Style::default().fg(tick_color)),
                        Span::styled(category_prefix.clone(), Style::default().fg(cat_color)),
                        Span::styled(line_buf, Style::default().fg(text_color)),
                    ]));
                } else {
                    let indent = " ".repeat(prefix_len);
                    all_lines.push(Line::from(vec![
                        Span::styled(indent, Style::default().fg(tick_color)),
                        Span::styled(line_buf, Style::default().fg(text_color)),
                    ]));
                }
            }
        }
    }

    let total = all_lines.len();
    let start = total.saturating_sub(inner_height);
    let visible: Vec<Line> = all_lines[start..].to_vec();

    let widget = Paragraph::new(visible).block(block);
    frame.render_widget(widget, area);
}

fn draw_site_panel(frame: &mut Frame, area: Rect, sim: &SimState, site_idx: usize, floor_idx: usize) {
    let site = match sim.sites.get(site_idx) {
        Some(s) => s,
        None => {
            let block = Block::default().title(" SITE ").borders(Borders::ALL);
            frame.render_widget(Paragraph::new("Site not found.").block(block), area);
            return;
        }
    };
    let floor = match site.floors.get(floor_idx) {
        Some(f) => f,
        None => {
            let block = Block::default().title(" SITE ").borders(Borders::ALL);
            frame.render_widget(Paragraph::new("Floor not found.").block(block), area);
            return;
        }
    };

    let floor_label = if site.floors.len() > 1 {
        format!(" {} — Floor {} of {} [ESC=back, </>=floors] ",
                site.name, floor_idx + 1, site.floors.len())
    } else {
        format!(" {} [ESC=back] ", site.name)
    };

    let block = Block::default()
        .title(floor_label)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(site.kind.map_color()));

    // Build agent positions within this site for overlay
    // (For now, agents don't actually move within sites — just show population)
    let agent_positions: Vec<(usize, usize, usize)> = site.population.iter()
        .filter_map(|&aid| {
            sim.agents.iter().find(|a| a.id == aid && a.alive).map(|a| {
                // Place agents in random-ish spots within rooms
                let room_idx = (aid as usize) % floor.rooms.len().max(1);
                if let Some(room) = floor.rooms.get(room_idx) {
                    let (cx, cy) = room.center();
                    (cx, cy, a.people_id)
                } else {
                    (FLOOR_WIDTH / 2, FLOOR_HEIGHT / 2, a.people_id)
                }
            })
        })
        .collect();

    let inner_w = area.width.saturating_sub(2) as usize;
    let inner_h = area.height.saturating_sub(2) as usize;

    // Build the tile grid lines
    let mut lines: Vec<Line> = Vec::new();

    // Scale: try 1:1 first, stretch if panel is bigger
    let col_base = inner_w / FLOOR_WIDTH;
    let col_extra = inner_w % FLOOR_WIDTH;
    let row_base = inner_h / FLOOR_HEIGHT;
    let row_extra = inner_h % FLOOR_HEIGHT;

    for y in 0..FLOOR_HEIGHT {
        let row_repeats = if y < row_extra { row_base + 1 } else { row_base };
        if row_repeats == 0 {
            continue;
        }

        let spans: Vec<Span> = (0..FLOOR_WIDTH)
            .map(|x| {
                let w = if x < col_extra { col_base + 1 } else { col_base };

                // Check if an agent is at this position
                if let Some((_, _, people_id)) = agent_positions.iter().find(|(ax, ay, _)| *ax == x && *ay == y) {
                    let color = PEOPLE_COLORS[people_id % PEOPLE_COLORS.len()];
                    let s: String = std::iter::repeat('@').take(w.max(1)).collect();
                    Span::styled(s, Style::default().fg(color))
                } else {
                    let tile = floor.tiles[y][x];
                    let s: String = std::iter::repeat(tile.glyph()).take(w.max(1)).collect();
                    Span::styled(s, Style::default().fg(tile.color()))
                }
            })
            .collect();

        let line = Line::from(spans);
        for _ in 0..row_repeats {
            lines.push(line.clone());
        }
    }

    let widget = Paragraph::new(lines).block(block);
    frame.render_widget(widget, area);
}

fn draw_status_bar(frame: &mut Frame, area: Rect, sim: &SimState) {
    // Eschaton flash takes priority over everything
    if sim.eschaton_flash > 0 {
        let flash_color = if sim.eschaton_flash % 10 < 5 {
            Color::Rgb(255, 60, 60)
        } else {
            Color::Rgb(255, 200, 60)
        };
        let status = Paragraph::new(" \u{2593}\u{2593}\u{2593}  THE ESCHATON HAS OCCURRED  \u{2593}\u{2593}\u{2593}")
            .style(Style::default().fg(flash_color).bg(Color::Rgb(20, 10, 10)));
        frame.render_widget(status, area);
        return;
    }

    // If there's a temporary status message, show it instead of the default bar
    if let Some((ref msg, _)) = sim.status_message {
        let status = Paragraph::new(format!(" {}", msg))
            .style(Style::default().fg(Color::Rgb(220, 200, 100)).bg(Color::Rgb(30, 30, 40)));
        frame.render_widget(status, area);
        return;
    }

    let alive_count = sim.agents.iter().filter(|a| a.alive).count();
    let save_label = sim
        .save_name
        .as_deref()
        .unwrap_or("unsaved");

    // Show site info if viewing a site
    let site_hint = if let Overlay::SiteView(si, fi) = &sim.overlay {
        if let Some(site) = sim.sites.get(*si) {
            format!("  |  {} (F{})", site.name, fi + 1)
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    // Activity spinner when sim is running
    let spinner = if sim.speed != crate::sim::SimSpeed::Paused {
        const SPINNER: &[char] = &['◜', '◝', '◞', '◟'];
        let idx = (sim.frame_count / 4) as usize % SPINNER.len();
        format!("{} ", SPINNER[idx])
    } else {
        "  ".to_string()
    };

    let status_line = Line::from(vec![
        Span::styled(&spinner, Style::default().fg(Color::Rgb(100, 200, 120)).bg(Color::Rgb(30, 30, 40))),
        Span::styled(
            format!("{}  ", sim.world.name),
            Style::default().fg(Color::Rgb(180, 200, 160)).bg(Color::Rgb(30, 30, 40)),
        ),
        Span::styled(
            format!("Tick {}  ", sim.world.tick),
            Style::default().fg(Color::Rgb(140, 140, 150)).bg(Color::Rgb(30, 30, 40)),
        ),
        Span::styled(
            format!("{}  ", sim.speed.label()),
            Style::default().fg(if sim.speed == crate::sim::SimSpeed::Paused {
                Color::Rgb(200, 160, 80)
            } else {
                Color::Rgb(100, 200, 120)
            }).bg(Color::Rgb(30, 30, 40)),
        ),
        Span::styled(
            format!("Pop: {}  ", alive_count),
            Style::default().fg(Color::Rgb(140, 140, 150)).bg(Color::Rgb(30, 30, 40)),
        ),
        Span::styled(
            format!("[{}]", save_label),
            Style::default().fg(Color::Rgb(100, 100, 110)).bg(Color::Rgb(30, 30, 40)),
        ),
        Span::styled(
            site_hint,
            Style::default().fg(Color::Rgb(180, 140, 100)).bg(Color::Rgb(30, 30, 40)),
        ),
        Span::styled(
            "  ?=help l=legend",
            Style::default().fg(Color::Rgb(80, 80, 90)).bg(Color::Rgb(30, 30, 40)),
        ),
    ]);

    let status = Paragraph::new(status_line)
        .style(Style::default().bg(Color::Rgb(30, 30, 40)));
    frame.render_widget(status, area);
}

/// Dim a Color by reducing its brightness (for pulse animation).
fn dim_color(color: Color) -> Color {
    match color {
        Color::Rgb(r, g, b) => Color::Rgb(
            (r as u16 * 6 / 10) as u8,
            (g as u16 * 6 / 10) as u8,
            (b as u16 * 6 / 10) as u8,
        ),
        other => other,
    }
}

/// Brighten a Color for highlighting new log entries.
fn brighten_color(color: Color) -> Color {
    match color {
        Color::Rgb(r, g, b) => Color::Rgb(
            (r as u16 + (255 - r as u16) / 3).min(255) as u8,
            (g as u16 + (255 - g as u16) / 3).min(255) as u8,
            (b as u16 + (255 - b as u16) / 3).min(255) as u8,
        ),
        // For non-RGB colors, just return White as a bright fallback
        Color::White => Color::White,
        Color::Cyan => Color::LightCyan,
        Color::Yellow => Color::LightYellow,
        Color::Green => Color::LightGreen,
        Color::Red => Color::LightRed,
        Color::Magenta => Color::LightMagenta,
        other => other,
    }
}
