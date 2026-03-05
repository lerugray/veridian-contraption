use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::sim::SimState;
use crate::sim::world::{MAP_HEIGHT, MAP_WIDTH};

/// Draw the main two-panel layout: world map (left) and live log (right).
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
    draw_log_panel(frame, panels[1], sim);
    draw_status_bar(frame, chunks[1], sim);
}

fn draw_map_panel(frame: &mut Frame, area: Rect, sim: &SimState) {
    let block = Block::default()
        .title(format!(" {} ", sim.world.name))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    let mut rendered = sim.world.render_map();

    // Overlay living agents on the map
    for agent in &sim.agents {
        if !agent.alive {
            continue;
        }
        let ax = agent.x as usize;
        let ay = agent.y as usize;
        if ay < MAP_HEIGHT && ax < MAP_WIDTH {
            rendered[ay][ax] = ('@', Color::Magenta);
        }
    }

    let lines: Vec<Line> = rendered
        .iter()
        .map(|row| {
            let spans: Vec<Span> = row
                .iter()
                .map(|&(ch, color)| {
                    Span::styled(ch.to_string(), Style::default().fg(color))
                })
                .collect();
            Line::from(spans)
        })
        .collect();

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

    // Build all formatted lines (events may wrap to multiple display lines).
    let mut all_lines: Vec<Line> = Vec::new();

    for event in &sim.events {
        // Format: [tick] description
        // The tick prefix is dim, the description is normal gray.
        let tick_str = format!("[{}] ", event.tick);
        let desc = &event.description;

        // Word-wrap the description manually so we can color the tick prefix
        // separately from the body text.
        let prefix_len = tick_str.len();
        let body_width = inner_width.saturating_sub(prefix_len);

        if body_width < 10 {
            // Panel too narrow for wrapping; just truncate
            all_lines.push(Line::from(vec![
                Span::styled(tick_str.clone(), Style::default().fg(Color::DarkGray)),
                Span::styled(desc.clone(), Style::default().fg(Color::Gray)),
            ]));
        } else {
            // First line gets the tick prefix
            let words: Vec<&str> = desc.split_whitespace().collect();
            let mut line_buf = String::new();
            let mut first = true;

            for word in &words {
                let space = if line_buf.is_empty() { 0 } else { 1 };
                let limit = if first { body_width } else { inner_width };

                if line_buf.len() + space + word.len() > limit && !line_buf.is_empty() {
                    // Emit this line
                    if first {
                        all_lines.push(Line::from(vec![
                            Span::styled(tick_str.clone(), Style::default().fg(Color::DarkGray)),
                            Span::styled(line_buf.clone(), Style::default().fg(Color::Gray)),
                        ]));
                        first = false;
                    } else {
                        // Continuation lines indented with spaces matching tick prefix
                        let indent = " ".repeat(prefix_len);
                        all_lines.push(Line::from(vec![
                            Span::styled(indent, Style::default().fg(Color::DarkGray)),
                            Span::styled(line_buf.clone(), Style::default().fg(Color::Gray)),
                        ]));
                    }
                    line_buf.clear();
                }

                if !line_buf.is_empty() {
                    line_buf.push(' ');
                }
                line_buf.push_str(word);
            }

            // Emit remaining text
            if !line_buf.is_empty() {
                if first {
                    all_lines.push(Line::from(vec![
                        Span::styled(tick_str.clone(), Style::default().fg(Color::DarkGray)),
                        Span::styled(line_buf, Style::default().fg(Color::Gray)),
                    ]));
                } else {
                    let indent = " ".repeat(prefix_len);
                    all_lines.push(Line::from(vec![
                        Span::styled(indent, Style::default().fg(Color::DarkGray)),
                        Span::styled(line_buf, Style::default().fg(Color::Gray)),
                    ]));
                }
            }
        }
    }

    // Apply scroll offset: log_scroll=0 means show the most recent lines.
    let total = all_lines.len();
    let end = total.saturating_sub(sim.log_scroll);
    let start = end.saturating_sub(inner_height);
    let visible: Vec<Line> = all_lines[start..end].to_vec();

    let log_widget = Paragraph::new(visible).block(block);
    frame.render_widget(log_widget, area);
}

fn draw_status_bar(frame: &mut Frame, area: Rect, sim: &SimState) {
    let alive_count = sim.agents.iter().filter(|a| a.alive).count();
    let status_text = format!(
        " {}  |  Tick {}  |  {}  |  Pop: {}  |  SPACE=pause  .=step  1/5/2=speed  q=quit",
        sim.world.name, sim.world.tick, sim.speed.label(), alive_count,
    );
    let status = Paragraph::new(status_text)
        .style(Style::default().fg(Color::White).bg(Color::DarkGray));

    frame.render_widget(status, area);
}
