# SESSION NOTES — Last updated: 2026-03-06

## Current State
- Phase: Phase 5 COMPLETE + post-phase polish (inhabitants, room purposes, faction disbanding)
- Last working feature: Site inhabitants, inhabitant-adventurer interactions, room-purpose-aware prose, faction disbanding
- Build status: Compiles and runs cleanly (5 warnings, all pre-existing dead_code)
- Tests: All passing

## What We Did
- Added permanent site inhabitants (2-8 per site, generated at world creation)
  - `SiteInhabitant` struct in `site.rs` with name, description, glyph, floor, position
  - Inhabitants generated in `dungeon_gen.rs` with kind-specific names/descriptions/glyphs
  - Inhabitant types by site kind: creature (c), remnant (r), shrine attendant (s), bureaucrat (b), mourner (m), taxonomic anomaly (t), abandoned staff (a)
  - Inhabitants rendered as lowercase letters in lavender (180, 160, 200) in the site view
  - Inhabitant count shown in site list overlay
  - Site inhabitants added to map legend

- Added inhabitant-adventurer interaction system
  - New `InhabitantInteraction` event type with rust-colored log entries and ⌂ prefix
  - Every 5 ticks, 15% chance per agent at a site of interacting with a random inhabitant
  - Four interaction outcomes: ignored, questioned, assisted, driven out
  - Each outcome has 6 prose variants, all in the game's bureaucratic register
  - Interactions recorded in site history

- Incorporated room purposes into prose generation
  - New `generate_site_description_with_room()` function wraps the original with optional room purpose
  - Site entry/exit events now pick a room purpose from the agent's assigned room
  - Room purpose clauses for all 6 types (Storage, Ritual, Administrative, Habitation, Trophy, Disputed)
  - ~40% chance of appending room context to site entry/exit prose
  - Inhabitant interactions also reference room purpose ~35% of the time

- Added faction disbanding
  - Factions with 0 members AND power < 5 are now disbanded (not just dissolved)
  - New `FactionDisbanded` event type (teal-colored, ◆ prefix)
  - Dedicated prose generator with 8 variants, register-sensitive
  - Faction chronicle records the disbanding with tick number
  - Factions with 0 members but power >= 5 still dissolve with existing prose
  - FactionDisbanded counts as a major event for era tracking

## Decisions Made
- Inhabitants are serialized with `#[serde(default)]` for backward compatibility with old saves
- Inhabitants don't leave their site — they are permanent fixtures
- Inhabitant glyph color is uniform lavender to distinguish from agents (@)
- Room purpose is selected deterministically from agent_id % room_count for consistency
- Faction disbanding threshold: 0 members AND power < 5 (not just 0 members)

## Known Issues
- 5 compiler warnings (pre-existing, all dead_code)
- Unicode symbols require a terminal with good Unicode support

## Next Steps
- Further polish / new features as directed by player
- Distribution prep (cross-platform builds, cargo-bundle, Steam packaging) when ready

## Notes for Next Claude
- Player is not a programmer — explain decisions briefly, don't ask them to edit code
- All 5 phases complete plus post-phase polish
- next_agent_id lives on SimState (not saved — reconstructed from max agent ID)
- frame_count lives on SimState (not saved), updated from main loop each frame
- Speed keybindings: 0/1/2/3 (not the old 1/5/2 scheme)
- SESSION_NOTES.md should be fully rewritten each update, not appended to
- Overlay enum variant for faction detail: FactionDetail(usize, usize) = (institution index, scroll offset)
- wrap_text() and truncate_str() helpers live at the bottom of overlays.rs
- SiteInhabitant struct lives in site.rs, generated in dungeon_gen.rs
- Inhabitant interaction prose lives in prose_gen.rs (generate_inhabitant_interaction)
- Room purpose clauses live in prose_gen.rs (room_purpose_clause)
