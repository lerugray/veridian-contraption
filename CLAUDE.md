## REQUIRED READING — DO THIS FIRST EVERY SESSION

Read the following files before doing anything else:
- GDD.md (full game design document)
- SESSION_NOTES.md (summary of last session — if this file doesn't exist yet, ignore it)

Do not proceed until you have read all available files listed above.

---

# VERIDIAN CONTRAPTION — CLAUDE.md

This file is read by Claude Code at the start of every session.
Follow all instructions here unless the user explicitly overrides them in conversation.

---

## Project Overview

Veridian Contraption is an autonomous terminal-based world simulator written in Rust.
It generates living worlds of procedural strangeness, narrates them in dense bureaucratic
prose, and runs continuously whether the player interacts or not.

**Stack:** Rust, ratatui (TUI), crossterm, serde/serde_json, rand
**Platform target:** Windows (PowerShell/Windows Terminal), macOS, Linux
**Player:** Non-programmer. Does not write code. Pastes prompts and describes results.

---

## Design Principles — Never Violate These

1. **Idle-first.** The simulation runs fine without player input. Interaction is always optional.
2. **Prose has voice.** Log entries should read like a bureaucrat who has absorbed too much Borges.
   Dry, precise, deadpan, occasionally surreal. Never winking. Never random-sounding.
3. **Each world is its own system.** World parameters are generated per world, not global defaults.
4. **Nothing is fully destroyed.** Apocalyptic events (Eschaton) damage and transform — they do
   not reset or end the simulation.
5. **The player is a spectator and occasional nudge.** Never require player input to proceed.

---

## Aesthetic References

- **Prose register:** Nabokov's Bend Sinister (dense but readable), Illuminatus! Trilogy
  (paranoid institutional logic, absurdist deadpan), early D&D adventure modules (earnest
  specificity about ridiculous things)
- **Simulation depth:** Dwarf Fortress (emergent narrative, systemic complexity)
- **Language generation:** Caves of Qud (procedural prose with genuine voice)
- **Visual presentation:** Brogue, classic Dwarf Fortress (expressive ASCII, color as meaning)

---

## Architecture Reference

```
/veridian-contraption
  /src
    main.rs              — entry point, app loop, keybindings
    sim/
      mod.rs             — SimState, tick(), simulation orchestration
      world.rs           — World, WorldParams, terrain, settlements
      agent.rs           — Agent, Disposition, Goal, epithet system
      event.rs           — Event, EventType, event generation
      institution.rs     — Institution, InstitutionKind, relationships
      site.rs            — Site, Floor, dungeon generation
      artifact.rs        — Artifact, ArtifactKind, ownership chain
      eschaton.rs        — EschatonEvent, world-altering catastrophe system
    gen/
      mod.rs
      world_gen.rs       — parametric world generation, flavor presets
      name_gen.rs        — phoneme-based name generation, epithets
      prose_gen.rs       — log entry generation, register-sensitive templates
      dungeon_gen.rs     — site/floor generation
      eschaton_gen.rs    — eschaton event type selection and execution
    ui/
      mod.rs
      layout.rs          — ratatui panel layout, rendering
      input.rs           — keybinding handling
      overlays.rs        — inspect view, faction list, annals, eschaton menu
      menu.rs            — main menu, new game, save/load screens
    export/
      mod.rs             — TXT export, save/load serialization
  /data
    phonemes.json        — phoneme tables per cultural group
    prose_templates.json — log entry templates by event type and register
    event_defs.json      — event type definitions and trigger conditions
    eschaton_defs.json   — eschaton event types and their world effects
  /saves                 — player save files (.json)
  /exports               — player-exported TXT files
  CLAUDE.md              — this file
  SESSION_NOTES.md       — session handoff notes (maintained by Claude Code)
  Cargo.toml
```

---

## Current Build Phase

**Update this section at the end of every session.**

```
CURRENT PHASE: Phase 5 COMPLETE
LAST COMPLETED PROMPT: Phase 5-C — Final cleanup, prose depth, overlay fixes
STATUS: Working
NEXT TASK: Phase 5+ — Distribution prep, further polish, or new features as directed
```

---

## Coding Conventions

- **Rust edition:** 2021
- **Error handling:** Use `Result<T, Box<dyn std::error::Error>>` for file I/O.
  Use `unwrap()` sparingly — prefer `?` propagation or explicit error handling.
- **RNG:** Always pass `&mut Rng` explicitly rather than creating new RNG instances.
  Use a seedable RNG (rand's `StdRng`) so worlds are reproducible.
- **Comments:** Comment non-obvious logic. The player may need to understand
  the code structure in future sessions even without reading Rust fluently.
- **Module organization:** Keep sim/ modules focused. Don't put generation logic
  in sim/ — generation belongs in gen/.
- **Data files:** Static data (phoneme tables, prose templates) lives in /data as JSON.
  Load at startup, not on every use.
- **No panics in the sim loop.** The simulation should degrade gracefully, not crash.
  Wrap tick() in error handling.

---

## Non-Programmer Collaboration Protocol

The user is not a programmer. They paste prompts and describe results.

- **Explain decisions briefly.** When making a non-obvious architectural choice,
  note it in one sentence. Don't lecture.
- **Don't ask the user to edit code directly** unless there is truly no alternative,
  and in that case, show them exactly what to change, where, with copy-paste precision.
- **When something is broken,** diagnose and fix it. Don't ask the user to interpret
  error messages — they'll paste them and you should read them.
- **Prefer working code over perfect code.** Ship phases that run. Refactor later.
- **Keep cargo run working** at the end of every session. Never leave the project
  in a state that won't compile.

---

## Session Handoff System

### Purpose
Claude Code has no memory between sessions. SESSION_NOTES.md bridges the gap.

### Claude Code's Responsibilities

**During every session:**
- At the start: read SESSION_NOTES.md and orient yourself
- Every ~15-20 exchanges or when context is getting long: update SESSION_NOTES.md
- At the end of every session (or when asked): write a full session summary to SESSION_NOTES.md

**SESSION_NOTES.md is a full rewrite every time.**
Do not append to the file. Each update should replace the entire contents with a
fresh snapshot: current state, what's working, known issues, and next steps. Old
session history does not need to be preserved — only the current picture matters.

**SESSION_NOTES.md format:**
```markdown
# SESSION NOTES — Last updated: [date/time if known, otherwise tick/phase]

## Current State
- Phase: [current phase]
- Last working feature: [what was working when session ended]
- Files modified this session: [list]

## What We Were Doing
[2-3 sentences describing the task in progress]

## Decisions Made
[Any architectural or design decisions made this session that future Claude should know]

## Known Issues / In Progress
[Anything that was being worked on but not finished]
[Any known bugs that exist but weren't fixed yet]

## Next Steps
[Specific next task — ideally matching a prompt from the prompts doc, or a precise description]

## Notes for Next Claude
[Anything else relevant — player preferences, things that didn't work, context that matters]
```

### When to Warn the User

If the conversation has been going on for a long time and you sense context is running
short (responses getting less accurate, you're losing track of earlier details), 
tell the user plainly:

> "We're getting close to my context limit for this session. I'm going to update
> SESSION_NOTES.md now so nothing is lost. When you start a new session, just open
> Claude Code and say 'read SESSION_NOTES.md and continue where we left off.'"

Then update SESSION_NOTES.md before continuing.

---

## Key Systems Reference

### Simulation Controls
SPACE = pause/unpause | . = step | 1/5/2 = speed | q = quit

### Player-Facing Keybindings (target)
- i = inspect entity (search by name)
- f = follow mode / F = faction list
- a = world annals
- e = export menu
- W = world parameters
- l = map legend
- ? = help
- Shift+E = immanentize the eschaton (confirmation screen, left/right to navigate)
- Ctrl+S = save

### Immanentize the Eschaton
- Triggered by: player via Shift+E (with confirmation), OR autonomously when
  cosmological_density > 0.65, tension > 0.7, and random check passes
- 500-tick cooldown between eschatons
- Effect: fires an EschatonEvent that permanently alters world parameters,
  geography, institutions, and/or population — but never ends the simulation
- Eschaton types: The Reckoning of Debts, The Taxonomic Correction,
  The Administrative Singularity, The Geological Argument, The Doctrinal Cascade,
  The Arrival of Something Owed
- After an eschaton: world parameters shift, tension resets, cosmological_density
  drops, a new era begins, the Annals records it, status bar flashes

### Save System
- Ctrl+S: manual save (prompts for name)
- Autosave every 500 ticks to /saves/autosave.json
- Main menu: load named save, start new world
- Multiple saved worlds supported (Phase 4+) — up to 10 named saves
- Save files are JSON via serde

### World Flavor Presets
TheLongBureaucracy | TheBurningProvinces | TheDeepTaxonomy | TheConspiratorial Age | Unguided

### NarrativeRegister enum
Clinical | Lyrical | Bureaucratic | Ominous | Conspiratorial

---

## Known Issues

**At the start of every session, check this list and flag any items to the player
before proceeding with other work.**

*(No known issues at this time.)*

---

## Things That Have Not Worked (update as needed)

*(Add entries here when approaches fail so future sessions don't repeat them)*

---

## Steam / Distribution (Future — Phase 5+)

- Cross-platform builds via Rust's native cross-compilation support
- Packaging tool: cargo-bundle
- Steam supports terminal applications (precedent: Dwarf Fortress)
- No action needed during active development phases
- Multi-world support (player maintains up to 10 simultaneous worlds, switchable
  from main menu) is targeted for Phase 4 completion

---

*This file should be committed to the repository and kept up to date.
It is the single source of truth for project context across sessions.*
