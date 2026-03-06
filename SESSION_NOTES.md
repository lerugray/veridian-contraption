# SESSION NOTES — Last updated: 2026-03-06

## Current State
- Phase: Phase 5 IN PROGRESS (5-A complete, Eschaton system complete)
- Last working feature: Immanentize the Eschaton system — full implementation
- Build status: Compiles and runs cleanly (6 warnings, all pre-existing dead_code)

## What's Working
- **NEW: Eschaton System**
  - **Six Eschaton types**: Reckoning of Debts, Taxonomic Correction, Administrative Singularity, Geological Argument, Doctrinal Cascade, Arrival of Something Owed
  - **Player trigger**: Shift+E opens ominous confirmation screen showing tension, cosmological density, eschaton history; Left/Right arrows to navigate, Enter to confirm
  - **Autonomous trigger**: Fires when cosmological_density > 0.65 AND tension > 0.7 AND random check passes (every 50 ticks)
  - **Tension system**: Accumulates from agent deaths (+0.02), institution dissolutions (+0.05), schisms (+0.03), rivalries (+0.02), adventurer deaths (+0.01); decays -0.001/tick
  - **Cooldown**: 500 ticks between eschatons
  - **Mechanical effects per type**:
    - Reckoning: dissolves ~40% institutions, inverts ~30% charters, clears all relationships, spawns 1-2 new institutions
    - Taxonomic Correction: revokes all epithets, assigns post-correction epithets, renames ~40% settlements
    - Administrative Singularity: merges all institutions → 3-5 rival successor bodies
    - Geological Argument: reshapes ~20% terrain, removes/moves settlements, adds 1-3 new ones
    - Doctrinal Cascade: revises all doctrines, ~30% agents lose affiliation, spawns 2-4 new institutions
    - Arrival: spawns 5-12 mysterious new agents with high ambition and distinctive epithets
  - **Post-eschaton effects**: forces era transition, resets tension to 0.1, reduces cosmological_density by 0.2, slightly shifts political_churn and weirdness_coefficient
  - **UI: Status bar flash** — "THE ESCHATON HAS OCCURRED" in alternating red/yellow for ~5 seconds
  - **UI: Log flood** — 8-12 dense prose events generated per eschaton, all register-sensitive
  - **UI: World Report** — shows eschaton count, history, current tension level
  - **UI: Help screen** — updated with Shift+E keybinding
  - **Save/load**: All eschaton state (history, tension, last_eschaton_tick) persisted via SaveData

- **EXISTING: All Phase 1-5A systems** — fully working

## Files Modified This Session
- `src/sim/eschaton.rs` — NEW: EschatonType, EschatonRecord, constants
- `src/gen/eschaton_gen.rs` — NEW: prose generation + mechanical execution for all 6 types
- `src/sim/mod.rs` — Added eschaton fields to SimState/SaveData, tension tracking in tick(), execute_eschaton(), can_eschaton()
- `src/sim/event.rs` — Added EschatonFired event type with LightRed color
- `src/gen/mod.rs` — Registered eschaton_gen module
- `src/gen/prose_gen.rs` — Added EschatonFired to match in generate_description
- `src/main.rs` — Added Shift+E keybinding, EschatonConfirm overlay handler (Left/Right navigation)
- `src/ui/layout.rs` — Added EschatonConfirm overlay rendering, status bar eschaton flash
- `src/ui/overlays.rs` — Added draw_eschaton_confirm(), eschaton section in world report, help screen update
- `CLAUDE.md` — Updated phase tracking and keybindings
- `SESSION_NOTES.md` — Full rewrite

## Decisions Made
- Shift+E for player trigger (not ESC — ESC is already used for closing overlays)
- Left/Right arrows to navigate Eschaton confirmation (horizontal layout matches button placement)
- Default selection on eschaton confirm is Cancel (safe default)
- Tension decays slowly (-0.001/tick) so it accumulates over time
- Autonomous trigger checks every 50 ticks (not every tick) to reduce overhead
- Cosmological density reduced by 0.2 after each eschaton (prevents rapid re-triggering)
- All eschaton prose is register-sensitive (Bureaucratic, Ominous, Clinical, etc.)
- Geological Argument always keeps at least 2 settlements
- Administrative Singularity creates 3-5 successor bodies (always more fragmentation)
- Arrival agents have high ambition (0.5-1.0) and mysterious epithets

## Known Issues
- Room purposes not yet referenced in prose generation (deferred from Phase 3)
- 6 compiler warnings (pre-existing, all dead_code)
- EschatonType::description() method exists but is #[allow(dead_code)] — available for future use

## Next Steps
- Phase 5-B: Color/symbol tuning (Brogue-quality ASCII expressiveness), full export system
- Phase 5-C: Nested clause generation for complex events, narrative register variation per world parameter

## Notes for Next Claude
- Player is not a programmer — explain decisions briefly, don't ask them to edit code
- The eschaton system touches many files but is well-contained: eschaton.rs for types, eschaton_gen.rs for logic, and hooks in sim/mod.rs tick() for tension + autonomous trigger
- Tension is a f32 that accumulates from events and decays slowly; displayed as percentage in UI
- execute_eschaton() in sim/mod.rs is the single entry point — handles both autonomous and player triggers
- The player triggers via Shift+E → EschatonConfirm overlay → execute_eschaton()
- Autonomous triggers happen in tick() after tension tracking, before era transition check
- Eschaton confirmation uses Left/Right arrows (not Up/Down) since buttons are side-by-side
- SESSION_NOTES.md should be fully rewritten each update, not appended to
