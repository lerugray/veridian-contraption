use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::export::SaveFileInfo;

/// Flavor preset names and descriptions.
pub const FLAVOR_PRESETS: &[(&str, &str)] = &[
    (
        "The Long Bureaucracy",
        "Slow time, dense institutions, Bureaucratic",
    ),
    (
        "The Burning Provinces",
        "Fast time, high volatility, Ominous",
    ),
    (
        "The Deep Taxonomy",
        "High weirdness, ecological focus, Clinical",
    ),
    (
        "The Conspiratorial Age",
        "Secret societies, high cosmological density",
    ),
    (
        "Unguided",
        "Fully random parameters",
    ),
];

/// Draw the main menu screen.
pub fn draw_main_menu(frame: &mut Frame, selected: usize, has_autosave: bool) {
    let area = frame.area();
    frame.render_widget(Clear, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Length(8),  // title block
            Constraint::Length(2),  // spacer
            Constraint::Length(8),  // menu items
            Constraint::Min(1),    // bottom spacer
            Constraint::Length(1), // footer
        ])
        .split(area);

    // Title
    let title_lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "VERIDIAN CONTRAPTION",
            Style::default().fg(Color::Green),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "A World-Simulator of Considerable",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(Span::styled(
            "Density and Dubious Intent",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(""),
    ];
    let title_widget = Paragraph::new(title_lines)
        .alignment(ratatui::layout::Alignment::Center);
    frame.render_widget(title_widget, chunks[1]);

    // Menu options
    let options = [
        ("New World", true),
        ("Continue", has_autosave),
        ("Load World", true),
        ("Quit", true),
    ];

    let menu_lines: Vec<Line> = options
        .iter()
        .enumerate()
        .map(|(i, (label, enabled))| {
            let prefix = if i == selected { " > " } else { "   " };
            let color = if !enabled {
                Color::DarkGray
            } else if i == selected {
                Color::Green
            } else {
                Color::Gray
            };
            Line::from(Span::styled(
                format!("{}{}", prefix, label),
                Style::default().fg(color),
            ))
        })
        .collect();

    // Center the menu horizontally
    let menu_area = centered_horizontal(30, chunks[3]);
    let menu_widget = Paragraph::new(menu_lines);
    frame.render_widget(menu_widget, menu_area);

    // Footer
    let footer = Paragraph::new(Span::styled(
        " Arrow keys to navigate  |  Enter to select",
        Style::default().fg(Color::DarkGray),
    ));
    frame.render_widget(footer, chunks[5]);
}

/// Draw the New World screen.
pub fn draw_new_world(
    frame: &mut Frame,
    selected_preset: usize,
    seed_input: &str,
    editing_seed: bool,
) {
    let area = frame.area();
    frame.render_widget(Clear, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(15),
            Constraint::Length(3),  // title
            Constraint::Length(1),  // spacer
            Constraint::Length(12), // preset list
            Constraint::Length(1),  // spacer
            Constraint::Length(4),  // seed input
            Constraint::Min(1),    // bottom spacer
            Constraint::Length(1), // footer
        ])
        .split(area);

    // Title
    let title = Paragraph::new(Span::styled(
        "NEW WORLD",
        Style::default().fg(Color::Green),
    ))
    .alignment(ratatui::layout::Alignment::Center);
    frame.render_widget(title, chunks[1]);

    // Preset list — show description on a second indented line under each name
    // Use up to 60 columns, but cap at available width
    let content_width = 60.min(area.width.saturating_sub(4));
    let content_area = centered_horizontal(content_width, chunks[3]);

    let mut preset_lines: Vec<Line> = vec![
        Line::from(Span::styled(
            "Choose a flavor preset:",
            Style::default().fg(Color::White),
        )),
        Line::from(""),
    ];

    for (i, (name, desc)) in FLAVOR_PRESETS.iter().enumerate() {
        let is_selected = !editing_seed && i == selected_preset;
        let prefix = if is_selected { " > " } else { "   " };
        let name_color = if is_selected { Color::Green } else { Color::Gray };
        preset_lines.push(Line::from(Span::styled(
            format!("{}{}", prefix, name),
            Style::default().fg(name_color),
        )));
        preset_lines.push(Line::from(Span::styled(
            format!("     {}", desc),
            Style::default().fg(Color::DarkGray),
        )));
    }

    let presets_widget = Paragraph::new(preset_lines);
    frame.render_widget(presets_widget, content_area);

    // Seed input
    let seed_area = centered_horizontal(content_width, chunks[5]);
    let cursor_char = if editing_seed { "_" } else { "" };
    let seed_border_color = if editing_seed {
        Color::Yellow
    } else {
        Color::DarkGray
    };

    let seed_lines = vec![
        Line::from(vec![
            Span::styled(" Seed: ", Style::default().fg(Color::White)),
            Span::styled(seed_input, Style::default().fg(Color::Yellow)),
            Span::styled(cursor_char, Style::default().fg(Color::DarkGray)),
            if seed_input.is_empty() && editing_seed {
                Span::styled(
                    " (blank = random)",
                    Style::default().fg(Color::DarkGray),
                )
            } else {
                Span::raw("")
            },
        ]),
    ];

    let seed_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(seed_border_color));
    let seed_widget = Paragraph::new(seed_lines).block(seed_block);
    frame.render_widget(seed_widget, seed_area);

    // Footer
    let footer_text = if editing_seed {
        " Enter=generate  |  ESC=back"
    } else {
        " Up/Down=preset  Tab=seed  Enter=generate  ESC=cancel"
    };
    let footer = Paragraph::new(Span::styled(
        footer_text,
        Style::default().fg(Color::DarkGray),
    ));
    frame.render_widget(footer, chunks[7]);
}

/// Draw the Load World screen.
pub fn draw_load_world(frame: &mut Frame, saves: &[SaveFileInfo], selected: usize) {
    let area = frame.area();
    frame.render_widget(Clear, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(15),
            Constraint::Length(3),  // title
            Constraint::Length(1),  // spacer
            Constraint::Min(5),    // save list
            Constraint::Length(1), // footer
        ])
        .split(area);

    let title = Paragraph::new(Span::styled(
        "LOAD WORLD",
        Style::default().fg(Color::Green),
    ))
    .alignment(ratatui::layout::Alignment::Center);
    frame.render_widget(title, chunks[1]);

    let content_area = centered_horizontal(50, chunks[3]);

    if saves.is_empty() {
        let empty = Paragraph::new(vec![
            Line::from(""),
            Line::from(Span::styled(
                "  No saved worlds found.",
                Style::default().fg(Color::DarkGray),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "  Press ESC to return.",
                Style::default().fg(Color::DarkGray),
            )),
        ]);
        frame.render_widget(empty, content_area);
    } else {
        let mut lines: Vec<Line> = Vec::new();
        for (i, save) in saves.iter().enumerate() {
            let prefix = if i == selected { " > " } else { "   " };
            let color = if i == selected {
                Color::Green
            } else {
                Color::Gray
            };
            lines.push(Line::from(Span::styled(
                format!("{}{}", prefix, save.name),
                Style::default().fg(color),
            )));
        }

        let block = Block::default()
            .title(" Saved Worlds ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let list_widget = Paragraph::new(lines).block(block);
        frame.render_widget(list_widget, content_area);
    }

    let footer = Paragraph::new(Span::styled(
        " Up/Down to select  |  Enter to load  |  ESC to cancel",
        Style::default().fg(Color::DarkGray),
    ));
    frame.render_widget(footer, chunks[4]);
}

/// Draw the "Generating world..." screen.
pub fn draw_generating(frame: &mut Frame) {
    let area = frame.area();
    frame.render_widget(Clear, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(area);

    let msg = Paragraph::new(vec![
        Line::from(Span::styled(
            "Generating world...",
            Style::default().fg(Color::Green),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "A ledger is being prepared.",
            Style::default().fg(Color::DarkGray),
        )),
    ])
    .alignment(ratatui::layout::Alignment::Center);
    frame.render_widget(msg, chunks[1]);
}

/// Center content horizontally within a given area.
fn centered_horizontal(width: u16, area: Rect) -> Rect {
    if area.width <= width {
        return area;
    }
    let pad = (area.width - width) / 2;
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(pad),
            Constraint::Length(width),
            Constraint::Min(0),
        ])
        .split(area)[1]
}
