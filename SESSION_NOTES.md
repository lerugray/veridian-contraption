# SESSION NOTES — Last updated: 2026-03-06

## Current State
- Phase: Phase 5 IN PROGRESS (5-A complete)
- Last working feature: Expanded prose generation system with register-sensitive templates
- Build status: Compiles and runs cleanly (6 warnings, all pre-existing dead_code)

## What's Working
- **NEW: Prose Expansion (Phase 5-A)**
  - **10 template variants per event type** (up from 5-7), all register-sensitive
  - **5 register-specific word pools**: Bureaucratic, Clinical, Lyrical, Ominous, Conspiratorial — each with ~15 verbs and ~15 nouns
  - **Subordinate clause system**: ~30% of sentences get a qualifying clause, drawn from register-specific pools (14 bureaucratic, 10 each for other registers)
  - **Event subordinate clauses**: situation-level clauses (not agent-specific) for variety
  - **Weirdness-scaled cause system**: mundane (low) → absurdist (medium) → impossible (high). 18 absurdist causes, 12 impossible causes reported as mundane fact
  - **Impossible institution names**: 30 Kafkaesque names at high weirdness ("The Bureau of Determining What Is and Is Not a Bureau", "The Guild of Procedures That Reference Themselves")
  - **Oxymoronic epithets**: 18 formally contradictory epithets at high weirdness ("the Provisionally Permanent", "the Officially Unofficial")
  - **All prose functions now register/weirdness-aware**: generate_institutional_description, generate_site_description, generate_artifact_event, generate_adventurer_death all take register + weirdness params
  - **lib.rs added** for example binary support
  - **examples/prose_samples.rs**: generates 20+ sample entries for prose review

- **EXISTING: All Phase 1-4 systems** — fully working

## Decisions Made
- All verb pools use single-word past-tense forms only (no multi-word phrases) to ensure grammatical compatibility with passive constructions ("was X", "the Y was X")
- All noun pools avoid vowel-initial words to prevent "a/an" article mismatches in templates
- Subordinate clauses are register-specific (bureaucratic clauses differ from ominous ones)
- Impossible causes only appear at weirdness > 0.8 (40% chance); absurdist causes appear at weirdness > 0.4
- Impossible institution names appear at weirdness > 0.7 (35% chance)
- Oxymoronic epithets appear at weirdness > 0.65 (25% chance)
- `generate_epithet` wrapper removed; callers use `generate_epithet_with_weirdness` directly
- `generate_institution_name` wrapper preserved for backward compat; new code uses `_with_weirdness` variant

## Known Issues
- Room purposes not yet referenced in prose generation (deferred from Phase 3)
- 6 compiler warnings (pre-existing, all dead_code)
- Some lyrical nouns ("tide", "remnant") read slightly oddly when used as bureaucratic objects in shared templates — acceptable given the register is meant to color the prose strangely

## Next Steps
- Phase 5-B: Color/symbol tuning (Brogue-quality ASCII expressiveness), full export system
- Phase 5-C: Nested clause generation for complex events, narrative register variation per world parameter

## Notes for Next Claude
- Player is not a programmer — explain decisions briefly, don't ask them to edit code
- prose_gen.rs is now ~1050 lines — the main prose engine. All generation flows through register-specific helper functions (gen_agent_died, gen_inst_founded, etc.)
- All prose functions that previously took only `rng` now also take `register: NarrativeRegister` and `weirdness: f32` as the last two params
- name_gen.rs has `generate_institution_name_with_weirdness()` and `generate_epithet_with_weirdness()` — these are the primary public APIs now
- lib.rs exists to support examples/ — it re-exports all modules
- Run `cargo run --example prose_samples` to preview prose output
- SESSION_NOTES.md should be fully rewritten each update, not appended to
