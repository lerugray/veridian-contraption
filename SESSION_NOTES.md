# SESSION NOTES — Last updated: 2026-03-06

## Current State
- Phase: Phase 3 IN PROGRESS (3-A complete, pre-3-B fix done)
- Last working feature: Agents now enter/leave sites during simulation
- Build status: Compiles and runs cleanly (7 warnings, all pre-existing or future-use)

## What Was Done This Session
- Read GDD.md, SESSION_NOTES.md, CLAUDE.md
- Fixed known issue: agents now enter and leave sites during simulation
  - Added `Goal::SeekSite(usize)` and `Goal::ExploreSite(usize, u32)` to agent goals
  - Agents with risk_tolerance > 0.4 have a chance to seek out sites via `maybe_change_goal`
  - `agent.act()` now accepts `site_positions` param; handles movement to sites, entry, and exit
  - Site populations synced every tick by rebuilding from agents with `ExploreSite` goals
  - New `EventType::AgentEnteredSite` / `AgentLeftSite` (colored red)
  - `generate_site_description()` in prose_gen.rs — 7 templates each for entry/exit
  - Agent inspect view shows "Heading to [site]" / "Exploring [site] (N ticks remaining)"
- Added `## Known Issues` section to CLAUDE.md with instruction to flag items at session start

## Files Modified This Session
- src/sim/agent.rs (SeekSite/ExploreSite goals, site_positions param, maybe_change_goal update)
- src/sim/event.rs (AgentEnteredSite, AgentLeftSite event types)
- src/sim/mod.rs (site positions built in tick(), population sync, site-specific prose routing)
- src/gen/prose_gen.rs (generate_site_description(), fallback arms for new event types)
- src/ui/overlays.rs (inspect view handles new goal variants)
- CLAUDE.md (Known Issues section added)
- SESSION_NOTES.md

## Architecture Decisions
- Site population is rebuilt every tick from agent goals rather than tracking individual add/remove — simpler, no desync risk
- `agent.act()` signature expanded to include `site_positions: &[(u32, u32)]` — consistent with how `settlements` are passed
- Agents explore sites for 20-80 ticks before leaving — enough to show up in the site view

## Known Issues
- Room purposes not yet referenced in prose generation (Phase 3-B scope)
- Artifacts not yet implemented (Phase 3-B scope)
- Flavor presets don't affect world generation yet (Phase 4)
- No delete-save on Load World screen (minor QOL)
- 7 compiler warnings (pre-existing + future-use methods)

## Next Steps
- Phase 3-B: Artifacts & Adventurer Agents
  - Artifact generation: objects with histories and properties
  - Adventurer class agents who pursue dungeon-delving goals
  - Character Chronicle export per-agent
  - Room purposes referenced in prose

## Notes for Next Claude
- Player is not a programmer — explain decisions briefly, don't ask them to edit code
- Keybindings: SPACE=pause, .=step, 1/5/2=speed, f=follow, F=faction list, Tab=agent list, i=search, s=sites, e=export, ?=help, ^S=save, q=menu
- Site view: s opens site list, Enter enters site view, </> navigate floors, ESC returns to map
- Sites are in sim.sites (Vec<Site>), indexed directly (not by ID lookup)
- agent.act() now takes 4 params: rng, terrain, settlement_positions, site_positions
- maybe_change_goal() now takes 3 params: rng, settlements, site_positions
- Site population sync happens in tick() after agent action loop, before weather events
- CLAUDE.md now has a Known Issues section — check it at session start and flag items to player
- All commits pushed to remote on main branch
