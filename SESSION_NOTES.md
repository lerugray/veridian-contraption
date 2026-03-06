# SESSION NOTES — Last updated: 2026-03-06

## Current State
- Phase: Phase 2 IN PROGRESS (2-B complete)
- Last working feature: Institution simulation layer with founding, schisms, doctrine shifts, alliances, rivalries, member join/depart/expel
- Build status: Compiles and runs cleanly (4 warnings — all pre-existing)

## What Was Done This Session
- Phase 2-B: Full institutional simulation layer
  - Created sim/institution.rs: Institution struct, InstitutionKind (Guild, Government, Cult, MercenaryCompany, RegulatoryBody, SecretSociety), InstitutionRelationship (Allied, Neutral, Rival, Disputed)
  - Moved InstitutionKind from name_gen.rs placeholder to sim/institution.rs (canonical location)
  - Updated name_gen.rs: generate_charter(), generate_actual_function(), generate_doctrines() for each institution kind
  - World generation now creates 4-8 starting institutions distributed across peoples, each with 3-8 member agents
  - Agents have institution_ids field (0-2 affiliations), backwards-compatible with old saves via #[serde(default)]
  - New agent goals: JoinInstitution, AdvanceInInstitution, FoundInstitution (high-ambition only)
  - Agent maybe_change_goal() now considers institutional goals based on ambition/loyalty
  - 10 new EventTypes: InstitutionFounded, InstitutionDissolved, SchismOccurred, DoctrineShifted, AllianceFormed, AllianceStrained, RivalryDeclared, MemberJoined, MemberDeparted, MemberExpelled
  - Full prose generation for all institutional events in prose_gen.rs (generate_institutional_description)
  - Institutional epithets for founders, joiners, expelled, departed agents
  - Simulation tick processes: agent founding/joining goals (every 10 ticks), unaffiliated agent recruitment, institutional events (every 75 ticks), relationship events (every 150 ticks), member departure (every 80 ticks), dissolution of empty institutions (every 200 ticks)
  - Schisms split institutions: create a new institution with half the members, set mutual Rival relationship
  - 'f' key opens Faction List overlay: shows all institutions with kind, power, member count, summary, and relationships
  - Agent inspect overlay now shows AFFILIATIONS section with institution names and kinds
  - Agent list (Tab) now shows epithets and institution affiliations inline
  - Institutions included in SaveData for save/load with #[serde(default)] backwards compat

## Files Modified This Session
sim/institution.rs (NEW), sim/mod.rs, sim/agent.rs, sim/event.rs, gen/name_gen.rs, gen/world_gen.rs, gen/prose_gen.rs, ui/overlays.rs, ui/layout.rs, main.rs, CLAUDE.md, SESSION_NOTES.md

## Architecture Decisions
- Institutions live in SimState.institutions Vec, indexed by position but identified by u64 id
- next_institution_id counter on SimState for safe ID allocation (survives save/load)
- InstitutionKind is in sim/institution.rs; gen/name_gen.rs imports it
- process_institutional_tick() is a separate method on SimState to keep tick() clean
- Institutional events spread across different tick intervals to avoid all happening at once
- phonemes loaded fresh in process_institutional_tick() (via include_str, so no I/O cost)
- Institution.charter vs actual_function: 40% chance they diverge at founding

## Known Issues
- Flavor presets don't affect world generation yet (by design — Phase 4)
- No delete-save on Load World screen
- No error display if load fails (silently stays on current screen)
- 4 pre-existing warnings (ticks_per_frame, agents_at, old_pos, name_flavor unused)
- Institutions load phoneme data each tick cycle — negligible cost since it's include_str but could cache

## Next Steps
- Phase 2-C: Faction Record export and Follow mode
  - Follow mode: track specific agent or faction in side panel
  - Faction Record export to TXT
  - Character Chronicle export

## Notes for Next Claude
- Player is not a programmer — explain decisions briefly, don't ask them to edit code
- The app launches to a main menu. New World generates 4-8 institutions at world start.
- 'f' opens faction list, Tab opens agent list, 'i' opens search
- Institutions tick on different intervals: 10 (goals), 75 (inst events), 80 (departures), 150 (relationships), 200 (dissolution)
- Schisms produce a new institution from half the members, set as mutual rivals
- Agent.institution_ids holds 0-2 institution IDs; agents with high loyalty/ambition pursue institutional goals
- All commits pushed to remote on main branch
