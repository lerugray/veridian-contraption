# SESSION NOTES — Last updated: 2026-03-06

## Current State
- Phase: Phase 2 COMPLETE
- Last working feature: Follow mode, faction record export, character chronicle export
- Build status: Compiles and runs cleanly (5 warnings — all pre-existing)

## What Was Done This Session
- Phase 2-C completion:
  - Reviewed all partial 2-C code (follow mode, prose gen improvements, follow panel)
  - Verified follow mode works: f=follow picker (agent or institution), F=faction list, follow panel shows filtered events
  - FollowTarget persisted in save/load via SaveData
  - Followed agent highlighted on map with 'X' marker
  - Added Faction Record export (option 2 in export menu): exports all institutions with charter, doctrine, members, chronicle, relationships
  - Added Character Chronicle export (option 3 in export menu): exports all living agents with epithets, affiliations, event history
  - Fixed status bar keybinding hints: now shows f=follow F=factions (was incorrectly showing f=factions)

## Files Modified This Session
- src/ui/layout.rs (status bar hint fix)
- src/ui/overlays.rs (export menu options 2 and 3)
- src/export/mod.rs (export_faction_record, export_character_chronicle, write_institution_record, sanitize_prefix)
- src/main.rs (export menu input handler for options 2 and 3)
- CLAUDE.md (phase status update)
- SESSION_NOTES.md

## Architecture Decisions
- Faction Record and Character Chronicle exports go directly to file (no filename prompt needed — auto-named with timestamp)
- Live Log export still prompts for filename prefix (existing behavior preserved)
- Export functions take &SimState to access agents, institutions, events, and world data

## Known Issues
- Flavor presets don't affect world generation yet (by design — Phase 4)
- No delete-save on Load World screen
- No error display if load fails (silently stays on current screen)
- 5 pre-existing warnings (ticks_per_frame, agents_at, old_pos, name_flavor unused, subordinate_clause name param unused)
- Character Chronicle export only shows events in the current ring buffer (last 200 events)
- Institutions load phoneme data each tick cycle — negligible cost since it's include_str but could cache

## Next Steps
- Phase 3: Dungeons, Artifacts & Adventurers
  - Dungeon generation: procedural sites with generated purpose and population
  - Artifact generation: objects with histories and properties
  - Adventurer class agents
  - Zoomed dungeon view

## Notes for Next Claude
- Player is not a programmer — explain decisions briefly, don't ask them to edit code
- Keybindings: f=follow, F(shift)=faction list, Tab=agent list, i=search, e=export, ^S=save, q=menu
- Follow mode: f toggles follow off if already following, otherwise opens picker (agent or institution)
- Export menu: 1=live log (prompts for name), 2=faction record (auto), 3=character chronicle (auto)
- All commits pushed to remote on main branch
