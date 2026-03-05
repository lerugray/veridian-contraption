use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Draw the main two-panel layout: world map (left) and live log (right).
pub fn draw_main_layout(frame: &mut Frame) {
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

    draw_map_panel(frame, panels[0]);
    draw_log_panel(frame, panels[1]);
    draw_status_bar(frame, chunks[1]);
}

fn draw_map_panel(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title(" WORLD MAP ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    let placeholder = Paragraph::new("Awaiting world generation...")
        .style(Style::default().fg(Color::Gray))
        .block(block);

    frame.render_widget(placeholder, area);
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

fn draw_status_bar(frame: &mut Frame, area: Rect) {
    let status = Paragraph::new(" VERIDIAN CONTRAPTION  |  Paused  |  q = quit  |  ? = help")
        .style(Style::default().fg(Color::White).bg(Color::DarkGray));

    frame.render_widget(status, area);
}
