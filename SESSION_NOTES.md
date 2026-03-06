# SESSION NOTES — Last updated: 2026-03-06

## Current State
- Phase: Phase 3 IN PROGRESS (3-A complete, 3-B complete)
- Last working feature: Artifacts and adventurer agents
- Build status: Compiles and runs cleanly (7 warnings, all pre-existing or future-use)

## What Was Done This Session
- Fixed three bugs before starting 3-B:
  1. Agents now appear visually on site floor plans (sim ticks during SiteView)
  2. Time continues ticking while viewing a site floor plan
  3. Pause/unpause preserves previous speed instead of resetting to 1x
  - Added `pre_pause_speed` field to SimState
  - Added speed controls (SPACE, 1/5/2, .) to site view input handler

- Implemented Phase 3-B: Artifacts & Adventurer Agents
  - Created `src/sim/artifact.rs` — Artifact, ArtifactKind, ArtifactLocation types
  - Created `src/gen/artifact_gen.rs` — Artifact name/material/property generation
  - 8-20 artifacts generated per world, distributed ~60% sites, ~30% settlements, ~10% lost
  - Artifact names are evocative bureaucratic-flavored (e.g. "The Seventh Register of the Pelmwick Compact")
  - 5-10 adventurer agents generated per world with high risk/ambition, low loyalty
  - Adventurers seek artifacts in sites, acquire them, return them to settlements
  - Adventurers have ~3% chance of death per tick while at a site
  - New event types: ArtifactAcquired, ArtifactDelivered, AdventurerDiedInSite
  - Prose generation for all artifact events and adventurer deaths
  - Epithet generation for artifact events
  - Agent inspect overlay shows held artifacts and adventurer flag
  - Site list shows artifact count per site
  - Character chronicle export includes held artifacts and adventurer role
  - New agent goals: AcquireArtifact(artifact_id, site_idx), ReturnArtifact(artifact_id, settlement_idx)

## Files Modified This Session
- src/sim/artifact.rs (NEW — Artifact types)
- src/gen/artifact_gen.rs (NEW — artifact generation)
- src/sim/mod.rs (artifacts in SimState/SaveData, process_adventurer_tick, pre_pause_speed)
- src/sim/agent.rs (is_adventurer, held_artifacts, AcquireArtifact/ReturnArtifact goals)
- src/sim/event.rs (ArtifactAcquired, ArtifactDelivered, AdventurerDiedInSite)
- src/gen/mod.rs (artifact_gen module)
- src/gen/world_gen.rs (generates artifacts and adventurer agents)
- src/gen/prose_gen.rs (artifact event prose, adventurer death prose)
- src/gen/name_gen.rs (artifact-related epithets)
- src/ui/overlays.rs (artifact/adventurer info in inspect view, site list artifact counts)
- src/ui/layout.rs (no changes this session)
- src/export/mod.rs (artifact info in character chronicle export)
- src/main.rs (site view speed controls, pre_pause_speed, 5-tuple from generate_world)
- CLAUDE.md (phase progress updated)
- SESSION_NOTES.md

## Architecture Decisions
- Artifacts are a top-level Vec<Artifact> in SimState, not nested in sites/settlements
- ArtifactLocation enum tracks where each artifact is (InSite, HeldByAgent, InSettlement, Lost)
- Adventurers are regular agents with `is_adventurer: bool` flag and biased dispositions, not a separate struct
- Artifact simulation runs in `process_adventurer_tick()` called from `tick()`, after agent actions but before weather
- `generate_world` now returns a 5-tuple including artifacts

## Known Issues
- Room purposes not yet referenced in prose generation
- Flavor presets don't affect world generation yet (Phase 4)
- No delete-save on Load World screen (minor QOL)
- 7 compiler warnings (pre-existing + future-use methods)
- Old save files will load but won't have artifacts (serde defaults handle this gracefully)

## Next Steps
- Phase 3 polish: room purposes in prose, more artifact interactions
- Phase 4: World Parametric Variance & Multiple Worlds

## Notes for Next Claude
- Player is not a programmer — explain decisions briefly, don't ask them to edit code
- Keybindings: SPACE=pause, .=step, 1/5/2=speed, f=follow, F=faction list, Tab=agent list, i=search, s=sites, e=export, ?=help, ^S=save, q=menu
- Site view: s opens site list, Enter enters site view, </> navigate floors, ESC returns to map; speed controls work in site view
- generate_world() now returns (World, Vec<Agent>, Vec<Institution>, Vec<Site>, Vec<Artifact>)
- SimState::new() takes 5 params: world, agents, institutions, sites, artifacts
- Artifacts tracked in sim.artifacts (Vec<Artifact>), site.artifacts (Vec<u64> of art IDs at site)
- agent.is_adventurer marks adventurer agents; agent.held_artifacts tracks carried items
- process_adventurer_tick() handles artifact acquisition/delivery/death logic
- pre_pause_speed stores the speed before pausing so unpause restores it
- All commits pushed to remote on main branch
