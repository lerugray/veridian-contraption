# SESSION NOTES — Last updated: 2026-03-06

## Current State
- Phase: Phase 5 COMPLETE + post-phase polish (seasons, relationships)
- Last working feature: Agent relationship system
- Build status: Compiles and runs cleanly (5 warnings, all pre-existing dead_code)
- Tests: All passing

## What We Did
- Implemented a full agent relationship system
- **Relationship types**: Friend, Rival, Partner, Mentor/Protégé, Estranged
- **Formation**: Proximity-based (agents at same tile), every 20 ticks
  - Friends: compatible dispositions (similar loyalty/ambition), ~1.5% chance per co-located pair
  - Rivals: conflicting institutions or opposed dispositions, ~1% chance
  - Partners: high compatibility + rare roll (15% of eligible friend formations)
  - Mentor/Protégé: older agent (10+ years) sharing institution with younger agent
- **Intensity**: 1-3 scale, deepens over time based on relationship age
- **Evolution** (every 100 ticks):
  - Friends deepen (intensity++) or cool into Estranged
  - Partners deepen or dissolve into Estranged
  - Rivals intensify or reconcile into Friend (rare)
  - Mentor/Protégé deepen over time
- **Behavioral effects**:
  - Friends accompany each other to sites (nearby friend heading to site → chance to follow)
  - Partners: higher adventurer conversion if partner is in danger (exploring site)
  - Estranged agents flee settlements when the other is present
  - Protégés get 3x boosted institution join chance (mentor smooths path)
- **Visibility**:
  - Agent inspect overlay: RELATIONSHIPS section with name, type, intensity
  - Log entries: only notable agents (2+ institutions or 2+ epithets) generate log events
  - World Report (W): shows "Active relationships" count
- **Event types**: RelationshipFormed, RelationshipChanged (with prose generation)
- **Save/load**: Relationships serialize via serde with #[serde(default)] for old saves

## Files Modified This Session
- src/sim/agent.rs — RelationshipKind, Relationship struct, relationships field on Agent
- src/sim/event.rs — RelationshipFormed, RelationshipChanged event types
- src/sim/mod.rs — process_relationship_tick(), relationship_count(), mentor boost in institutional join, behavioral effects
- src/gen/prose_gen.rs — generate_relationship_event() with register-aware templates
- src/ui/overlays.rs — RELATIONSHIPS section in inspect overlay, relationship count in world report
- src/gen/world_gen.rs — relationships: Vec::new() in Agent initializers
- src/gen/eschaton_gen.rs — relationships: Vec::new() in Agent initializers

## Decisions Made
- Relationships stored per-agent (Vec<Relationship>) rather than a central table — simpler, serializes naturally
- Both sides of a relationship are stored (agent A has rel to B, B has rel to A) for fast lookup
- Formation checks run every 20 ticks; evolution every 100 ticks — balances performance with responsiveness
- Only notable agents generate log events (2+ institutions or 2+ epithets) to avoid log spam
- Proximity = same grid tile (agents must be co-located, not just nearby) for formation
- relationship_count() divides by 2 since each relationship is stored on both agents

## Known Issues
- 5 compiler warnings (pre-existing, all dead_code)
- Old saves will work fine — relationships default to empty Vec via serde(default)

## Next Steps
- Further polish / new features as directed by player
- Distribution prep when ready

## Notes for Next Claude
- Player is not a programmer — explain decisions briefly, don't ask them to edit code
- All 5 phases complete plus post-phase polish
- Relationship system: RelationshipKind/Relationship in agent.rs, process_relationship_tick() in mod.rs
- Speed keybindings: 0/1/2/3 (not the old 1/5/2 scheme)
- SESSION_NOTES.md should be fully rewritten each update, not appended to
