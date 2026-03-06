# SESSION NOTES — Last updated: 2026-03-06

## Current State
- Phase: Phase 3 IN PROGRESS (3-A complete)
- Last working feature: Site/dungeon generation, site list overlay, site floor view
- Build status: Compiles and runs cleanly (7 warnings, all pre-existing or future-use)

## What Was Done This Session
- Phase 3-A: Dungeon/Site Generation & Site View
  - Created `src/sim/site.rs`: Site, Floor, SiteKind (7 kinds), Tile (7 types), Room, RoomPurpose
  - Created `src/gen/dungeon_gen.rs`: site placement, floor generation (room-and-corridor), naming, origin stories
  - 7 site kinds: Dungeon, Ruin, Shrine, BureaucraticAnnex, ControversialTombsite, TaxonomicallyAmbiguousRegion, AbandonedInstitution
  - Floor generation: 40x20 tile grids, 4-8 rooms per floor, L-shaped corridors, doors, stairs, water/pit hazards
  - Dungeons: 2-4 floors guaranteed; Ruins: 1-2 floors; others: 1 floor
  - First 2 sites per world are always Dungeons (guaranteed multi-floor content)
  - ~40% of sites assigned a controlling faction from existing institutions
  - Sites appear as Ω on world map, colored by kind
  - Site list overlay (s key): browsable, shows kind/location/faction/origin, proper scrolling
  - Site view: replaces map panel with ASCII floor plan, </> to navigate floors, ESC to return
  - Agents in sites rendered as @ with people color
  - Unit test: `dungeon_floors_are_2_to_4` verifies floor counts across 20 seeds
  - Sites integrated into SaveData for save/load persistence
  - Status bar shows site name when in site view; help screen updated with s keybinding

## Files Modified This Session
- src/sim/site.rs (NEW — Site, Floor, Tile, SiteKind, Room, RoomPurpose)
- src/gen/dungeon_gen.rs (NEW — generate_sites, generate_floor, naming, origins, test)
- src/sim/mod.rs (added site module, Site to SimState/SaveData, SiteList/SiteView overlays)
- src/gen/mod.rs (added dungeon_gen module)
- src/gen/world_gen.rs (calls dungeon_gen::generate_sites, returns sites tuple)
- src/ui/layout.rs (site Ω on world map, draw_site_panel, site view status bar)
- src/ui/overlays.rs (draw_site_list with proper scrolling, help screen updated)
- src/main.rs (s keybinding, site list/view input handlers, updated world gen call)
- CLAUDE.md (phase status update)
- SESSION_NOTES.md

## Architecture Decisions
- Sites are a top-level entity in SimState alongside agents/institutions (not nested in World)
- Floor generation uses room-and-corridor algorithm with L-shaped corridors
- First 2 sites are always Dungeons to guarantee multi-floor content for the player
- Site view replaces the map panel (not a popup overlay) — left pane shows floor, right pane still shows log
- Floor navigation uses < and > keys (or , and . since shift isn't always available)
- Each room has a RoomPurpose — not yet used in prose but ready for Phase 3-B

## Known Issues
- Agents don't actually enter/leave sites yet (population list exists but isn't populated during sim)
- Room purposes not yet referenced in prose generation
- Artifacts not yet implemented (Vec<u64> field exists but empty)
- Flavor presets don't affect world generation yet (by design — Phase 4)
- No delete-save on Load World screen
- 7 warnings (pre-existing + walkable/label methods reserved for future use)

## Next Steps
- Phase 3-B: Artifacts & Adventurer Agents
  - Artifact generation: objects with histories and properties
  - Adventurer class agents who pursue dungeon-delving goals
  - Agents entering/leaving sites
  - Character Chronicle export per-agent from site/dungeon context

## Notes for Next Claude
- Player is not a programmer — explain decisions briefly, don't ask them to edit code
- Keybindings: SPACE=pause, .=step, 1/5/2=speed, f=follow, F=faction list, Tab=agent list, i=search, s=sites, e=export, ?=help, ^S=save, q=menu
- Site view: s opens site list, Enter enters site view, </> navigate floors, ESC returns to map
- Sites are in sim.sites (Vec<Site>), indexed directly (not by ID lookup)
- The test in dungeon_gen.rs verifies floor counts — run with `cargo test -- dungeon_floors`
- All commits pushed to remote on main branch
