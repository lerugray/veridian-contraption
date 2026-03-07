# SESSION NOTES — Last updated: 2026-03-06

## Current State
- Phase: Phase 5 COMPLETE + post-phase polish
- Last working feature: Distinct visual floor plans for all 7 site types
- Build status: Compiles and runs cleanly (5 warnings, all pre-existing dead_code)
- Tests: All passing

## What We Did
- Gave each of the 7 site types a unique floor plan generation algorithm and color palette
- **Dungeon**: Deep grey walls (60,55,50), very dim floors (35,32,28), more water/pit hazards that stand out
- **Ruin**: Partially collapsed — 25% walls become rubble (%), 15% floors become open sky, large sky patches carved out
- **Shrine**: Symmetrical layout with central focal point (*), cardinal approach corridors, small side chambers, warm candlelight colors
- **Bureaucratic Annex**: Grid-like room layout (2-3 rows x 3-4 cols), central cross corridors, cool grey/pale blue palette, office room labels
- **Controversial Tombsite**: Central tomb chamber with focal point sarcophagus, ring of burial niches (°) around it, mourning hall at entrance
- **Taxonomically Ambiguous Region**: Cellular automata organic caves with OrganicWall (▓), mixed tile types (water, ground, floor), no straight walls
- **Abandoned Institution**: Annex grid structure with damage pass — 20% walls broken, 8% floor debris, 30% doors broken, 1-2 rooms blocked off
- Added 5 new Tile variants: Rubble, OpenSky, FocalPoint, Niche, OrganicWall
- Added 7 new RoomPurpose variants for site-specific rooms (FilingRoom, WaitingArea, ProcessingDesk, ArchiveVault, TombChamber, BurialNiche, MourningHall, FormerOffice, CollapsedWing)
- Site panel rendering now uses `site_palette()` and `site_tile_color()` for per-kind color mapping
- Room labels displayed inside site floors for kinds that support them (Dungeon, Shrine, Annex, Tombsite, Abandoned)

## Files Modified This Session
- src/sim/site.rs — New Tile variants, new RoomPurpose variants, labels
- src/gen/dungeon_gen.rs — Per-kind floor generators (7 functions), dispatch via `generate_site_floor()`
- src/ui/layout.rs — `site_palette()`, `site_tile_color()`, labels in site panel

## Decisions Made
- Each site kind has its own generation function rather than parameterizing the base dungeon generator — cleaner and more flexible
- Ruin and Abandoned Institution both use a "generate then damage" approach
- Abandoned Institution reuses the Annex generator then applies decay
- Taxonomically Ambiguous Region uses cellular automata (B5678/S45678) for organic cave shapes
- Shrine uses explicit symmetrical construction rather than random room placement
- Tombsite uses polar coordinate ring for burial niches around central tomb
- Room labels are opt-in per site kind via `show_labels` in palette

## Known Issues
- 5 compiler warnings (pre-existing, all dead_code)
- Old saves without new tile types will still render correctly (serde default handles missing variants)
- Old saves may show old-style dungeon layouts for non-dungeon sites (generated before this change)

## Next Steps
- Further polish / new features as directed by player
- Distribution prep when ready

## Notes for Next Claude
- Player is not a programmer — explain decisions briefly, don't ask them to edit code
- All 5 phases complete plus post-phase polish
- `generate_site_floor()` dispatches to 7 kind-specific generators in dungeon_gen.rs
- `site_palette()` and `site_tile_color()` in layout.rs handle per-kind rendering
- Speed keybindings: 0/1/2/3 (not the old 1/5/2 scheme)
- SESSION_NOTES.md should be fully rewritten each update, not appended to
