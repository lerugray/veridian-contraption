# SESSION NOTES — Last updated: 2026-03-07

## Current State
- Phase: Phase 5 COMPLETE + post-phase polish
- Last working feature: Clinical register bleed fix + death rate tuning
- Build status: Compiles cleanly (5 warnings, all pre-existing dead_code)

## What We Did This Session
Two targeted fixes, no structural changes:

### 1. Clinical Register Bleed in Site Entry/Exit Prose
- Template 7 in `gen_site_entered_indexed` and `gen_site_left_indexed` had a `match reg` block with hardcoded Clinical-specific text ("Subject X entered site Y. Ingress logged." / "Subject X exited site Y. Condition: intact.")
- The register IS passed correctly from `self.world.params.narrative_register` (mod.rs:494), but Clinical text was firing in Bureaucratic worlds
- Fix: Replaced the register-branching template 7 in both functions with register-adaptive templates using `pick_noun(reg, rng)` and `pick_verb(reg, rng)` helpers — these automatically produce register-appropriate vocabulary regardless of which register is active
- Entry template 7 now: "{name} entered {site} under conditions the {noun} described as 'not entirely unprecedented.' A {noun} was {verb} accordingly."
- Exit template 7 now: "{name} emerged from {site} bearing a {noun} that the local {noun} {verb} without comment."

### 2. Death Rate Tuning
- **Natural death** (agent.rs:214): Base rate reduced from `0.0002` to `0.00005`, quadratic coefficient from `0.005` to `0.002`
  - Old: ~7% annual death rate at age 50, ~31% at age 70
  - New: ~1.8% at age 50, ~12.7% at age 70 — much more reasonable
  - Hard cap at age 100 (36500 ticks) unchanged
- **Adventurer site death** (mod.rs:2484): Rate reduced from `0.03` to `0.008` per tick
  - Old: ~78% fatality for a 50-tick site visit (most adventurers died on first expedition)
  - New: ~33% fatality for 50-tick visit — still dangerous but survivable
- Winter death rates left unchanged (already low and scaled by ecological_volatility)

## Files Modified This Session
- src/gen/prose_gen.rs — replaced template 7 in both `gen_site_entered_indexed` and `gen_site_left_indexed`
- src/sim/agent.rs — reduced natural death rate constants
- src/sim/mod.rs — reduced adventurer site death rate

## Decisions Made
- Removed register-specific branching in site templates rather than debugging the root cause of register mismatch — this eliminates the problem entirely and keeps templates register-adaptive
- Death rate tuning is rates-only, no simulation logic changes
- Adventurer death at 0.008/tick still makes sites meaningfully dangerous (~33% per visit) without killing most adventurers on their first expedition

## Known Issues / To Investigate
- 5 compiler warnings (pre-existing, all dead_code)
- All template histories are transient (not saved/loaded) — intentional

## Next Steps
- Further polish / new features as directed by player
- Distribution prep when ready

## Notes for Next Claude
- Player is not a programmer — explain decisions briefly, don't ask them to edit code
- All 5 phases complete plus extensive post-phase polish
- InspectAgent overlay is now InspectAgent(usize, usize) — (agent_idx, scroll_offset)
- Speed keybindings: 0/1/2/3 (not the old 1/5/2 scheme)
- SESSION_NOTES.md should be fully rewritten each update, not appended to
- Conversations are separate from relationships — they influence relationship formation but are tracked independently
- generate_relationship_event now takes 2 extra params (inst_a, inst_b) at the end
- pick_neutral_verb(rng) exists for weather/movement templates; pick_verb(reg, rng) is still used in institutional/death/other templates where register-specific verbs fit
- Suppression pattern: indexed functions return (u8, String), accept Option<u8> exclude; sim loop maintains history maps and passes excludes
- Census suppression is text-based ("contains census") for non-census event types, not template-index-based
- Global arrival/departure suppression (12-tick window) is in addition to per-settlement (50-tick window)
- `without_leading_article()` and `sanitize_prose()` helpers are available in prose_gen.rs for future use
