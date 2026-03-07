use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::sim::SimState;
use crate::sim::agent::Goal;
use crate::sim::eschaton::{ESCHATON_COOLDOWN, TENSION_THRESHOLD, COSMO_THRESHOLD};
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
        Goal::SeekSite(idx) => {
            if *idx < sim.sites.len() {
                format!("Heading to {}", sim.sites[*idx].name)
            } else {
                "Seeking site (unknown)".to_string()
            }
        }
        Goal::ExploreSite(idx, ticks) => {
            if *idx < sim.sites.len() {
                format!("Exploring {} ({} ticks remaining)", sim.sites[*idx].name, ticks)
            } else {
                format!("Exploring site ({} ticks remaining)", ticks)
            }
        }
        Goal::AcquireArtifact(art_id, site_idx) => {
            let art_name = sim.artifacts.iter().find(|a| a.id == *art_id)
                .map(|a| a.name.as_str()).unwrap_or("unknown artifact");
            let site_name = sim.sites.get(*site_idx)
                .map(|s| s.name.as_str()).unwrap_or("unknown site");
            format!("Seeking {} in {}", art_name, site_name)
        }
        Goal::ReturnArtifact(art_id, sett_idx) => {
            let art_name = sim.artifacts.iter().find(|a| a.id == *art_id)
                .map(|a| a.name.as_str()).unwrap_or("unknown artifact");
            let sett_name = sim.world.settlements.get(*sett_idx)
                .map(|s| s.name.as_str()).unwrap_or("unknown settlement");
            format!("Returning {} to {}", art_name, sett_name)
        }
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

    // Adventurer flag
    if agent.is_adventurer {
        lines.push(Line::from(Span::styled(" ADVENTURER", Style::default().fg(Color::LightYellow))));
    }

    // Held artifacts
    if !agent.held_artifacts.is_empty() {
        lines.push(Line::from(Span::styled(" HELD ARTIFACTS", Style::default().fg(Color::White))));
        for &art_id in &agent.held_artifacts {
            if let Some(art) = sim.artifacts.iter().find(|a| a.id == art_id) {
                lines.push(Line::from(vec![
                    Span::styled("  ", Style::default()),
                    Span::styled(&art.name, Style::default().fg(Color::LightYellow)),
                    Span::styled(format!(" ({})", art.kind.label()), Style::default().fg(Color::DarkGray)),
                ]));
            }
        }
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
    lines.push(Line::from(Span::styled(" Up/Down=browse  Enter=details  ESC=close", Style::default().fg(Color::DarkGray))));

    let block = Block::default()
        .title(" FACTIONS ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let widget = Paragraph::new(lines).block(block);
    frame.render_widget(widget, area);
}

/// Draw the faction detail dossier overlay (Enter from faction list).
pub fn draw_faction_detail(frame: &mut Frame, sim: &SimState, inst_idx: usize, scroll: usize) {
    let area = centered_rect(70, 85, frame.area());
    frame.render_widget(Clear, area);

    let inst = if inst_idx < sim.institutions.len() {
        &sim.institutions[inst_idx]
    } else {
        return;
    };

    let inner_height = area.height.saturating_sub(4) as usize; // border + title + footer
    // Available text width inside borders (2 border cols)
    let max_w = area.width.saturating_sub(2) as usize;

    let mut lines: Vec<Line> = Vec::new();

    // -- Header: name and type --
    lines.push(Line::from(Span::styled(
        truncate_str(&format!(" {} ", inst.name), max_w),
        Style::default().fg(Color::Cyan),
    )));
    lines.push(Line::from(Span::styled(
        truncate_str(
            &format!(" Type: {}  |  Founded: tick {}", inst.kind.label(), inst.founded_tick),
            max_w,
        ),
        Style::default().fg(Color::DarkGray),
    )));
    lines.push(Line::from(""));

    // -- Charter / Doctrine --
    lines.push(Line::from(Span::styled(" CHARTER", Style::default().fg(Color::Yellow))));
    for wl in wrap_text(&format!("  {}", inst.charter), max_w) {
        lines.push(Line::from(Span::styled(wl, Style::default().fg(Color::Gray))));
    }
    if inst.charter != inst.actual_function {
        for wl in wrap_text(&format!("  (Actual function: {})", inst.actual_function), max_w) {
            lines.push(Line::from(Span::styled(wl, Style::default().fg(Color::DarkGray))));
        }
    }
    if !inst.doctrine.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(" DOCTRINE", Style::default().fg(Color::Yellow))));
        for d in &inst.doctrine {
            for wl in wrap_text(&format!("  - {}", d), max_w) {
                lines.push(Line::from(Span::styled(wl, Style::default().fg(Color::Gray))));
            }
        }
    }
    lines.push(Line::from(""));

    // -- Institutional Health --
    lines.push(Line::from(Span::styled(" INSTITUTIONAL HEALTH", Style::default().fg(Color::Yellow))));
    let member_count = inst.member_ids.iter()
        .filter(|id| sim.agents.iter().any(|a| a.id == **id && a.alive))
        .count();
    let health_label = if member_count == 0 {
        ("Defunct", Color::Red)
    } else if inst.power >= 80 && member_count >= 8 {
        ("Ascendant", Color::Green)
    } else if inst.power >= 50 || member_count >= 5 {
        ("Stable", Color::White)
    } else if inst.power >= 20 || member_count >= 2 {
        ("Declining", Color::Yellow)
    } else {
        ("Diminished", Color::Red)
    };
    lines.push(Line::from(Span::styled(
        truncate_str(
            &format!("  Status: {}  |  Power: {}  |  Members: {}", health_label.0, inst.power, member_count),
            max_w,
        ),
        Style::default().fg(health_label.1),
    )));
    lines.push(Line::from(""));

    // -- Members --
    lines.push(Line::from(Span::styled(
        format!(" MEMBERS ({})", member_count),
        Style::default().fg(Color::Yellow),
    )));
    let mut member_lines = 0;
    for &mid in &inst.member_ids {
        if let Some(agent) = sim.agents.iter().find(|a| a.id == mid && a.alive) {
            lines.push(Line::from(Span::styled(
                truncate_str(&format!("  {}", agent.display_name()), max_w),
                Style::default().fg(Color::White),
            )));
            member_lines += 1;
            if member_lines >= 20 {
                let remaining = inst.member_ids.iter()
                    .filter(|id| sim.agents.iter().any(|a| a.id == **id && a.alive))
                    .count() - 20;
                if remaining > 0 {
                    lines.push(Line::from(Span::styled(
                        format!("  ... and {} more", remaining),
                        Style::default().fg(Color::DarkGray),
                    )));
                }
                break;
            }
        }
    }
    if member_count == 0 {
        lines.push(Line::from(Span::styled("  No living members.", Style::default().fg(Color::DarkGray))));
    }
    lines.push(Line::from(""));

    // -- Relationships --
    lines.push(Line::from(Span::styled(" RELATIONSHIPS", Style::default().fg(Color::Yellow))));
    if inst.relationships.is_empty() {
        lines.push(Line::from(Span::styled("  No formal relations on record.", Style::default().fg(Color::DarkGray))));
    } else {
        for (&other_id, rel) in &inst.relationships {
            if let Some(other) = sim.institutions.iter().find(|i| i.id == other_id) {
                let rel_color = match rel {
                    crate::sim::institution::InstitutionRelationship::Allied => Color::Green,
                    crate::sim::institution::InstitutionRelationship::Neutral => Color::White,
                    crate::sim::institution::InstitutionRelationship::Rival => Color::Red,
                    crate::sim::institution::InstitutionRelationship::Disputed(_) => Color::Yellow,
                };
                lines.push(Line::from(Span::styled(
                    truncate_str(&format!("  {} — {}", other.name, rel.label()), max_w),
                    Style::default().fg(rel_color),
                )));
            }
        }
    }
    lines.push(Line::from(""));

    // -- Artifacts held by faction members --
    lines.push(Line::from(Span::styled(" ARTIFACTS", Style::default().fg(Color::Yellow))));
    let mut artifact_count = 0;
    for art in &sim.artifacts {
        // Check if held by a member of this institution
        let held_by_member = inst.member_ids.iter().any(|mid| {
            sim.agents.iter().any(|a| a.id == *mid && a.held_artifacts.contains(&art.id))
        });
        if held_by_member {
            if let Some(holder) = sim.agents.iter().find(|a| a.held_artifacts.contains(&art.id)) {
                lines.push(Line::from(Span::styled(
                    truncate_str(
                        &format!("  {} ({}) — held by {}", art.name, art.kind.label(), holder.display_name()),
                        max_w,
                    ),
                    Style::default().fg(Color::LightYellow),
                )));
                artifact_count += 1;
            }
        }
    }
    if artifact_count == 0 {
        lines.push(Line::from(Span::styled("  None on record.", Style::default().fg(Color::DarkGray))));
    }
    lines.push(Line::from(""));

    // -- Notable events from annals --
    lines.push(Line::from(Span::styled(" HISTORICAL RECORD", Style::default().fg(Color::Yellow))));
    let mut history_count = 0;
    for entry in &sim.annals {
        if entry.notable_institutions.iter().any(|n| n == &inst.name) {
            for wl in wrap_text(&format!("  [{}] {}", entry.era_name, entry.defining_event), max_w) {
                lines.push(Line::from(Span::styled(wl, Style::default().fg(Color::Gray))));
            }
            history_count += 1;
        }
    }
    // Also show recent chronicle entries
    let chronicle_start = inst.chronicle.len().saturating_sub(10);
    for ch in &inst.chronicle[chronicle_start..] {
        for wl in wrap_text(&format!("  {}", ch), max_w) {
            lines.push(Line::from(Span::styled(wl, Style::default().fg(Color::DarkGray))));
        }
        history_count += 1;
    }
    if history_count == 0 {
        lines.push(Line::from(Span::styled("  No notable events recorded.", Style::default().fg(Color::DarkGray))));
    }
    lines.push(Line::from(""));

    // -- Footer --
    lines.push(Line::from(Span::styled(" Up/Down=scroll  ESC=back", Style::default().fg(Color::DarkGray))));

    // Apply scroll
    let total_lines = lines.len();
    let effective_scroll = scroll.min(total_lines.saturating_sub(inner_height));
    let visible_lines: Vec<Line> = lines.into_iter()
        .skip(effective_scroll)
        .take(inner_height)
        .collect();

    let block = Block::default()
        .title(" FACTION DOSSIER ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let widget = Paragraph::new(visible_lines).block(block);
    frame.render_widget(widget, area);
}

/// Truncate a string to fit within `max_width` columns, adding "..." if truncated.
fn truncate_str(s: &str, max_width: usize) -> String {
    if max_width <= 3 {
        return s.chars().take(max_width).collect();
    }
    let char_count: usize = s.chars().count();
    if char_count <= max_width {
        s.to_string()
    } else {
        let truncated: String = s.chars().take(max_width - 3).collect();
        format!("{}...", truncated)
    }
}

/// Word-wrap a string into multiple lines that fit within `max_width` columns.
/// Preserves leading whitespace on the first line; continuation lines get the same indent.
fn wrap_text(s: &str, max_width: usize) -> Vec<String> {
    if max_width == 0 {
        return vec![s.to_string()];
    }
    let char_count: usize = s.chars().count();
    if char_count <= max_width {
        return vec![s.to_string()];
    }
    // Determine indent from leading spaces
    let indent_len = s.chars().take_while(|c| *c == ' ').count();
    let indent: String = " ".repeat(indent_len.min(max_width / 2));
    let cont_indent = format!("{}  ", indent); // continuation gets 2 extra spaces

    let mut result = Vec::new();
    let mut current_line = String::new();
    let mut is_first = true;

    for word in s.split_whitespace() {
        if current_line.is_empty() {
            let pfx = if is_first { &indent } else { &cont_indent };
            current_line = format!("{}{}", pfx, word);
        } else {
            let test_len = current_line.chars().count() + 1 + word.chars().count();
            if test_len > max_width {
                result.push(current_line);
                current_line = format!("{}{}", cont_indent, word);
                is_first = false;
            } else {
                current_line.push(' ');
                current_line.push_str(word);
            }
        }
    }
    if !current_line.is_empty() {
        result.push(current_line);
    }
    if result.is_empty() {
        result.push(String::new());
    }
    result
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

            let artifact_count = site.artifacts.len();
            let artifact_label = if artifact_count > 0 {
                format!(" | {} artifact{}", artifact_count, if artifact_count == 1 { "" } else { "s" })
            } else {
                String::new()
            };

            let inhab_count = site.inhabitants.len();
            let inhab_label = if inhab_count > 0 {
                format!(" | {} inhabitant{}", inhab_count, if inhab_count == 1 { "" } else { "s" })
            } else {
                String::new()
            };

            let detail = format!(
                "     {} | ({},{}) | {} floor{} | {}{}{}",
                site.kind.label(),
                site.grid_x, site.grid_y,
                site.floors.len(),
                if site.floors.len() == 1 { "" } else { "s" },
                faction_label,
                artifact_label,
                inhab_label,
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
    let area = centered_rect(58, 95, frame.area());
    frame.render_widget(Clear, area);

    let key_style = Style::default().fg(Color::Rgb(220, 190, 100));
    let desc_style = Style::default().fg(Color::Rgb(170, 170, 165));
    let header_style = Style::default().fg(Color::Rgb(130, 200, 160));
    let title_style = Style::default().fg(Color::Rgb(220, 210, 180));
    let dim_style = Style::default().fg(Color::Rgb(80, 80, 85));
    let sep = Line::from(Span::styled(
        "  \u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}",
        dim_style,
    ));

    let lines: Vec<Line> = vec![
        Line::from(Span::styled(" VERIDIAN CONTRAPTION \u{2014} REFERENCE", title_style)),
        Line::from(""),
        Line::from(Span::styled(" SIMULATION CONTROL", header_style)),
        Line::from(vec![Span::styled("   SPACE    ", key_style), Span::styled("Pause / unpause", desc_style)]),
        Line::from(vec![Span::styled("   .        ", key_style), Span::styled("Step one tick (while paused)", desc_style)]),
        Line::from(vec![Span::styled("   0        ", key_style), Span::styled("Speed 0.5x (half speed)", desc_style)]),
        Line::from(vec![Span::styled("   1        ", key_style), Span::styled("Speed 1x", desc_style)]),
        Line::from(vec![Span::styled("   2        ", key_style), Span::styled("Speed 5x", desc_style)]),
        Line::from(vec![Span::styled("   3        ", key_style), Span::styled("Speed 10x", desc_style)]),
        sep.clone(),
        Line::from(Span::styled(" NAVIGATION", header_style)),
        Line::from(vec![Span::styled("   i        ", key_style), Span::styled("Find agent by name", desc_style)]),
        Line::from(vec![Span::styled("   Tab      ", key_style), Span::styled("Browse all agents", desc_style)]),
        Line::from(vec![Span::styled("   f        ", key_style), Span::styled("Follow agent or institution", desc_style)]),
        Line::from(vec![Span::styled("   F        ", key_style), Span::styled("Faction list", desc_style)]),
        Line::from(vec![Span::styled("   s        ", key_style), Span::styled("Sites (dungeons, ruins, shrines)", desc_style)]),
        Line::from(vec![Span::styled("   l        ", key_style), Span::styled("Map legend", desc_style)]),
        Line::from(vec![Span::styled("   PgUp/Dn  ", key_style), Span::styled("Scroll log pane", desc_style)]),
        sep.clone(),
        Line::from(Span::styled(" INSPECTION", header_style)),
        Line::from(vec![Span::styled("   W        ", key_style), Span::styled("World Assessment Report", desc_style)]),
        Line::from(vec![Span::styled("   a        ", key_style), Span::styled("World Annals (era history)", desc_style)]),
        sep.clone(),
        Line::from(Span::styled(" EXPORT & SAVE", header_style)),
        Line::from(vec![Span::styled("   e        ", key_style), Span::styled("Export (log, factions, chronicles, annals)", desc_style)]),
        Line::from(vec![Span::styled("   Ctrl+S   ", key_style), Span::styled("Save (silent re-save if named)", desc_style)]),
        Line::from(vec![Span::styled("  Ctrl+Sh+S ", key_style), Span::styled("Save As (always prompts)", desc_style)]),
        sep.clone(),
        Line::from(Span::styled(" THE ESCHATON", header_style)),
        Line::from(vec![Span::styled("   Shift+E  ", key_style), Span::styled("Immanentize the Eschaton", desc_style)]),
        Line::from(vec![Span::styled("            ", key_style), Span::styled("(confirmation required; irreversible)", dim_style)]),
        sep,
        Line::from(vec![Span::styled("   q        ", key_style), Span::styled("Return to main menu", desc_style)]),
        Line::from(vec![Span::styled("   ?        ", key_style), Span::styled("This help screen", desc_style)]),
        Line::from(""),
        Line::from(Span::styled(" ESC to close", dim_style)),
    ];

    let block = Block::default()
        .title(" HELP ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(100, 100, 90)));

    let widget = Paragraph::new(lines).block(block);
    frame.render_widget(widget, area);
}

/// Draw the World Annals overlay.
pub fn draw_annals(frame: &mut Frame, sim: &SimState, scroll: usize) {
    let area = centered_rect(75, 85, frame.area());
    frame.render_widget(Clear, area);

    let header_style = Style::default().fg(Color::Yellow);
    let era_style = Style::default().fg(Color::White);
    let body_style = Style::default().fg(Color::Gray);
    let dim_style = Style::default().fg(Color::DarkGray);
    let current_style = Style::default().fg(Color::Green);

    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from(Span::styled(
        format!(" WORLD ANNALS — {}", sim.world.name),
        header_style,
    )));
    lines.push(Line::from(""));

    if sim.annals.is_empty() {
        lines.push(Line::from(Span::styled(
            "  No completed eras yet.",
            dim_style,
        )));
        lines.push(Line::from(""));
    }

    for entry in &sim.annals {
        lines.push(Line::from(Span::styled(
            format!("  {} (ticks {}–{})", entry.era_name, entry.start_tick, entry.end_tick),
            era_style,
        )));
        lines.push(Line::from(""));

        // Word-wrap summary
        let inner_width = area.width.saturating_sub(6) as usize;
        let words: Vec<&str> = entry.summary.split_whitespace().collect();
        let mut line_buf = String::from("    ");
        for word in &words {
            if line_buf.len() + 1 + word.len() > inner_width && line_buf.len() > 4 {
                lines.push(Line::from(Span::styled(line_buf.clone(), body_style)));
                line_buf = String::from("    ");
            }
            if line_buf.len() > 4 { line_buf.push(' '); }
            line_buf.push_str(word);
        }
        if line_buf.len() > 4 {
            lines.push(Line::from(Span::styled(line_buf, body_style)));
        }
        lines.push(Line::from(""));

        if !entry.notable_agents.is_empty() {
            lines.push(Line::from(Span::styled(
                format!("    Notable: {}", entry.notable_agents.join(", ")),
                dim_style,
            )));
        }
        if !entry.notable_institutions.is_empty() {
            lines.push(Line::from(Span::styled(
                format!("    Institutions: {}", entry.notable_institutions.join(", ")),
                dim_style,
            )));
        }
        lines.push(Line::from(Span::styled(
            "  ─────────────────────────────────────────",
            dim_style,
        )));
        lines.push(Line::from(""));
    }

    // Current era (ongoing)
    lines.push(Line::from(Span::styled(
        format!("  {} (tick {}–present)  CURRENT ERA", sim.current_era_name, sim.current_era_start),
        current_style,
    )));
    let alive = sim.agents.iter().filter(|a| a.alive).count();
    let living_inst = sim.institutions.iter().filter(|i| i.alive).count();
    lines.push(Line::from(Span::styled(
        format!("    Major events: {} / {}  |  Pop: {}  |  Institutions: {}",
            sim.era_major_events, crate::sim::ERA_THRESHOLD, alive, living_inst),
        dim_style,
    )));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        " ESC to close  |  Up/Down/PgUp/PgDn to scroll",
        dim_style,
    )));

    // Apply scroll
    let inner_h = area.height.saturating_sub(2) as usize;
    let max_scroll = lines.len().saturating_sub(inner_h);
    let effective_scroll = scroll.min(max_scroll);
    let end = (effective_scroll + inner_h).min(lines.len());
    let visible: Vec<Line> = lines[effective_scroll..end].to_vec();

    let block = Block::default()
        .title(" WORLD ANNALS ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let widget = Paragraph::new(visible).block(block);
    frame.render_widget(widget, area);
}

/// Draw the export menu overlay.
pub fn draw_export_menu(frame: &mut Frame) {
    let area = centered_rect(45, 40, frame.area());
    frame.render_widget(Clear, area);

    let lines = vec![
        Line::from(Span::styled(" EXPORT", Style::default().fg(Color::White))),
        Line::from(""),
        Line::from(Span::styled(" [1] Export Live Log", Style::default().fg(Color::Gray))),
        Line::from(Span::styled(" [2] Export Faction Record", Style::default().fg(Color::Gray))),
        Line::from(Span::styled(" [3] Export Character Chronicles", Style::default().fg(Color::Gray))),
        Line::from(Span::styled(" [4] Export World Annals", Style::default().fg(Color::Gray))),
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

/// Build the content lines for the World Assessment Report.
fn build_world_report_lines(sim: &SimState) -> Vec<Line<'static>> {
    let w = &sim.world;
    let label_style = Style::default().fg(Color::DarkGray);
    let header_style = Style::default().fg(Color::Yellow);
    let value_style = Style::default().fg(Color::White);
    let accent_style = Style::default().fg(Color::Cyan);
    let dim_style = Style::default().fg(Color::Rgb(100, 100, 100));

    let mut lines: Vec<Line> = Vec::new();

    // Title block
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "═══════════════════════════════════════════════════════════════",
        Style::default().fg(Color::DarkGray),
    )));
    lines.push(Line::from(Span::styled(
        "        WORLD ASSESSMENT REPORT",
        Style::default().fg(Color::Green),
    )));
    lines.push(Line::from(Span::styled(
        "        Prepared by the Bureau of Initial Conditions",
        dim_style,
    )));
    lines.push(Line::from(Span::styled(
        "═══════════════════════════════════════════════════════════════",
        Style::default().fg(Color::DarkGray),
    )));
    lines.push(Line::from(""));

    // World identity
    lines.push(Line::from(vec![
        Span::styled("  Designation:       ", label_style),
        Span::styled(w.name.clone(), accent_style),
    ]));
    lines.push(Line::from(vec![
        Span::styled("  Seed:              ", label_style),
        Span::styled(format!("{}", w.seed), value_style),
    ]));
    lines.push(Line::from(vec![
        Span::styled("  Current Tick:      ", label_style),
        Span::styled(format!("{}", w.tick), value_style),
    ]));
    lines.push(Line::from(""));

    // World parameters — generated per world
    lines.push(Line::from(Span::styled(
        "  OPERATIONAL PARAMETERS",
        header_style,
    )));
    lines.push(Line::from(Span::styled(
        "  ─────────────────────────────────────────",
        dim_style,
    )));
    let p = &w.params;
    use crate::sim::world::WorldParams;
    let param_display = [
        ("  Temporal Rate:         ", format!("{} ({:.2}x)", p.describe_temporal_rate(), p.temporal_rate)),
        ("  Political Churn:       ", format!("{} ({:.0}%)", WorldParams::describe_level(p.political_churn), p.political_churn * 100.0)),
        ("  Cosmological Density:  ", format!("{} ({:.0}%)", WorldParams::describe_level(p.cosmological_density), p.cosmological_density * 100.0)),
        ("  Ecological Volatility: ", format!("{} ({:.0}%)", WorldParams::describe_level(p.ecological_volatility), p.ecological_volatility * 100.0)),
        ("  Narrative Register:    ", p.narrative_register.label().to_string()),
        ("  Weirdness Coefficient: ", format!("{} ({:.0}%)", WorldParams::describe_level(p.weirdness_coefficient), p.weirdness_coefficient * 100.0)),
    ];
    for (lbl, val) in &param_display {
        lines.push(Line::from(vec![
            Span::styled(lbl.to_string(), label_style),
            Span::styled(val.clone(), value_style),
        ]));
    }
    lines.push(Line::from(""));

    // Peoples
    lines.push(Line::from(Span::styled(
        "  REGISTERED PEOPLES",
        header_style,
    )));
    lines.push(Line::from(Span::styled(
        "  ─────────────────────────────────────────",
        dim_style,
    )));

    let total_pop: u32 = w.peoples.iter().map(|p| p.population).sum();
    let alive_count = sim.agents.iter().filter(|a| a.alive).count();

    for people in &w.peoples {
        let terrain_note: Vec<&str> = people.preferred_terrain.iter().map(|t| match t {
            crate::sim::world::Terrain::Plains => "plains-dwelling",
            crate::sim::world::Terrain::Hills => "hill-favoring",
            crate::sim::world::Terrain::Forest => "forest-adapted",
            crate::sim::world::Terrain::Mountains => "mountain-dwelling",
            crate::sim::world::Terrain::Desert => "desert-acclimated",
            _ => "amphibiously inclined",
        }).collect();
        let culture_note = if terrain_note.is_empty() {
            "of uncertain territorial preference".to_string()
        } else {
            terrain_note.join(", ")
        };

        // Count living agents of this people
        let people_idx = w.peoples.iter().position(|p| p.name == people.name).unwrap_or(0);
        let living = sim.agents.iter().filter(|a| a.alive && a.people_id == people_idx).count();

        lines.push(Line::from(vec![
            Span::styled("  ", label_style),
            Span::styled(people.name.clone(), accent_style),
        ]));
        lines.push(Line::from(vec![
            Span::styled("    Registered population: ", label_style),
            Span::styled(format!("{}", people.population), value_style),
            Span::styled(format!("  (agents extant: {})", living), dim_style),
        ]));
        lines.push(Line::from(vec![
            Span::styled("    Cultural assessment:   ", label_style),
            Span::styled(culture_note, value_style),
        ]));
    }
    lines.push(Line::from(vec![
        Span::styled("  Total registered population: ", label_style),
        Span::styled(format!("{}", total_pop), value_style),
        Span::styled(format!("  (living agents: {})", alive_count), dim_style),
    ]));
    lines.push(Line::from(""));

    // Settlements
    lines.push(Line::from(Span::styled(
        "  CHARTERED SETTLEMENTS",
        header_style,
    )));
    lines.push(Line::from(Span::styled(
        "  ─────────────────────────────────────────",
        dim_style,
    )));
    for s in &w.settlements {
        let size_label = match s.size {
            crate::sim::world::SettlementSize::Hamlet => "Hamlet",
            crate::sim::world::SettlementSize::Town => "Town",
            crate::sim::world::SettlementSize::City => "City",
        };
        lines.push(Line::from(vec![
            Span::styled("  ", label_style),
            Span::styled(s.name.clone(), accent_style),
            Span::styled(format!("  ({}, grid {},{})", size_label, s.x, s.y), label_style),
        ]));
    }
    lines.push(Line::from(vec![
        Span::styled(
            format!("  {} settlement{} on record.", w.settlements.len(),
                if w.settlements.len() == 1 { "" } else { "s" }),
            dim_style,
        ),
    ]));
    lines.push(Line::from(""));

    // Institutions
    let active_institutions: Vec<_> = sim.institutions.iter().filter(|i| i.alive).collect();
    lines.push(Line::from(Span::styled(
        "  RECOGNIZED INSTITUTIONS",
        header_style,
    )));
    lines.push(Line::from(Span::styled(
        "  ─────────────────────────────────────────",
        dim_style,
    )));
    if active_institutions.is_empty() {
        lines.push(Line::from(Span::styled(
            "  None currently recognized. (This is administratively unusual.)",
            dim_style,
        )));
    } else {
        for inst in &active_institutions {
            lines.push(Line::from(vec![
                Span::styled("  ", label_style),
                Span::styled(inst.name.clone(), accent_style),
                Span::styled(format!("  ({})", inst.kind.label()), label_style),
            ]));
            lines.push(Line::from(vec![
                Span::styled("    Charter: ", label_style),
                Span::styled(inst.charter.clone(), value_style),
            ]));
            lines.push(Line::from(vec![
                Span::styled("    Members: ", label_style),
                Span::styled(format!("{}  ", inst.member_ids.len()), value_style),
                Span::styled("Power: ", label_style),
                Span::styled(format!("{}", inst.power), value_style),
            ]));
        }
    }
    lines.push(Line::from(vec![
        Span::styled(
            format!("  {} institution{} at present.",
                active_institutions.len(),
                if active_institutions.len() == 1 { "" } else { "s" }),
            dim_style,
        ),
    ]));
    lines.push(Line::from(""));

    // Sites
    lines.push(Line::from(Span::styled(
        "  CATALOGUED SITES OF INTEREST",
        header_style,
    )));
    lines.push(Line::from(Span::styled(
        "  ─────────────────────────────────────────",
        dim_style,
    )));
    if sim.sites.is_empty() {
        lines.push(Line::from(Span::styled(
            "  No sites catalogued. (The cartographers have been notified.)",
            dim_style,
        )));
    } else {
        for site in &sim.sites {
            let artifact_note = if site.artifacts.is_empty() {
                String::new()
            } else {
                format!("  [{} artifact{}]",
                    site.artifacts.len(),
                    if site.artifacts.len() == 1 { "" } else { "s" })
            };
            lines.push(Line::from(vec![
                Span::styled("  ", label_style),
                Span::styled(site.name.clone(), Style::default().fg(site.kind.map_color())),
                Span::styled(format!("  ({})", site.kind.label()), label_style),
                Span::styled(artifact_note, Style::default().fg(Color::LightYellow)),
            ]));
            lines.push(Line::from(vec![
                Span::styled("    ", label_style),
                Span::styled(site.origin.clone(), dim_style),
            ]));
        }
    }
    lines.push(Line::from(vec![
        Span::styled(
            format!("  {} site{} documented.",
                sim.sites.len(),
                if sim.sites.len() == 1 { "" } else { "s" }),
            dim_style,
        ),
    ]));
    lines.push(Line::from(""));

    // Artifacts summary
    if !sim.artifacts.is_empty() {
        lines.push(Line::from(Span::styled(
            "  REGISTERED ARTIFACTS",
            header_style,
        )));
        lines.push(Line::from(Span::styled(
            "  ─────────────────────────────────────────",
            dim_style,
        )));
        for art in &sim.artifacts {
            let loc_str = match &art.current_location {
                crate::sim::artifact::ArtifactLocation::InSite(idx) => {
                    sim.sites.get(*idx).map(|s| format!("in {}", s.name))
                        .unwrap_or_else(|| "in unknown site".to_string())
                }
                crate::sim::artifact::ArtifactLocation::HeldByAgent(id) => {
                    sim.agents.iter().find(|a| a.id == *id)
                        .map(|a| format!("held by {}", a.display_name()))
                        .unwrap_or_else(|| "held by unknown party".to_string())
                }
                crate::sim::artifact::ArtifactLocation::InSettlement(idx) => {
                    sim.world.settlements.get(*idx).map(|s| format!("in {}", s.name))
                        .unwrap_or_else(|| "in unknown settlement".to_string())
                }
                crate::sim::artifact::ArtifactLocation::Lost => "whereabouts unknown".to_string(),
            };
            lines.push(Line::from(vec![
                Span::styled("  ", label_style),
                Span::styled(art.name.clone(), Style::default().fg(Color::LightYellow)),
                Span::styled(format!("  ({})", art.kind.label()), label_style),
            ]));
            lines.push(Line::from(vec![
                Span::styled("    Location: ", label_style),
                Span::styled(loc_str, value_style),
            ]));
        }
        lines.push(Line::from(""));
    }

    // Eschaton status
    lines.push(Line::from(Span::styled(
        "  ESCHATON STATUS",
        header_style,
    )));
    lines.push(Line::from(Span::styled(
        "  ─────────────────────────────────────────",
        dim_style,
    )));
    let eschaton_count = sim.eschaton_history.len();
    if eschaton_count == 0 {
        lines.push(Line::from(vec![
            Span::styled("  Eschatons recorded:    ", label_style),
            Span::styled("None", value_style),
        ]));
        lines.push(Line::from(Span::styled(
            "  (The world has not yet been fundamentally reorganized.)",
            dim_style,
        )));
    } else {
        lines.push(Line::from(vec![
            Span::styled("  Eschatons recorded:    ", label_style),
            Span::styled(format!("{}", eschaton_count), Style::default().fg(Color::LightRed)),
        ]));
        for record in &sim.eschaton_history {
            lines.push(Line::from(vec![
                Span::styled("    Tick ", label_style),
                Span::styled(format!("{}: ", record.tick), value_style),
                Span::styled(record.eschaton_type.label().to_string(), Style::default().fg(Color::LightRed)),
            ]));
        }
    }
    let tension = sim.tension;
    lines.push(Line::from(vec![
        Span::styled("  Current tension:       ", label_style),
        Span::styled(
            format!("{:.0}%", tension * 100.0),
            if tension > 0.7 { Style::default().fg(Color::LightRed) } else { value_style },
        ),
    ]));
    lines.push(Line::from(""));

    // Closing flourish
    lines.push(Line::from(Span::styled(
        "═══════════════════════════════════════════════════════════════",
        Style::default().fg(Color::DarkGray),
    )));
    lines.push(Line::from(Span::styled(
        "  This report was generated automatically by the Bureau of",
        dim_style,
    )));
    lines.push(Line::from(Span::styled(
        "  Initial Conditions. Any resemblance to a coherent world is",
        dim_style,
    )));
    lines.push(Line::from(Span::styled(
        "  provisional and subject to revision without notice.",
        dim_style,
    )));
    lines.push(Line::from(Span::styled(
        "═══════════════════════════════════════════════════════════════",
        Style::default().fg(Color::DarkGray),
    )));
    lines.push(Line::from(""));

    lines
}

/// Draw the World Assessment Report as a fullscreen view.
/// `is_pre_sim` controls whether R=Reroll is shown (only before sim starts).
pub fn draw_world_report_fullscreen(frame: &mut Frame, sim: &SimState, scroll: usize, is_pre_sim: bool) {
    let area = frame.area();
    frame.render_widget(Clear, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),    // report content
            Constraint::Length(1), // footer
        ])
        .split(area);

    let inner_height = chunks[0].height.saturating_sub(2) as usize;
    let report_lines = build_world_report_lines(sim);
    let total = report_lines.len();
    let max_scroll = total.saturating_sub(inner_height);
    let effective_scroll = scroll.min(max_scroll);
    let end = (effective_scroll + inner_height).min(total);
    let visible: Vec<Line> = report_lines[effective_scroll..end].to_vec();

    let block = Block::default()
        .title(" WORLD ASSESSMENT REPORT ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green));

    let widget = Paragraph::new(visible).block(block);
    frame.render_widget(widget, chunks[0]);

    // Footer
    let footer_text = if is_pre_sim {
        " ENTER = Begin Simulation  |  R = Reroll  |  Up/Down/PgUp/PgDn = Scroll  |  ESC = Cancel"
    } else {
        " ESC = Close  |  Up/Down/PgUp/PgDn = Scroll"
    };
    let footer = Paragraph::new(Span::styled(
        footer_text,
        Style::default().fg(Color::DarkGray),
    ));
    frame.render_widget(footer, chunks[1]);
}

/// Draw the Eschaton confirmation overlay — ominous, warns of consequences.
pub fn draw_eschaton_confirm(frame: &mut Frame, sim: &SimState, selected: usize) {
    let area = centered_rect(65, 70, frame.area());
    frame.render_widget(Clear, area);

    let warn_style = Style::default().fg(Color::LightRed);
    let dim_style = Style::default().fg(Color::DarkGray);
    let label_style = Style::default().fg(Color::Gray);
    let value_style = Style::default().fg(Color::White);
    let ominous_style = Style::default().fg(Color::Yellow);

    let tension = sim.tension;
    let cosmo = sim.world.params.cosmological_density;
    let can_fire = sim.can_eschaton();
    let eschaton_count = sim.eschaton_history.len();

    let tension_bar = format!(
        "[{}{}] {:.0}%",
        "#".repeat((tension * 10.0).round() as usize),
        ".".repeat(10 - (tension * 10.0).round().min(10.0) as usize),
        tension * 100.0
    );
    let cosmo_bar = format!(
        "[{}{}] {:.0}%",
        "#".repeat((cosmo * 10.0).round() as usize),
        ".".repeat(10 - (cosmo * 10.0).round().min(10.0) as usize),
        cosmo * 100.0
    );

    let mut lines: Vec<Line> = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  ╔═══════════════════════════════════════════════════════╗",
            warn_style,
        )),
        Line::from(Span::styled(
            "  ║     IMMANENTIZE THE ESCHATON                        ║",
            warn_style,
        )),
        Line::from(Span::styled(
            "  ╚═══════════════════════════════════════════════════════╝",
            warn_style,
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  You are about to bring about the end of the current order.",
            ominous_style,
        )),
        Line::from(Span::styled(
            "  The consequences are permanent and unpredictable.",
            ominous_style,
        )),
        Line::from(Span::styled(
            "  The world will not end. It will be changed.",
            ominous_style,
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  ─────────────────────────────────────────────────────",
            dim_style,
        )),
        Line::from(vec![
            Span::styled("  World Tension:          ", label_style),
            Span::styled(tension_bar, if tension > TENSION_THRESHOLD { warn_style } else { value_style }),
        ]),
        Line::from(vec![
            Span::styled("  Cosmological Density:   ", label_style),
            Span::styled(cosmo_bar, if cosmo > COSMO_THRESHOLD { warn_style } else { value_style }),
        ]),
        Line::from(vec![
            Span::styled("  Previous Eschatons:     ", label_style),
            Span::styled(format!("{}", eschaton_count), value_style),
        ]),
    ];

    if !can_fire {
        let ticks_remaining = ESCHATON_COOLDOWN.saturating_sub(sim.world.tick - sim.last_eschaton_tick);
        lines.push(Line::from(vec![
            Span::styled("  Cooldown:               ", label_style),
            Span::styled(format!("{} ticks remaining", ticks_remaining), Style::default().fg(Color::Red)),
        ]));
    }

    lines.push(Line::from(Span::styled(
        "  ─────────────────────────────────────────────────────",
        dim_style,
    )));
    lines.push(Line::from(""));

    if !can_fire {
        lines.push(Line::from(Span::styled(
            "  The Eschaton is not yet available. The world requires more time.",
            Style::default().fg(Color::Red),
        )));
        lines.push(Line::from(""));
    }

    // Selection options
    let confirm_style = if selected == 0 {
        Style::default().fg(Color::Black).bg(Color::LightRed)
    } else {
        if can_fire { warn_style } else { Style::default().fg(Color::DarkGray) }
    };
    let cancel_style = if selected == 1 {
        Style::default().fg(Color::Black).bg(Color::White)
    } else {
        value_style
    };

    lines.push(Line::from(vec![
        Span::styled("     ", label_style),
        Span::styled("  IMMANENTIZE  ", confirm_style),
        Span::styled("     ", label_style),
        Span::styled("  CANCEL  ", cancel_style),
    ]));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  There is no confirmation after this. There is no undo.",
        dim_style,
    )));

    // Show eschaton history if any
    if !sim.eschaton_history.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "  PREVIOUS ESCHATONS",
            Style::default().fg(Color::Yellow),
        )));
        for record in &sim.eschaton_history {
            lines.push(Line::from(vec![
                Span::styled("    Tick ", label_style),
                Span::styled(format!("{}: ", record.tick), value_style),
                Span::styled(record.eschaton_type.label().to_string(), warn_style),
            ]));
        }
    }

    let block = Block::default()
        .title(" ESCHATON ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::LightRed));

    let widget = Paragraph::new(lines).block(block);
    frame.render_widget(widget, area);
}

/// Simple text bar visualization for a 0.0-1.0 value.
fn disposition_bar(value: f32) -> String {
    let filled = (value * 10.0).round() as usize;
    let empty = 10 - filled.min(10);
    format!("[{}{}] {:.0}%", "#".repeat(filled), ".".repeat(empty), value * 100.0)
}

/// Draw the map legend overlay showing symbol meanings.
pub fn draw_map_legend(frame: &mut Frame) {
    let area = centered_rect(45, 70, frame.area());
    frame.render_widget(Clear, area);

    let header = Style::default().fg(Color::Rgb(220, 210, 180));
    let desc = Style::default().fg(Color::Rgb(160, 160, 155));
    let dim = Style::default().fg(Color::Rgb(80, 80, 85));

    let lines: Vec<Line> = vec![
        Line::from(Span::styled(" MAP LEGEND", header)),
        Line::from(""),
        Line::from(Span::styled(" TERRAIN", header)),
        Line::from(vec![Span::styled("   ~  ", Style::default().fg(Color::Rgb(20, 60, 140))),   Span::styled("Deep Water", desc)]),
        Line::from(vec![Span::styled("   :  ", Style::default().fg(Color::Rgb(60, 130, 190))),   Span::styled("Shallow Water", desc)]),
        Line::from(vec![Span::styled("   .  ", Style::default().fg(Color::Rgb(90, 160, 60))),    Span::styled("Plains", desc)]),
        Line::from(vec![Span::styled("   ^  ", Style::default().fg(Color::Rgb(170, 150, 80))),   Span::styled("Hills", desc)]),
        Line::from(vec![Span::styled("   T  ", Style::default().fg(Color::Rgb(30, 110, 40))),    Span::styled("Forest", desc)]),
        Line::from(vec![Span::styled("   M  ", Style::default().fg(Color::Rgb(140, 140, 155))),  Span::styled("Mountains", desc)]),
        Line::from(vec![Span::styled("   s  ", Style::default().fg(Color::Rgb(210, 180, 60))),   Span::styled("Desert", desc)]),
        Line::from(""),
        Line::from(Span::styled(" SETTLEMENTS", header)),
        Line::from(vec![Span::styled("   \u{00B7}  ", Style::default().fg(Color::Rgb(180, 170, 150))),  Span::styled("Hamlet", desc)]),
        Line::from(vec![Span::styled("   o  ", Style::default().fg(Color::Rgb(230, 210, 160))),  Span::styled("Town", desc)]),
        Line::from(vec![Span::styled("   O  ", Style::default().fg(Color::Rgb(255, 240, 200))),  Span::styled("City", desc)]),
        Line::from(""),
        Line::from(Span::styled(" ENTITIES", header)),
        Line::from(vec![Span::styled("   @  ", Style::default().fg(Color::Rgb(230, 120, 220))),  Span::styled("Agent (color = people)", desc)]),
        Line::from(vec![Span::styled("   3  ", Style::default().fg(Color::Rgb(100, 220, 230))),  Span::styled("Agent group (count shown)", desc)]),
        Line::from(vec![Span::styled("   X  ", Style::default().fg(Color::LightRed)),             Span::styled("Followed agent", desc)]),
        Line::from(vec![Span::styled("   \u{03A9}  ", Style::default().fg(Color::Red)),           Span::styled("Site (dungeon, ruin, etc.)", desc)]),
        Line::from(""),
        Line::from(Span::styled(" SITE INHABITANTS (in site view)", header)),
        Line::from(vec![Span::styled("   c  ", Style::default().fg(Color::Rgb(180, 160, 200))),  Span::styled("Creature/marginal figure", desc)]),
        Line::from(vec![Span::styled("   r  ", Style::default().fg(Color::Rgb(180, 160, 200))),  Span::styled("Remnant occupant", desc)]),
        Line::from(vec![Span::styled("   s  ", Style::default().fg(Color::Rgb(180, 160, 200))),  Span::styled("Shrine attendant", desc)]),
        Line::from(vec![Span::styled("   b  ", Style::default().fg(Color::Rgb(180, 160, 200))),  Span::styled("Bureaucratic staff", desc)]),
        Line::from(vec![Span::styled("   m  ", Style::default().fg(Color::Rgb(180, 160, 200))),  Span::styled("Mourner/investigator", desc)]),
        Line::from(vec![Span::styled("   t  ", Style::default().fg(Color::Rgb(180, 160, 200))),  Span::styled("Taxonomic anomaly", desc)]),
        Line::from(vec![Span::styled("   a  ", Style::default().fg(Color::Rgb(180, 160, 200))),  Span::styled("Abandoned staff", desc)]),
        Line::from(""),
        Line::from(Span::styled(" ESC or l to close", dim)),
    ];

    let block = Block::default()
        .title(" LEGEND ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(120, 110, 90)));

    let widget = Paragraph::new(lines).block(block);
    frame.render_widget(widget, area);
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
