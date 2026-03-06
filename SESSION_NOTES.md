# SESSION NOTES — Last updated: 2026-03-06

## Current State
- Phase: Phase 2 IN PROGRESS (2-A complete)
- Last working feature: Phoneme-based name generation, epithet system, 4 QOL fixes
- Build status: Compiles and runs cleanly (7 warnings — 3 pre-existing + 4 new unused items for Phase 2)

## What Was Done This Session
- 4 QOL fixes: Tab=agent list overlay, i=search with selectable results, log freeze while scrolled, Q=return to menu with save prompt
- Phase 2-A: Phoneme-based name generation and epithet system
  - Created data/phonemes.json with 4 distinct phoneme sets (Guttural, Sibilant, Nasal, Compound)
  - Rewrote gen/name_gen.rs: generate_personal_name, generate_settlement_name, generate_world_name, generate_institution_name (ready for Phase 2-B), generate_epithet
  - Each People gets a phoneme_set index — names within a culture sound consistent
  - Agents accumulate epithets from events (50-100 tick cooldown per agent)
  - Agent.display_name() returns "Name the Epithet" — used in log entries
  - Inspect overlay shows all epithets with current one highlighted
  - InstitutionKind enum defined in name_gen.rs (placeholder for Phase 2-B)

## Files Modified This Session
main.rs, sim/mod.rs, sim/agent.rs, sim/world.rs, gen/name_gen.rs, gen/world_gen.rs, ui/layout.rs, ui/overlays.rs, data/phonemes.json, CLAUDE.md, SESSION_NOTES.md

## Architecture Decisions
- Phoneme data embedded via include_str! (no runtime file I/O needed)
- PhonemeSet has onset/nucleus/coda/syllable_patterns — syllable generation uses pattern strings like "CVC", "CV", "VC"
- People.phoneme_set indexes into the 4 phoneme sets (0=Guttural, 1=Sibilant, 2=Nasal, 3=Compound)
- Compound set (index 3) generates hyphenated names: "Durr-Pelm", "Krev-On"
- Epithets stored as Vec<String> on Agent, last_epithet_tick tracks cooldown
- New fields use #[serde(default)] for backwards compatibility with old saves
- Log freeze uses log_frozen_len: Option<usize> — snapshots event count on first scroll, clears when scroll returns to 0
- Q key now shows QuitConfirm overlay (save & return / return without saving / cancel) instead of exiting
- InputResult::ReturnToMenu variant transitions from InGame back to MainMenu

## Known Issues
- Flavor presets don't affect world generation yet (by design — Phase 4)
- No delete-save on Load World screen
- No error display if load fails (silently stays on current screen)
- InstitutionKind, generate_institution_name, pick() show unused warnings (Phase 2-B will use them)
- name_flavor field on PhonemeSet is unused (debug/display only, could be shown in world params view)

## Next Steps
- Phase 2-B: Institution simulation
  - Create sim/institution.rs with Institution struct, InstitutionKind (move from name_gen)
  - Institutional events (schisms, elections, doctrinal disputes, diplomacy)
  - Agent affiliation with institutions
  - Faction Record export
  - Follow mode (track agent or faction in side panel)

## Notes for Next Claude
- Player is not a programmer — explain decisions briefly, don't ask them to edit code
- The app launches to a main menu. New World lets you pick presets and enter a seed.
- Ctrl+S saves, autosave every 500 ticks to saves/autosave.json
- Q returns to menu (not quit) — shows save prompt first
- Tab opens agent list overlay, i opens search with selectable results
- 1x speed ticks every 6th frame (~5 ticks/sec). 5x = 2 per frame. 20x = 20 per frame.
- Map is 60x30 tiles but renderer scales to fill panel dynamically
- Names are now phoneme-based: 4 sets mapped to peoples. Agents get names from their people's set.
- Epithets accumulate on agents every 50-100 ticks when events trigger them
- All commits pushed to remote on main branch
