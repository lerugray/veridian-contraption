# SESSION NOTES — Last updated: 2026-03-06

## Current State
- Phase: Phase 5 COMPLETE + post-phase polish (seasons, relationships, inspect fixes)
- Last working feature: Inspect overlay scroll capping
- Build status: Compiles and runs cleanly (5 warnings, all pre-existing dead_code)
- Tests: All passing

## What We Did
- Implemented a full agent relationship system
- Fixed inspect overlay chronicle display (word-wrap, scrolling, scroll bounds)

### Relationship System
- **Types**: Friend, Rival, Partner, Mentor/Protégé, Estranged (with intensity 1-3)
- **Formation**: Proximity-based (agents at same tile), every 20 ticks
  - Friends: compatible dispositions (~1.5% chance per co-located pair)
  - Rivals: conflicting institutions or opposed dispositions (~1%)
  - Partners: high compatibility + rare roll
  - Mentor/Protégé: older agent (10+ years) sharing institution with younger
- **Evolution** (every 100 ticks): friendships deepen/cool, partners deepen/dissolve, rivals intensify/reconcile
- **Behavioral effects**: friends accompany to sites, partners boost adventuring, estranged flee, protégés get 3x join boost
- **Visibility**: inspect overlay RELATIONSHIPS section, world report count, log events for notable agents only
- **Event types**: RelationshipFormed, RelationshipChanged
- **Save compat**: #[serde(default)] for old saves

### Inspect Overlay Fixes
- Chronicle entries word-wrap at word boundaries (word_wrap helper function)
- Inspect overlay is scrollable (Up/Down/PgUp/PgDn)
- Scroll capped so it stops at last entry (no empty space past end)
- InspectAgent overlay variant now stores (agent_idx, scroll_offset)

## Files Modified This Session
- src/sim/agent.rs — RelationshipKind, Relationship struct, relationships field
- src/sim/event.rs — RelationshipFormed, RelationshipChanged event types
- src/sim/mod.rs — process_relationship_tick(), relationship_count(), InspectAgent(usize, usize)
- src/gen/prose_gen.rs — generate_relationship_event()
- src/gen/world_gen.rs — relationships: Vec::new() in Agent initializers
- src/gen/eschaton_gen.rs — relationships: Vec::new() in Agent initializers
- src/ui/overlays.rs — RELATIONSHIPS in inspect, word_wrap(), scroll support, scroll capping
- src/ui/layout.rs — InspectAgent(idx, scroll) destructuring
- src/main.rs — handle_inspect_input with scroll keys, InspectAgent(idx, 0) calls

## Decisions Made
- Relationships stored per-agent (both sides) for fast lookup
- Formation every 20 ticks, evolution every 100 ticks
- Only notable agents (2+ institutions or 2+ epithets) generate log events
- word_wrap() breaks at whitespace boundaries, handles words longer than width
- Scroll capping done in renderer (max_scroll = lines.len() - inner_height)

## Known Issues
- 5 compiler warnings (pre-existing, all dead_code)
- Old saves work fine — relationships default to empty Vec

## Next Steps
- Further polish / new features as directed by player
- Distribution prep when ready

## Notes for Next Claude
- Player is not a programmer — explain decisions briefly, don't ask them to edit code
- All 5 phases complete plus post-phase polish
- InspectAgent overlay is now InspectAgent(usize, usize) — (agent_idx, scroll_offset)
- Speed keybindings: 0/1/2/3 (not the old 1/5/2 scheme)
- SESSION_NOTES.md should be fully rewritten each update, not appended to
