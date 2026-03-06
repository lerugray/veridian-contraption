use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::sim::{Overlay, SimState};
use crate::sim::world::{MAP_HEIGHT, MAP_WIDTH};
use crate::ui::overlays;

/// Colors assigned to agents based on their people_id.
const PEOPLE_COLORS: [Color; 6] = [
    Color::Magenta,
    Color::Cyan,
    Color::LightYellow,
    Color::LightGreen,
    Color::LightRed,
    Color::LightBlue,
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

    draw_map_panel(frame, panels[0], sim);

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
    }
}

fn draw_map_panel(frame: &mut Frame, area: Rect, sim: &SimState) {
    let block = Block::default()
        .title(format!(" {} ", sim.world.name))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

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

    // Overlay agents on the map
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let count = agent_counts[y][x];
            if count == 0 {
                continue;
            }
            let color = PEOPLE_COLORS[agent_people[y][x] % PEOPLE_COLORS.len()];
            if count == 1 {
                rendered[y][x] = ('@', color);
            } else if count < 10 {
                let ch = char::from_digit(count, 10).unwrap_or('*');
                rendered[y][x] = (ch, color);
            } else {
                rendered[y][x] = ('*', color);
            }
        }
    }

    // Highlight followed agent with a distinct marker
    if let Some((fx, fy)) = sim.follow_agent_pos() {
        let fx = fx as usize;
        let fy = fy as usize;
        if fy < MAP_HEIGHT && fx < MAP_WIDTH {
            rendered[fy][fx] = ('X', Color::LightRed);
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
            Color::Yellow
        } else {
            Color::DarkGray
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

    for event in visible_events {
        let tick_str = format!("[{}] ", event.tick);
        let desc = &event.description;
        let prefix_len = tick_str.len();
        let body_width = inner_width.saturating_sub(prefix_len);
        let text_color = event.event_type.log_color();

        if body_width < 10 {
            all_lines.push(Line::from(vec![
                Span::styled(tick_str.clone(), Style::default().fg(Color::DarkGray)),
                Span::styled(desc.clone(), Style::default().fg(text_color)),
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
                            Span::styled(tick_str.clone(), Style::default().fg(Color::DarkGray)),
                            Span::styled(line_buf.clone(), Style::default().fg(text_color)),
                        ]));
                        first = false;
                    } else {
                        let indent = " ".repeat(prefix_len);
                        all_lines.push(Line::from(vec![
                            Span::styled(indent, Style::default().fg(Color::DarkGray)),
                            Span::styled(line_buf.clone(), Style::default().fg(text_color)),
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
                        Span::styled(tick_str.clone(), Style::default().fg(Color::DarkGray)),
                        Span::styled(line_buf, Style::default().fg(text_color)),
                    ]));
                } else {
                    let indent = " ".repeat(prefix_len);
                    all_lines.push(Line::from(vec![
                        Span::styled(indent, Style::default().fg(Color::DarkGray)),
                        Span::styled(line_buf, Style::default().fg(text_color)),
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
        .border_style(Style::default().fg(Color::LightRed));

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

    // Build wrapped lines from follow events (reuse same word-wrap logic)
    let mut all_lines: Vec<Line> = Vec::new();
    for event in &follow_events {
        let tick_str = format!("[{}] ", event.tick);
        let desc = &event.description;
        let prefix_len = tick_str.len();
        let body_width = inner_width.saturating_sub(prefix_len);
        let text_color = event.event_type.log_color();

        if body_width < 10 {
            all_lines.push(Line::from(vec![
                Span::styled(tick_str.clone(), Style::default().fg(Color::DarkGray)),
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
                            Span::styled(tick_str.clone(), Style::default().fg(Color::DarkGray)),
                            Span::styled(line_buf.clone(), Style::default().fg(text_color)),
                        ]));
                        first = false;
                    } else {
                        let indent = " ".repeat(prefix_len);
                        all_lines.push(Line::from(vec![
                            Span::styled(indent, Style::default().fg(Color::DarkGray)),
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
                        Span::styled(tick_str.clone(), Style::default().fg(Color::DarkGray)),
                        Span::styled(line_buf, Style::default().fg(text_color)),
                    ]));
                } else {
                    let indent = " ".repeat(prefix_len);
                    all_lines.push(Line::from(vec![
                        Span::styled(indent, Style::default().fg(Color::DarkGray)),
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

fn draw_status_bar(frame: &mut Frame, area: Rect, sim: &SimState) {
    // If there's a temporary status message, show it instead of the default bar
    if let Some((ref msg, _)) = sim.status_message {
        let status = Paragraph::new(format!(" {}", msg))
            .style(Style::default().fg(Color::Yellow).bg(Color::DarkGray));
        frame.render_widget(status, area);
        return;
    }

    let alive_count = sim.agents.iter().filter(|a| a.alive).count();
    let save_label = sim
        .save_name
        .as_deref()
        .unwrap_or("unsaved");
    let status_text = format!(
        " {}  |  Tick {}  |  {}  |  Pop: {}  |  [{}]  |  ?=help",
        sim.world.name, sim.world.tick, sim.speed.label(), alive_count, save_label,
    );
    let status = Paragraph::new(status_text)
        .style(Style::default().fg(Color::White).bg(Color::DarkGray));

    frame.render_widget(status, area);
}
