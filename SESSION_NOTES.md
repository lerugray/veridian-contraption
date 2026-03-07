# SESSION NOTES — Last updated: 2026-03-07

## Current State
- Phase: Phase 5 COMPLETE + combat system + inspect overlay word-wrap fix
- Last working feature: word_wrap applied to all free-form text in inspect overlay
- Build status: Compiles cleanly (6 warnings, all dead_code / unused fields)

## What We Did This Session

### Combat System Implementation
Created a complete simulation-driven combat system:

**New file: src/sim/combat.rs**
- `InjuryStatus` enum: Uninjured, Bruised, Wounded, GravelyWounded (with serde Default)
- `CombatExperienceTier` enum: Untested, Blooded, Seasoned, Dangerous (from combats_survived count)
- `combat_weight()`: hidden stat from risk_tolerance (0.6 weight), age curve (peaks 20-45, declines past 45), experience bonus, injury penalty
- `resolve_combat()`: roll-based with +/-0.15 random variance, draw threshold 0.05, injury severity scaled by margin
- `injury_prose()`: prose descriptions for inspect screen, varies by severity and recovery progress
- `CombatExperienceTier::prose_description()`: 3-4 prose options per tier, no raw stats exposed

**Agent changes (src/sim/agent.rs):**
- New fields (all `#[serde(default)]`): `injury`, `recovery_remaining`, `combats_survived`, `last_combat_tick`
- New goal: `SeekSettlementForHealing(usize)` — wounded/gravely wounded agents seek nearest settlement
- Injury recovery: 1 tick per tick, auto-heals to Uninjured when recovery_remaining hits 0
- Gravely wounded death chance: 0.3% per tick when not at a settlement
- Wounded movement: moves every other tick when SeekSettlementForHealing
- `maybe_change_goal()`: wounded agents automatically seek settlement after current goal completes

**Combat triggers (src/sim/mod.rs — process_combat_tick):**
- Runs every 5 ticks, checks all agent pairs sharing a tile
- Rival agents: 1-3% per check (scaled by rivalry intensity)
- Opposing faction members (Rival institutional relationship): 0.5% per check
- Volatile agents (risk_tolerance > 0.8, institutional_loyalty < 0.3): 0.2% per check
- 20-tick cooldown per agent between fights
- Max 2 combats per tick to prevent spam
- Only logs combat for notable agents (1+ epithets or 2+ institutions)

**Prose (src/gen/prose_gen.rs):**
- `gen_combat_indexed()`: 10 templates with register-sensitive verbs/nouns, draw variants (4), win variants (10)
- Injury clauses appended for Wounded/GravelyWounded outcomes
- Template suppression via `combat_template_history` (30-tick window)

### Inspect Overlay Word-Wrap Fix
Applied `word_wrap()` consistently to all free-form text in the inspect overlay:
- Moved `inner_width` computation to top of function (was only computed for Chronicle)
- **Goal** line: wrapped with " Goal: " prefix, continuation lines indented to match
- **Relationships** section: name + kind + intensity wrapped as single string
- **Conversations** section: header (tick + name + tone) and both dialogue lines wrapped
- **CONDITION** section: injury prose and experience prose wrapped
- Chronicle already had wrapping (no change needed)
- Epithets, affiliations, disposition bars, and short label lines don't need wrapping

## Files Modified This Session
- src/sim/combat.rs — NEW: combat types, resolution, prose helpers
- src/sim/agent.rs — injury/experience fields, SeekSettlementForHealing goal, recovery logic
- src/sim/event.rs — CombatOccurred variant
- src/sim/mod.rs — combat module, process_combat_tick, template suppression, tension/era tracking
- src/gen/prose_gen.rs — gen_combat_indexed, CombatOccurred in generate_description
- src/gen/name_gen.rs — combat epithets
- src/gen/world_gen.rs — new agent fields in constructors
- src/gen/eschaton_gen.rs — new agent fields in constructor
- src/ui/overlays.rs — CONDITION section, word-wrap on all prose sections, removed raw health
- src/export/mod.rs — condition label instead of raw health

## Decisions Made
- Combat is simulation-only (no player controls) — future adventure mode can build on this
- Hidden combat_weight — player never sees the number, only prose descriptions of experience
- Notable-only logging — only agents with epithets or 2+ institutions generate combat log entries
- Injury severity only worsens (won't overwrite GravelyWounded with Bruised)
- Gravely wounded persistence for very loyal agents is intentionally rare (~10% of top-10% loyalty)
- Recovery happens every tick regardless of location, but gravely wounded have death risk when not at settlement
- Template suppression uses global history (not per-pair) with 30-tick window
- Word-wrap uses existing `word_wrap()` helper (not `wrap_text()` which is used in faction detail)

## Known Issues / To Investigate
- 6 compiler warnings (all dead_code / unused fields, pre-existing pattern)
- combat.rs `loser_id` and `margin` fields on CombatResult are structurally present but trigger unused warnings
- All template histories are transient (not saved/loaded) — intentional
- Site-based combat (agent vs site inhabitant) is a trigger condition in the spec but site inhabitants are not full agents — would need separate handling

## Next Steps
- Agent vs site inhabitant combat (requires design decision on inhabitant combat model)
- Further polish / new features as directed by player
- Distribution prep when ready

## Notes for Next Claude
- Player is not a programmer — explain decisions briefly, don't ask them to edit code
- All 5 phases complete plus combat system
- InspectAgent overlay is now InspectAgent(usize, usize) — (agent_idx, scroll_offset)
- Speed keybindings: 0/1/2/3 (not the old 1/5/2 scheme)
- SESSION_NOTES.md should be fully rewritten each update, not appended to
- Conversations are separate from relationships — they influence relationship formation but are tracked independently
- generate_relationship_event now takes 2 extra params (inst_a, inst_b) at the end
- pick_neutral_verb(rng) exists for weather/movement templates; pick_verb(reg, rng) is still used in institutional/death/other templates where register-specific verbs fit
- Suppression pattern: indexed functions return (u8, String), accept Option<u8> exclude; sim loop maintains history maps and passes excludes
- Census suppression is text-based ("contains census") for non-census event types, not template-index-based
- Global arrival/departure suppression (12-tick window) is in addition to per-settlement (50-tick window)
- sanitize_prose now handles both ",." → "." and "upon upon" → "upon"
- Combat template suppression is global (not per-agent-pair), 30-tick window
- Agent fields for combat: injury (InjuryStatus), recovery_remaining (u32), combats_survived (u16), last_combat_tick (u64) — all #[serde(default)]
- InstitutionRelationship::Rival is what drives faction-based combat triggers (not DiplomaticStance)
- Inspect overlay: `inner_width` computed once at top, `word_wrap()` used for Goal, Relationships, Conversations, Condition, Chronicle
- Two different wrap helpers exist: `word_wrap()` (inspect overlay) and `wrap_text()` (faction detail) — they do the same thing
