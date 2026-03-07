# SESSION NOTES — Last updated: 2026-03-07

## Current State
- Phase: Phase 5 COMPLETE + post-phase polish
- Last working feature: Prose pass 4 + minor fixes (upon upon, substituted verb)
- Build status: Compiles cleanly (5 warnings, all pre-existing dead_code)

## What We Did This Session

### Prose Pass 4 — Clinical register bleed + death rate tuning
- Replaced register-branching template 7 in `gen_site_entered_indexed` and `gen_site_left_indexed` with register-adaptive templates using `pick_noun`/`pick_verb` — eliminates Clinical text firing in non-Clinical worlds
- Natural death base rate: `0.0002` → `0.00005`, quadratic coefficient: `0.005` → `0.002`
- Adventurer site death rate: `0.03` → `0.008` per tick

### Minor fixes — upon upon + substituted
- Extended `sanitize_prose()` to collapse "upon upon" → "upon" (was occurring when templates ending in "upon" got a TEMPORAL_HEDGE starting with "upon completion...")
- Applied `sanitize_prose()` to return values of `gen_site_entered_indexed`, `gen_site_left_indexed`, `gen_agent_arrived_indexed`, `gen_agent_departed_indexed`
- Removed "substituted" from `CONSPIRATORIAL_VERBS` — was producing nonsensical results in census/relationship templates ("A population of 82 was substituted", "A rivalry has been substituted")

## Files Modified This Session
- src/gen/prose_gen.rs — sanitize_prose extended, template 7 rewrites, CONSPIRATORIAL_VERBS fix, sanitize_prose applied to 4 indexed functions
- src/sim/agent.rs — natural death rate constants reduced
- src/sim/mod.rs — adventurer site death rate reduced

## Decisions Made
- Removed register-specific branching in site templates rather than debugging root cause of register mismatch — eliminates the problem entirely
- Death rate tuning is rates-only, no simulation logic changes
- sanitize_prose applied at return of indexed functions rather than globally — targeted and predictable
- "substituted" removed entirely from conspiratorial verbs rather than trying to fix individual templates

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
- sanitize_prose now handles both ",." → "." and "upon upon" → "upon"
