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
            Constraint::Percentage(60), // world map gets more space
            Constraint::Percentage(40), // log pane
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

    // Convert rendered map to ratatui Lines.
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
    let block = Block::default()
        .title(" LIVE LOG ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    // Show the most recent log entries that fit in the panel
    let inner_height = area.height.saturating_sub(2) as usize; // subtract borders
    let start = sim.log.len().saturating_sub(inner_height);
    let visible: Vec<Line> = sim.log[start..]
        .iter()
        .map(|entry| Line::from(Span::styled(entry.as_str(), Style::default().fg(Color::Gray))))
        .collect();

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
