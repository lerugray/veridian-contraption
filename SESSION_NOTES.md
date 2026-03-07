# SESSION NOTES — Last updated: 2026-03-07

## Current State
- Phase: Phase 5 COMPLETE + combat system + combat history inspect overlay
- Last working feature: COMBAT HISTORY section in agent inspect overlay
- Build status: Compiles cleanly (warnings only, all dead_code / unused fields, pre-existing)

## What We Did This Session

### Combat History in Inspect Overlay
Added a COMBAT HISTORY section to the agent inspect overlay showing the last 8 combat events involving the inspected agent.

**New types (src/sim/combat.rs):**
- `CombatOutcome` enum: Win, Loss, Draw (with serde + Default)
- `CombatHistoryEntry` struct: tick, opponent_name, outcome, prose (serde-compatible)

**Agent changes (src/sim/agent.rs):**
- New field: `combat_history: Vec<CombatHistoryEntry>` with `#[serde(default)]`
- Capped at 20 entries (oldest dropped when exceeded)
- Import updated to include `CombatHistoryEntry`

**Recording (src/sim/mod.rs — process_combat_tick):**
- After injury application, both winner and loser get a CombatHistoryEntry pushed
- History recorded for ALL combats (not just notable ones that hit the live log)
- Each entry gets its own prose line from `gen_combat_inspect_prose()`
- Winner gets `is_winner: true`, loser gets `is_winner: false`; draws get CombatOutcome::Draw for both

**Prose (src/gen/prose_gen.rs):**
- New function: `gen_combat_inspect_prose()` — short one-line combat summaries for inspect view
- Uses existing `pick_verb()` for register sensitivity
- Separate pools for: draw (4), win/clean (5), win/injured (3), loss/grave (3), loss/wounded (3), loss/minor (4)
- All use existing sanitize_prose() and voice conventions

**Display (src/ui/overlays.rs):**
- COMBAT HISTORY section appears below CONDITION, before HELD ARTIFACTS
- Shows last 8 entries (from the 20 stored)
- Format: `[tick] [W/L/D] prose line`
- Color-coded: Win = green (100,200,100), Loss = red (220,80,80), Draw = grey (150,150,150)
- Word-wrapped using existing `word_wrap()` helper at panel width

**Constructor updates:**
- `combat_history: Vec::new()` added to all Agent constructors in:
  - src/gen/world_gen.rs (2 locations)
  - src/gen/eschaton_gen.rs (1 location)
  - src/sim/mod.rs (2 locations — birth events)

## Files Modified This Session
- src/sim/combat.rs — CombatOutcome enum, CombatHistoryEntry struct
- src/sim/agent.rs — combat_history field, updated import
- src/sim/mod.rs — combat history recording in process_combat_tick, constructor updates
- src/gen/prose_gen.rs — gen_combat_inspect_prose() function
- src/gen/world_gen.rs — combat_history in agent constructors
- src/gen/eschaton_gen.rs — combat_history in agent constructor
- src/ui/overlays.rs — COMBAT HISTORY section in inspect overlay

## Decisions Made
- Combat history is recorded for ALL combats, not just notable ones — the inspect overlay shows the agent's full personal record
- 20 entries stored, 8 displayed — gives depth without overwhelming the overlay
- Short one-line prose format for inspect (vs. full paragraph templates used in the live log)
- New prose templates were necessary — existing gen_combat_indexed templates are full paragraphs, too long for inspect-style entries
- Color coding matches intuitive expectations: green=win, red=loss, grey=draw

## Known Issues / To Investigate
- 6+ compiler warnings (all dead_code / unused fields, pre-existing pattern)
- combat.rs `loser_id` and `margin` fields on CombatResult are structurally present but trigger unused warnings
- All template histories are transient (not saved/loaded) — intentional
- Site-based combat (agent vs site inhabitant) not yet implemented — site inhabitants are not full agents

## Next Steps
- Agent vs site inhabitant combat (requires design decision on inhabitant combat model)
- Further polish / new features as directed by player
- Distribution prep when ready

## Notes for Next Claude
- Player is not a programmer — explain decisions briefly, don't ask them to edit code
- All 5 phases complete plus combat system with inspect history
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
- Agent fields for combat: injury (InjuryStatus), recovery_remaining (u32), combats_survived (u16), last_combat_tick (u64), combat_history (Vec<CombatHistoryEntry>) — all #[serde(default)]
- InstitutionRelationship::Rival is what drives faction-based combat triggers (not DiplomaticStance)
- Inspect overlay sections in order: header, epithets, status, goal, affiliations, relationships, conversations, CONDITION, COMBAT HISTORY, HELD ARTIFACTS, DISPOSITION, CHRONICLE, footer
- Two different wrap helpers exist: `word_wrap()` (inspect overlay) and `wrap_text()` (faction detail) — they do the same thing
