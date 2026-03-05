use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::sim::world::World;

/// Draw the main two-panel layout: world map (left) and live log (right).
pub fn draw_main_layout(frame: &mut Frame, world: &World) {
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

    draw_map_panel(frame, panels[0], world);
    draw_log_panel(frame, panels[1]);
    draw_status_bar(frame, chunks[1], world);
}

fn draw_map_panel(frame: &mut Frame, area: Rect, world: &World) {
    let block = Block::default()
        .title(format!(" {} ", world.name))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    let rendered = world.render_map();

    // Convert rendered map to ratatui Lines.
    // Each row becomes a Line of individually-colored Spans.
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

fn draw_log_panel(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title(" LIVE LOG ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    let placeholder = Paragraph::new("No events yet.")
        .style(Style::default().fg(Color::Gray))
        .block(block);

    frame.render_widget(placeholder, area);
}

fn draw_status_bar(frame: &mut Frame, area: Rect, world: &World) {
    let status_text = format!(
        " {}  |  Tick {}  |  Paused  |  q = quit  |  ? = help",
        world.name, world.tick,
    );
    let status = Paragraph::new(status_text)
        .style(Style::default().fg(Color::White).bg(Color::DarkGray));

    frame.render_widget(status, area);
}
