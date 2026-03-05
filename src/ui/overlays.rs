use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::sim::SimState;
use crate::sim::agent::Goal;
use crate::gen::prose_gen;

/// Draw the agent inspect overlay as a centered box over the main layout.
pub fn draw_inspect_overlay(frame: &mut Frame, sim: &SimState, agent_idx: usize) {
    let area = centered_rect(60, 70, frame.area());
    frame.render_widget(Clear, area);

    let agent = &sim.agents[agent_idx];
    let people_name = if agent.people_id < sim.world.peoples.len() {
        &sim.world.peoples[agent.people_id].name
    } else {
        "Unknown"
    };

    let loc_name = prose_gen::nearest_settlement_name(agent.x, agent.y, &sim.world);

    let goal_str = match &agent.current_goal {
        Goal::Wander => "Wandering".to_string(),
        Goal::SeekSettlement(idx) => {
            if *idx < sim.world.settlements.len() {
                format!("Traveling to {}", sim.world.settlements[*idx].name)
            } else {
                "Seeking settlement (unknown)".to_string()
            }
        }
        Goal::Rest(ticks) => format!("Resting ({} ticks remaining)", ticks),
    };

    let alive_str = if agent.alive { "Alive" } else { "Deceased" };
    let age_years = agent.age / 365;

    let mut lines: Vec<Line> = vec![
        Line::from(Span::styled(
            format!(" {} ", agent.name),
            Style::default().fg(Color::White),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled(" People: ", Style::default().fg(Color::DarkGray)),
            Span::styled(people_name, Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::styled(" Status: ", Style::default().fg(Color::DarkGray)),
            Span::styled(alive_str, Style::default().fg(if agent.alive { Color::Green } else { Color::Red })),
        ]),
        Line::from(vec![
            Span::styled(" Age: ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{} years ({} ticks)", age_years, agent.age), Style::default().fg(Color::Gray)),
        ]),
        Line::from(vec![
            Span::styled(" Health: ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{}/100", agent.health), Style::default().fg(Color::Gray)),
        ]),
        Line::from(vec![
            Span::styled(" Location: ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("({}, {}) near {}", agent.x, agent.y, loc_name), Style::default().fg(Color::Gray)),
        ]),
        Line::from(vec![
            Span::styled(" Goal: ", Style::default().fg(Color::DarkGray)),
            Span::styled(&goal_str, Style::default().fg(Color::Yellow)),
        ]),
        Line::from(""),
        Line::from(Span::styled(" DISPOSITION", Style::default().fg(Color::White))),
        Line::from(vec![
            Span::styled("  Risk tolerance:      ", Style::default().fg(Color::DarkGray)),
            Span::styled(disposition_bar(agent.disposition.risk_tolerance), Style::default().fg(Color::Gray)),
        ]),
        Line::from(vec![
            Span::styled("  Ambition:            ", Style::default().fg(Color::DarkGray)),
            Span::styled(disposition_bar(agent.disposition.ambition), Style::default().fg(Color::Gray)),
        ]),
        Line::from(vec![
            Span::styled("  Institutional loyalty:", Style::default().fg(Color::DarkGray)),
            Span::styled(disposition_bar(agent.disposition.institutional_loyalty), Style::default().fg(Color::Gray)),
        ]),
        Line::from(vec![
            Span::styled("  Paranoia:            ", Style::default().fg(Color::DarkGray)),
            Span::styled(disposition_bar(agent.disposition.paranoia), Style::default().fg(Color::Gray)),
        ]),
    ];

    // Chronicle section
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(" CHRONICLE", Style::default().fg(Color::White))));

    // Show events related to this agent from the sim event log
    let agent_events: Vec<&crate::sim::event::Event> = sim.events.iter()
        .filter(|e| e.subject_id == Some(agent.id))
        .collect();
    let recent = if agent_events.len() > 10 {
        &agent_events[agent_events.len() - 10..]
    } else {
        &agent_events
    };

    if recent.is_empty() {
        lines.push(Line::from(Span::styled("  No notable events recorded.", Style::default().fg(Color::DarkGray))));
    } else {
        for event in recent {
            lines.push(Line::from(vec![
                Span::styled(format!("  [{}] ", event.tick), Style::default().fg(Color::DarkGray)),
                Span::styled(&event.description, Style::default().fg(Color::Gray)),
            ]));
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(" ESC to close", Style::default().fg(Color::DarkGray))));

    let block = Block::default()
        .title(format!(" INSPECT: {} ", agent.name))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let widget = Paragraph::new(lines).block(block);
    frame.render_widget(widget, area);
}

/// Draw the agent search overlay (typing a name to find).
pub fn draw_search_overlay(frame: &mut Frame, sim: &SimState, query: &str) {
    let area = centered_rect(50, 50, frame.area());
    frame.render_widget(Clear, area);

    let mut lines: Vec<Line> = vec![
        Line::from(Span::styled(" Search for agent by name:", Style::default().fg(Color::White))),
        Line::from(""),
        Line::from(vec![
            Span::styled(" > ", Style::default().fg(Color::Yellow)),
            Span::styled(query, Style::default().fg(Color::White)),
            Span::styled("_", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(""),
    ];

    // Show matching results
    if query.len() >= 2 {
        let matches = sim.search_agents(query);
        if matches.is_empty() {
            lines.push(Line::from(Span::styled(" No matching agents found.", Style::default().fg(Color::DarkGray))));
        } else {
            let show = matches.len().min(10);
            for &idx in &matches[..show] {
                let a = &sim.agents[idx];
                lines.push(Line::from(Span::styled(
                    format!("  {} (age {}, near {})", a.name, a.age / 365,
                        prose_gen::nearest_settlement_name(a.x, a.y, &sim.world)),
                    Style::default().fg(Color::Gray),
                )));
            }
            if matches.len() > 10 {
                lines.push(Line::from(Span::styled(
                    format!("  ... and {} more", matches.len() - 10),
                    Style::default().fg(Color::DarkGray),
                )));
            }
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(" ENTER to inspect first match", Style::default().fg(Color::DarkGray))));
        }
    } else {
        lines.push(Line::from(Span::styled(" Type at least 2 characters...", Style::default().fg(Color::DarkGray))));
    }

    lines.push(Line::from(Span::styled(" ESC to cancel", Style::default().fg(Color::DarkGray))));

    let block = Block::default()
        .title(" FIND AGENT ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let widget = Paragraph::new(lines).block(block);
    frame.render_widget(widget, area);
}

/// Draw the export menu overlay.
pub fn draw_export_menu(frame: &mut Frame) {
    let area = centered_rect(40, 20, frame.area());
    frame.render_widget(Clear, area);

    let lines = vec![
        Line::from(Span::styled(" EXPORT", Style::default().fg(Color::White))),
        Line::from(""),
        Line::from(Span::styled(" [1] Export Live Log", Style::default().fg(Color::Gray))),
        Line::from(""),
        Line::from(Span::styled(" ESC to cancel", Style::default().fg(Color::DarkGray))),
    ];

    let block = Block::default()
        .title(" EXPORT MENU ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green));

    let widget = Paragraph::new(lines).block(block);
    frame.render_widget(widget, area);
}

/// Draw the export filename input overlay.
pub fn draw_export_input(frame: &mut Frame, input: &str) {
    let area = centered_rect(50, 20, frame.area());
    frame.render_widget(Clear, area);

    let lines = vec![
        Line::from(Span::styled(" Enter filename prefix:", Style::default().fg(Color::White))),
        Line::from(""),
        Line::from(vec![
            Span::styled(" > ", Style::default().fg(Color::Yellow)),
            Span::styled(input, Style::default().fg(Color::White)),
            Span::styled("_", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(""),
        Line::from(Span::styled(" ENTER to export  |  ESC to cancel", Style::default().fg(Color::DarkGray))),
    ];

    let block = Block::default()
        .title(" EXPORT LOG ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green));

    let widget = Paragraph::new(lines).block(block);
    frame.render_widget(widget, area);
}

/// Draw the save name input overlay (Ctrl+S).
pub fn draw_save_name_input(frame: &mut Frame, input: &str) {
    let area = centered_rect(50, 20, frame.area());
    frame.render_widget(Clear, area);

    let lines = vec![
        Line::from(Span::styled(" Enter save name:", Style::default().fg(Color::White))),
        Line::from(""),
        Line::from(vec![
            Span::styled(" > ", Style::default().fg(Color::Yellow)),
            Span::styled(input, Style::default().fg(Color::White)),
            Span::styled("_", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(""),
        Line::from(Span::styled(" ENTER to save  |  ESC to cancel", Style::default().fg(Color::DarkGray))),
    ];

    let block = Block::default()
        .title(" SAVE WORLD ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let widget = Paragraph::new(lines).block(block);
    frame.render_widget(widget, area);
}

/// Simple text bar visualization for a 0.0-1.0 value.
fn disposition_bar(value: f32) -> String {
    let filled = (value * 10.0).round() as usize;
    let empty = 10 - filled.min(10);
    format!("[{}{}] {:.0}%", "#".repeat(filled), ".".repeat(empty), value * 100.0)
}

/// Helper: create a centered Rect with percentage width/height of the parent.
fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    let horiz = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vert[1]);

    horiz[1]
}
