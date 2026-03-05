# SESSION NOTES — Last updated: Phase 1-F

## Current State
- Phase: 1-F (Main Menu & Save/Load) — COMPLETE
- Last working feature: Full main menu, new world screen, save/load, autosave, Ctrl+S save
- Files modified this session: main.rs, sim/mod.rs, export/mod.rs, ui/mod.rs, ui/menu.rs (new), ui/layout.rs, ui/overlays.rs

## What We Were Doing
Implemented Phase 1-F: main menu system, new world screen with flavor presets and seed input, save/load system with JSON serialization, autosave every 500 ticks, Ctrl+S manual save, load world screen, and status bar showing save name.

## Decisions Made
- AppMode enum in main.rs controls top-level state (MainMenu, NewWorld, LoadWorld, Generating, InGame)
- SaveData struct in sim/mod.rs handles serialization (separate from SimState since StdRng can't serialize directly — RNG reconstructed from seed+tick hash on load)
- Flavor presets are placeholder names only (full parametric implementation deferred to Phase 4 as designed)
- Seed input uses FNV-1a hash to convert strings to u64 seeds
- Save name input overlay (Overlay::SaveNameInput) added to the existing overlay system
- Autosave fires in the main game loop, not inside SimState, since it needs file I/O
- "Continue" option greyed out when no autosave.json exists

## Known Issues / In Progress
- Flavor presets don't affect world generation yet (by design — Phase 4)
- No delete-save functionality on the Load World screen yet
- No error display on menu screens if load fails (silently stays on current screen)
- 3 pre-existing warnings from Phase 1-E code (unused method ticks_per_frame, agents_at, field old_pos)

## Next Steps
- Phase 2: Institutions & Language (institution simulation, phoneme-based names, epithets, faction record, follow mode)

## Notes for Next Claude
- All Phase 1 prompts (1-A through 1-F) are complete. The game compiles and runs.
- The app launches to a main menu. Arrow keys navigate, Enter selects.
- New World screen lets you pick a flavor preset and enter a seed. Tab switches between preset selection and seed input.
- Ctrl+S in-game opens a save name prompt. Enter saves to /saves/{name}.json.
- Autosave writes to /saves/autosave.json every 500 ticks with a brief "~ autosaved" status message.
- Load World screen lists all .json files in /saves/ for selection.
- The status bar shows the save name (or "unsaved") alongside existing info.
