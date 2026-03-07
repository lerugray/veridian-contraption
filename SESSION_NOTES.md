# SESSION NOTES — Last updated: 2026-03-06

## Current State
- Phase: Phase 5 COMPLETE + post-phase polish
- Last working feature: Settlement floor plans viewable from site list
- Build status: Compiles and runs cleanly (5 warnings, all pre-existing dead_code)
- Tests: All passing

## What We Did
- Added viewable settlement floor plans, accessible from the site list (S key)
- Settlements are now listed alongside sites in the unified site list overlay
- Each settlement generates a floor plan at world creation, sized by settlement class
- Settlement floor plans show civic buildings (Tavern, Market, Temple, Administrative, Residential, Warehouse, Garrison)
- Buildings connected by wider streets; towns/cities get extra cross-connections and a well/fountain
- Agents at a settlement's coordinates appear as @ symbols in the floor plan
- ESC from settlement/site view returns to site list (not main view)
- Help (?) and legend (l) overlays work from settlement view via pre_overlay

## Files Modified This Session
- src/sim/site.rs — Added civic RoomPurpose variants (Tavern, Market, Temple, Residential, Warehouse, Garrison)
- src/sim/world.rs — Added `floor: Option<Floor>` to Settlement struct
- src/sim/mod.rs — Added `SettlementView(usize)` to Overlay enum
- src/gen/dungeon_gen.rs — Added `generate_settlement_floor()` and `carve_street()` functions
- src/gen/world_gen.rs — Settlement floor plans generated during world creation
- src/gen/eschaton_gen.rs — New settlements from eschaton events get floor plans too
- src/main.rs — Added `handle_settlement_view_input()`, updated site list input for combined list, sim ticks during settlement view
- src/ui/layout.rs — Added `draw_settlement_panel()`, settlement view detection for map panel and status bar
- src/ui/overlays.rs — Rewrote `draw_site_list()` to show settlements first, then sites, with type-appropriate details

## Decisions Made
- Settlement floor uses `Option<Floor>` with `#[serde(default)]` for backwards compatibility with old saves
- Settlements listed before sites in the unified list; settlements use warm color (Rgb 230,210,160), sites keep their kind colors
- Settlement border color is warm lantern (Rgb 230,210,160) to feel civic rather than dangerous
- Site list title changed to "SETTLEMENTS & SITES", border color warm gold
- Street generation uses wider (2-tile) corridors, extra cross-connections for towns/cities
- ESC from both site view and settlement view now returns to site list (was returning to None for sites)

## Known Issues
- 5 compiler warnings (pre-existing, all dead_code)
- Old saves without settlement floor plans will show "No floor plan available" — works but no floor plan generated retroactively

## Next Steps
- Further polish / new features as directed by player
- Distribution prep when ready

## Notes for Next Claude
- Player is not a programmer — explain decisions briefly, don't ask them to edit code
- All 5 phases complete plus post-phase polish
- Settlement floor plan: `Option<Floor>` on Settlement, generated via `dungeon_gen::generate_settlement_floor()`
- Site list is now a unified list: settlements first (indices 0..settlement_count), sites after (indices settlement_count..total)
- SettlementView(usize) overlay variant — single floor, no floor navigation keys
- pre_overlay field on SimState stores the overlay to return to when closing Help/MapLegend from settlement/site view
- Speed keybindings: 0/1/2/3 (not the old 1/5/2 scheme)
- SESSION_NOTES.md should be fully rewritten each update, not appended to
