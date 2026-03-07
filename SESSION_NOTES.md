# SESSION NOTES — Last updated: 2026-03-06

## Current State
- Phase: Phase 5 COMPLETE + post-phase polish
- Last working feature: Help and legend overlays now work in site view
- Build status: Compiles and runs cleanly (5 warnings, all pre-existing dead_code)
- Tests: All passing

## What We Did
- Fixed help (?) and legend (l) keybindings not responding when viewing a site floor plan
  - Added `pre_overlay` field (`Option<Box<Overlay>>`) to SimState to track the overlay to return to
  - `handle_site_view_input` in main.rs now handles `?` and `l`, saving the SiteView state to `pre_overlay`
  - Help and MapLegend close handlers restore from `pre_overlay` if set, otherwise return to Overlay::None
  - Layout rendering checks `pre_overlay` so the site floor plan stays visible behind Help/MapLegend overlays
  - Status bar also checks `pre_overlay` to keep showing site info while overlays are open

## Decisions Made
- Used `pre_overlay: Option<Box<Overlay>>` on SimState rather than modifying Overlay enum variants to carry return info — simpler, less invasive
- `pre_overlay` is not serialized (not saved) — it's transient UI state only

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
- pre_overlay field on SimState stores the overlay to return to when closing Help/MapLegend from site view
