# SESSION NOTES — Last updated: 2026-03-07

## Current State
- Phase: Phase 5 COMPLETE + post-phase polish (seasons, relationships, conversations, prose repetition fixes, prose cleanup passes, suppression, grammar/article fixes)
- Last working feature: Prose pass 3 — article doubling, trailing comma, global arrival cooldown, register grammar
- Build status: Compiles and runs cleanly (5 warnings, all pre-existing dead_code)

## What We Did (Prose Pass 3)
Four targeted prose fixes, no simulation logic changes:

### 1. Article Doubling Fix
- Inhabitant names like "The Unnamed Occupant" or "A Former Surveyor" were getting doubled articles when templates prepended "The" — producing "The The Unnamed Occupant"
- Added `without_leading_article()` helper that strips "The "/"A "/"An " prefix
- Applied to the one affected template in `gen_inhabitant_ignored` (template 1)

### 2. Trailing Comma Before Period Fix
- `name_with_optional_clause` adds trailing comma to "Name, clause," — when placed at end of sentence before period, produced ",."
- Added `sanitize_prose()` helper that replaces ",." with "."
- Applied to `gen_agent_arrived` template 9 (the only template where `nwc` appears right before a period)

### 3. Global Arrival/Departure Template Cooldown
- Previously, arrival/departure suppression was per-settlement only (50-tick window)
- "The gates admitted... in the usual manner" could fire for 3 different settlements in the same tick
- Added `global_arrival_template_history` and `global_departure_template_history` fields to SimState
- 12-tick global cooldown window prevents same template firing across different settlements in rapid succession
- Per-settlement 50-tick suppression still active alongside global

### 4. Register Vocabulary Grammar Fixes
- `pick_verb(reg, rng)` returns past tense ("gathered", "carried", "scattered") but some templates needed infinitive/base form
- Fixed 4 templates:
  - `gen_artifact_acquired` template 3: "no one present to gathered it" — rewrote to avoid infinitive slot
  - `gen_inhabitant_drove_out` template 2: "would later carried as 'prudent'" — rewrote to fixed prose
  - `gen_inhabitant_questioned` template 2: "was scattered by the occupant as 'improbable'" — fixed to "dismissed"
  - Conspiratorial `event_subordinate_clause` template 2: "interesting enough to intercepted" — fixed to "interesting enough to note"
- Fixed seasonal template: "Several tide were filed" — "Several" + uncountable lyrical noun; changed to fixed "complaints"

## Files Modified This Session
- src/gen/prose_gen.rs — `without_leading_article()` helper, `sanitize_prose()` helper, 6 template fixes
- src/sim/mod.rs — `global_arrival_template_history` and `global_departure_template_history` fields + initialization + tick-loop logic

## Decisions Made
- Global arrival/departure cooldown is 12 ticks (short enough to not suppress variety, long enough to prevent same-tick repeats across settlements)
- Per-settlement and global suppression work together: exclude = per_settle.or(global) — per-settlement takes priority since it has the longer window
- For grammar-broken templates, preferred rewriting to fixed prose over creating a new pick_verb_infinitive() function — simpler, less risk of new bugs
- `sanitize_prose` is a targeted wrapper, not applied globally — only used where ",." can actually occur

## Known Issues / To Investigate
- 5 compiler warnings (pre-existing, all dead_code)
- All new template histories are transient (not saved/loaded) — intentional, same as weather/arrival/departure
- **Red (death) events may be firing too frequently** — player noticed high rate in Unguided world, possibly across other profiles too. Needs investigation into death rate tuning (population balance / aging / adventurer death rates).
- **Possible clinical register bleed in site exit prose** — player saw "Subject X exited site Y. Condition: intact. Debriefing: not conducted. taxonomic summary filed." in an Unguided world. This is `gen_site_left_indexed` template 7, Clinical branch. Need to determine: (1) was the Unguided world's register actually Clinical (in which case this is correct), or (2) is the register not being passed correctly to site prose generators? Same pattern exists in `gen_site_entered_indexed` template 7. Player should press W in-game to check world register next time this is seen.

## Next Steps
- Investigate red event frequency (death rate tuning)
- Confirm or fix clinical register bleed in site prose for Unguided worlds
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
