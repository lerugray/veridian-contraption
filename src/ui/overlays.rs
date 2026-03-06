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
        Goal::JoinInstitution(id) => {
            let name = sim.institution_name(*id).unwrap_or("unknown body");
            format!("Seeking to join {}", name)
        }
        Goal::AdvanceInInstitution(id) => {
            let name = sim.institution_name(*id).unwrap_or("unknown body");
            format!("Advancing in {}", name)
        }
        Goal::FoundInstitution => "Planning to found an institution".to_string(),
    };

    let alive_str = if agent.alive { "Alive" } else { "Deceased" };
    let age_years = agent.age / 365;

    let mut lines: Vec<Line> = vec![
        Line::from(Span::styled(
            format!(" {} ", agent.display_name()),
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
    ];

    // Epithets section (if any)
    if !agent.epithets.is_empty() {
        lines.push(Line::from(Span::styled(" EPITHETS", Style::default().fg(Color::White))));
        for (i, ep) in agent.epithets.iter().rev().enumerate() {
            let label = if i == 0 { "(current)" } else { "" };
            lines.push(Line::from(Span::styled(
                format!("  {} {}", ep, label),
                Style::default().fg(if i == 0 { Color::Yellow } else { Color::DarkGray }),
            )));
        }
    }

    // Institutional affiliations
    if !agent.institution_ids.is_empty() {
        lines.push(Line::from(Span::styled(" AFFILIATIONS", Style::default().fg(Color::White))));
        for &inst_id in &agent.institution_ids {
            if let Some(inst) = sim.institutions.iter().find(|i| i.id == inst_id) {
                lines.push(Line::from(vec![
                    Span::styled("  ", Style::default()),
                    Span::styled(&inst.name, Style::default().fg(Color::Cyan)),
                    Span::styled(format!(" ({})", inst.kind.label()), Style::default().fg(Color::DarkGray)),
                ]));
            }
        }
    } else {
        lines.push(Line::from(Span::styled(" AFFILIATIONS: None", Style::default().fg(Color::DarkGray))));
    }

    lines.extend(vec![
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
    ]);

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

/// Draw the agent search overlay with selectable match list.
pub fn draw_search_overlay(frame: &mut Frame, sim: &SimState, query: &str, selected: usize) {
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

    // Show matching results with selection highlight
    if query.len() >= 2 {
        let matches = sim.search_agents(query);
        if matches.is_empty() {
            lines.push(Line::from(Span::styled(" No matching agents found.", Style::default().fg(Color::DarkGray))));
        } else {
            let show = matches.len().min(15);
            for (i, &idx) in matches[..show].iter().enumerate() {
                let a = &sim.agents[idx];
                let is_selected = i == selected;
                let prefix = if is_selected { " > " } else { "   " };
                let color = if is_selected { Color::Green } else { Color::Gray };
                lines.push(Line::from(Span::styled(
                    format!("{}{} (age {}, near {})", prefix, a.name, a.age / 365,
                        prose_gen::nearest_settlement_name(a.x, a.y, &sim.world)),
                    Style::default().fg(color),
                )));
            }
            if matches.len() > 15 {
                lines.push(Line::from(Span::styled(
                    format!("  ... and {} more", matches.len() - 15),
                    Style::default().fg(Color::DarkGray),
                )));
            }
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(" Up/Down=select  ENTER=inspect", Style::default().fg(Color::DarkGray))));
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

/// Draw the agent list overlay (Tab key — browsable list of all living agents).
pub fn draw_agent_list(frame: &mut Frame, sim: &SimState, selected: usize) {
    let area = centered_rect(80, 70, frame.area());
    frame.render_widget(Clear, area);

    let living = sim.living_agent_indices();
    let inner_height = area.height.saturating_sub(5) as usize; // borders + header + footer

    let mut lines: Vec<Line> = vec![
        Line::from(Span::styled(
            format!(" {} living agents", living.len()),
            Style::default().fg(Color::White),
        )),
        Line::from(""),
    ];

    if living.is_empty() {
        lines.push(Line::from(Span::styled(" No living agents.", Style::default().fg(Color::DarkGray))));
    } else {
        // Derive scroll window from selected position
        let visible_count = inner_height.saturating_sub(4); // account for header/footer lines
        let scroll_start = if selected >= visible_count {
            selected - visible_count + 1
        } else {
            0
        };
        let scroll_end = (scroll_start + visible_count).min(living.len());

        for i in scroll_start..scroll_end {
            let idx = living[i];
            let a = &sim.agents[idx];
            let is_selected = i == selected;
            let prefix = if is_selected { " > " } else { "   " };
            let color = if is_selected { Color::Green } else { Color::Gray };
            let loc = prose_gen::nearest_settlement_name(a.x, a.y, &sim.world);

            // Show institution affiliation if any
            let affil = if a.institution_ids.is_empty() {
                String::new()
            } else {
                let names: Vec<&str> = a.institution_ids.iter()
                    .filter_map(|&id| sim.institutions.iter().find(|i| i.id == id).map(|i| i.name.as_str()))
                    .collect();
                if names.is_empty() { String::new() } else { format!(" [{}]", names.join(", ")) }
            };

            // Show epithet if any
            let epithet = a.epithets.last().map(|e| format!(" {}", e)).unwrap_or_default();

            lines.push(Line::from(Span::styled(
                format!("{}{}{} — age {}, near {}{}", prefix, a.name, epithet, a.age / 365, loc, affil),
                Style::default().fg(color),
            )));
        }

        if scroll_end < living.len() {
            lines.push(Line::from(Span::styled(
                format!("  ... {} more below", living.len() - scroll_end),
                Style::default().fg(Color::DarkGray),
            )));
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(" Up/Down=browse  ENTER=inspect  ESC=close", Style::default().fg(Color::DarkGray))));

    let block = Block::default()
        .title(" AGENTS ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Magenta));

    let widget = Paragraph::new(lines).block(block);
    frame.render_widget(widget, area);
}

/// Draw the faction list overlay (f key — browsable list of all institutions).
pub fn draw_faction_list(frame: &mut Frame, sim: &SimState, selected: usize) {
    let area = centered_rect(65, 75, frame.area());
    frame.render_widget(Clear, area);

    let living = sim.living_institution_indices();
    let inner_height = area.height.saturating_sub(5) as usize;

    let mut lines: Vec<Line> = vec![
        Line::from(Span::styled(
            format!(" {} active institutions", living.len()),
            Style::default().fg(Color::White),
        )),
        Line::from(""),
    ];

    if living.is_empty() {
        lines.push(Line::from(Span::styled(" No active institutions.", Style::default().fg(Color::DarkGray))));
    } else {
        let visible_count = inner_height.saturating_sub(4);
        let scroll_start = if selected >= visible_count {
            selected - visible_count + 1
        } else {
            0
        };
        let scroll_end = (scroll_start + visible_count).min(living.len());

        for i in scroll_start..scroll_end {
            let idx = living[i];
            let inst = &sim.institutions[idx];
            let is_selected = i == selected;
            let prefix = if is_selected { " > " } else { "   " };
            let name_color = if is_selected { Color::Green } else { Color::Cyan };

            // Institution name and kind
            lines.push(Line::from(vec![
                Span::styled(prefix, Style::default()),
                Span::styled(&inst.name, Style::default().fg(name_color)),
            ]));

            // Details line: kind, power, members
            let detail = format!(
                "     {} | Power: {} | Members: {} | {}",
                inst.kind.label(),
                inst.power,
                inst.member_ids.len(),
                inst.summary(),
            );
            lines.push(Line::from(Span::styled(
                detail,
                Style::default().fg(Color::DarkGray),
            )));

            // Show relationships if selected
            if is_selected && !inst.relationships.is_empty() {
                for (&other_id, rel) in &inst.relationships {
                    if let Some(other) = sim.institutions.iter().find(|i| i.id == other_id) {
                        lines.push(Line::from(Span::styled(
                            format!("       {} — {}", other.name, rel.label()),
                            Style::default().fg(Color::DarkGray),
                        )));
                    }
                }
            }
        }

        if scroll_end < living.len() {
            lines.push(Line::from(Span::styled(
                format!("  ... {} more below", living.len() - scroll_end),
                Style::default().fg(Color::DarkGray),
            )));
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(" Up/Down=browse  ESC=close", Style::default().fg(Color::DarkGray))));

    let block = Block::default()
        .title(" FACTIONS ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let widget = Paragraph::new(lines).block(block);
    frame.render_widget(widget, area);
}

/// Draw the quit confirmation overlay (Q key — return to menu prompt).
pub fn draw_quit_confirm(frame: &mut Frame, selected: usize) {
    let area = centered_rect(40, 25, frame.area());
    frame.render_widget(Clear, area);

    let options = ["Save and return to menu", "Return without saving", "Cancel"];
    let mut lines: Vec<Line> = vec![
        Line::from(Span::styled(" Return to main menu?", Style::default().fg(Color::White))),
        Line::from(""),
    ];

    for (i, label) in options.iter().enumerate() {
        let prefix = if i == selected { " > " } else { "   " };
        let color = if i == selected { Color::Green } else { Color::Gray };
        lines.push(Line::from(Span::styled(
            format!("{}{}", prefix, label),
            Style::default().fg(color),
        )));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(" Up/Down + Enter to select", Style::default().fg(Color::DarkGray))));

    let block = Block::default()
        .title(" QUIT ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red));

    let widget = Paragraph::new(lines).block(block);
    frame.render_widget(widget, area);
}

/// Draw the follow mode selection overlay (choose agent or institution).
pub fn draw_follow_select(frame: &mut Frame, selected: usize) {
    let area = centered_rect(40, 20, frame.area());
    frame.render_widget(Clear, area);

    let options = ["Follow an Agent", "Follow an Institution"];
    let mut lines: Vec<Line> = vec![
        Line::from(Span::styled(" Choose what to follow:", Style::default().fg(Color::White))),
        Line::from(""),
    ];

    for (i, label) in options.iter().enumerate() {
        let prefix = if i == selected { " > " } else { "   " };
        let color = if i == selected { Color::Green } else { Color::Gray };
        lines.push(Line::from(Span::styled(
            format!("{}{}", prefix, label),
            Style::default().fg(color),
        )));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(" Up/Down + Enter | ESC=cancel", Style::default().fg(Color::DarkGray))));

    let block = Block::default()
        .title(" FOLLOW ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::LightRed));

    let widget = Paragraph::new(lines).block(block);
    frame.render_widget(widget, area);
}

/// Draw the follow agent picker overlay.
pub fn draw_follow_agent_pick(frame: &mut Frame, sim: &SimState, selected: usize) {
    let area = centered_rect(70, 60, frame.area());
    frame.render_widget(Clear, area);

    let living = sim.living_agent_indices();
    let inner_height = area.height.saturating_sub(5) as usize;

    let mut lines: Vec<Line> = vec![
        Line::from(Span::styled(
            format!(" Select agent to follow ({} living)", living.len()),
            Style::default().fg(Color::White),
        )),
        Line::from(""),
    ];

    if !living.is_empty() {
        let visible_count = inner_height.saturating_sub(4);
        let scroll_start = if selected >= visible_count { selected - visible_count + 1 } else { 0 };
        let scroll_end = (scroll_start + visible_count).min(living.len());

        for i in scroll_start..scroll_end {
            let idx = living[i];
            let a = &sim.agents[idx];
            let is_selected = i == selected;
            let prefix = if is_selected { " > " } else { "   " };
            let color = if is_selected { Color::Green } else { Color::Gray };
            let loc = prose_gen::nearest_settlement_name(a.x, a.y, &sim.world);
            lines.push(Line::from(Span::styled(
                format!("{}{} — near {}", prefix, a.display_name(), loc),
                Style::default().fg(color),
            )));
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(" Up/Down + Enter | ESC=cancel", Style::default().fg(Color::DarkGray))));

    let block = Block::default()
        .title(" FOLLOW AGENT ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::LightRed));

    let widget = Paragraph::new(lines).block(block);
    frame.render_widget(widget, area);
}

/// Draw the follow institution picker overlay.
pub fn draw_follow_institution_pick(frame: &mut Frame, sim: &SimState, selected: usize) {
    let area = centered_rect(65, 50, frame.area());
    frame.render_widget(Clear, area);

    let living = sim.living_institution_indices();
    let inner_height = area.height.saturating_sub(5) as usize;

    let mut lines: Vec<Line> = vec![
        Line::from(Span::styled(
            format!(" Select institution to follow ({} active)", living.len()),
            Style::default().fg(Color::White),
        )),
        Line::from(""),
    ];

    if !living.is_empty() {
        let visible_count = inner_height.saturating_sub(4);
        let scroll_start = if selected >= visible_count { selected - visible_count + 1 } else { 0 };
        let scroll_end = (scroll_start + visible_count).min(living.len());

        for i in scroll_start..scroll_end {
            let idx = living[i];
            let inst = &sim.institutions[idx];
            let is_selected = i == selected;
            let prefix = if is_selected { " > " } else { "   " };
            let color = if is_selected { Color::Green } else { Color::Gray };
            lines.push(Line::from(Span::styled(
                format!("{}{} ({})", prefix, inst.name, inst.kind.label()),
                Style::default().fg(color),
            )));
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(" Up/Down + Enter | ESC=cancel", Style::default().fg(Color::DarkGray))));

    let block = Block::default()
        .title(" FOLLOW INSTITUTION ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::LightRed));

    let widget = Paragraph::new(lines).block(block);
    frame.render_widget(widget, area);
}

/// Draw the site list overlay (s key — browsable list of all sites).
pub fn draw_site_list(frame: &mut Frame, sim: &SimState, selected: usize) {
    let area = centered_rect(70, 65, frame.area());
    frame.render_widget(Clear, area);

    let inner_height = area.height.saturating_sub(5) as usize;

    let mut lines: Vec<Line> = vec![
        Line::from(Span::styled(
            format!(" {} sites discovered", sim.sites.len()),
            Style::default().fg(Color::White),
        )),
        Line::from(""),
    ];

    if sim.sites.is_empty() {
        lines.push(Line::from(Span::styled(" No sites discovered.", Style::default().fg(Color::DarkGray))));
    } else {
        // Each entry takes 2 lines (name + detail), selected takes 3 (+ origin).
        // Budget lines for the scrollable area.
        let available_lines = inner_height.saturating_sub(4); // header + footer
        // Estimate ~3 lines per entry to be safe
        let entries_per_page = (available_lines / 3).max(1);

        let scroll_start = if selected >= entries_per_page {
            selected - entries_per_page + 1
        } else {
            0
        };

        // Render entries, tracking how many display lines we've used
        let mut lines_used: usize = 0;
        let mut last_shown = scroll_start;

        for i in scroll_start..sim.sites.len() {
            let site = &sim.sites[i];
            let is_selected = i == selected;
            let entry_lines = if is_selected { 3 } else { 2 };

            // Stop if we'd overflow the available space
            if lines_used + entry_lines > available_lines {
                break;
            }

            let prefix = if is_selected { " > " } else { "   " };
            let name_color = if is_selected { Color::Green } else { site.kind.map_color() };

            lines.push(Line::from(vec![
                Span::styled(prefix, Style::default()),
                Span::styled(&site.name, Style::default().fg(name_color)),
            ]));

            let faction_label = if let Some(fid) = site.controlling_faction {
                sim.institutions.iter()
                    .find(|inst| inst.id == fid)
                    .map(|inst| format!("Controlled by {}", inst.name))
                    .unwrap_or_else(|| "Unclaimed".to_string())
            } else {
                "Unclaimed".to_string()
            };

            let detail = format!(
                "     {} | ({},{}) | {} floor{} | {}",
                site.kind.label(),
                site.grid_x, site.grid_y,
                site.floors.len(),
                if site.floors.len() == 1 { "" } else { "s" },
                faction_label,
            );
            lines.push(Line::from(Span::styled(
                detail,
                Style::default().fg(Color::DarkGray),
            )));

            if is_selected {
                lines.push(Line::from(Span::styled(
                    format!("     {}", site.origin),
                    Style::default().fg(Color::DarkGray),
                )));
            }

            lines_used += entry_lines;
            last_shown = i + 1;
        }

        if last_shown < sim.sites.len() {
            lines.push(Line::from(Span::styled(
                format!("  ... {} more below", sim.sites.len() - last_shown),
                Style::default().fg(Color::DarkGray),
            )));
        }
        if scroll_start > 0 {
            // Insert a "more above" hint after the header
            lines.insert(2, Line::from(Span::styled(
                format!("  ... {} above", scroll_start),
                Style::default().fg(Color::DarkGray),
            )));
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(" Up/Down=browse  ENTER=view site  ESC=close", Style::default().fg(Color::DarkGray))));

    let block = Block::default()
        .title(" SITES ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red));

    let widget = Paragraph::new(lines).block(block);
    frame.render_widget(widget, area);
}

/// Draw the help overlay showing all keybindings.
pub fn draw_help(frame: &mut Frame) {
    let area = centered_rect(55, 75, frame.area());
    frame.render_widget(Clear, area);

    let key_style = Style::default().fg(Color::Yellow);
    let desc_style = Style::default().fg(Color::Gray);
    let header_style = Style::default().fg(Color::White);

    let lines: Vec<Line> = vec![
        Line::from(Span::styled(" KEYBINDINGS", header_style)),
        Line::from(""),
        Line::from(Span::styled(" SIMULATION", header_style)),
        Line::from(vec![Span::styled("   SPACE    ", key_style), Span::styled("Pause / unpause", desc_style)]),
        Line::from(vec![Span::styled("   .        ", key_style), Span::styled("Step one tick (while paused)", desc_style)]),
        Line::from(vec![Span::styled("   1        ", key_style), Span::styled("Speed 1x", desc_style)]),
        Line::from(vec![Span::styled("   5        ", key_style), Span::styled("Speed 5x", desc_style)]),
        Line::from(vec![Span::styled("   2        ", key_style), Span::styled("Speed 20x", desc_style)]),
        Line::from(""),
        Line::from(Span::styled(" NAVIGATION", header_style)),
        Line::from(vec![Span::styled("   i        ", key_style), Span::styled("Find agent by name", desc_style)]),
        Line::from(vec![Span::styled("   Tab      ", key_style), Span::styled("Browse agent list", desc_style)]),
        Line::from(vec![Span::styled("   f        ", key_style), Span::styled("Follow agent or institution", desc_style)]),
        Line::from(vec![Span::styled("   F        ", key_style), Span::styled("View faction list", desc_style)]),
        Line::from(vec![Span::styled("   s        ", key_style), Span::styled("Browse sites (dungeons, ruins, etc.)", desc_style)]),
        Line::from(vec![Span::styled("   PgUp/Dn  ", key_style), Span::styled("Scroll log", desc_style)]),
        Line::from(""),
        Line::from(Span::styled(" EXPORT & SAVE", header_style)),
        Line::from(vec![Span::styled("   e        ", key_style), Span::styled("Export menu (log, factions, chronicles)", desc_style)]),
        Line::from(vec![Span::styled("   Ctrl+S   ", key_style), Span::styled("Save world", desc_style)]),
        Line::from(""),
        Line::from(Span::styled(" OTHER", header_style)),
        Line::from(vec![Span::styled("   q        ", key_style), Span::styled("Return to main menu", desc_style)]),
        Line::from(vec![Span::styled("   ?        ", key_style), Span::styled("This help screen", desc_style)]),
        Line::from(""),
        Line::from(Span::styled(" ESC to close", Style::default().fg(Color::DarkGray))),
    ];

    let block = Block::default()
        .title(" HELP ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));

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
        Line::from(Span::styled(" [2] Export Faction Record", Style::default().fg(Color::Gray))),
        Line::from(Span::styled(" [3] Export Character Chronicles", Style::default().fg(Color::Gray))),
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
