# SESSION NOTES — Last updated: 2026-03-05

## Current State
- Phase: Phase 1 COMPLETE (all prompts 1-A through 1-F done)
- Last working feature: Everything — main menu, world gen, agents, events, log, inspect, export, save/load, autosave
- Build status: Compiles and runs cleanly (3 pre-existing warnings only)

## What Was Done This Session
- Implemented Phase 1-F: main menu, new world screen, save/load system, autosave, Ctrl+S save, load world screen
- Fixed new world screen text cutoff (preset descriptions on separate lines)
- Slowed 1x speed (tick every 6th frame instead of 3rd)
- Fixed log scroll: view now stays pinned when scrolled up (event-based scrolling)
- Made map scale to fill panel (distributes extra rows/columns evenly)
- Restored speed controls in status bar (shortened labels to fit)

## Files Modified This Session
main.rs, sim/mod.rs, export/mod.rs, ui/mod.rs, ui/menu.rs (new), ui/layout.rs, ui/overlays.rs, CLAUDE.md, SESSION_NOTES.md

## Architecture Decisions
- AppMode enum in main.rs: MainMenu, NewWorld, LoadWorld, Generating, InGame
- SaveData struct separate from SimState (StdRng not serializable — RNG reconstructed from seed+tick)
- log_scroll is event-count based (not display lines) for stable scroll anchoring
- Map rendering scales tiles with distributed extra rows/columns to fill panel
- Flavor presets are placeholder names only (Phase 4 for full parametric implementation)

## Known Issues
- Flavor presets don't affect world generation yet (by design — Phase 4)
- No delete-save on Load World screen
- No error display if load fails (silently stays on current screen)
- 3 pre-existing warnings (unused ticks_per_frame, agents_at, old_pos field)

## Next Steps
- Phase 2: Institutions & Language
  - Institution simulation (factions, guilds, political entities)
  - Institutional events (schisms, elections, doctrinal disputes, diplomacy)
  - Phoneme-based name generation per people
  - Title and epithet accumulation for agents
  - Faction Record export
  - Follow mode (track agent or faction in side panel)

## Notes for Next Claude
- Player is not a programmer — explain decisions briefly, don't ask them to edit code
- The app launches to a main menu. New World lets you pick presets and enter a seed.
- Ctrl+S saves, autosave every 500 ticks to saves/autosave.json
- 1x speed ticks every 6th frame (~5 ticks/sec). 5x = 2 per frame. 20x = 20 per frame.
- Map is 60x30 tiles but renderer scales to fill panel dynamically
- All commits pushed to remote on main branch
