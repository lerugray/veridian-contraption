# SESSION NOTES — Last updated: 2026-03-06

## Current State
- Phase: Phase 5 COMPLETE + post-phase polish
- Last working feature: Demographic system (births, deaths, emigration, immigration)
- Build status: Compiles and runs cleanly (6 warnings, all pre-existing dead_code)

## What We Did
- Implemented full demographic oscillation system:
  1. **Births**: Settlements generate new agents periodically. Birth rate scales with local population (sqrt) and temporal_rate. New agents are fully generated (name, people, disposition). Prose reads as census registrations.
  2. **Natural Death**: Replaced hard death cap at 36500 ticks with gradual mortality starting at age 50 (18250 ticks), ramping quadratically. Uses new NaturalDeath event type (distinct from AgentDied which is for violent/adventure deaths).
  3. **Emigration**: Agents with high risk_tolerance, low institutional_loyalty, or high paranoia may leave the known world. Prose is suitably vague about destinations.
  4. **Immigration**: New agents arrive from outside the known world (~1.5% chance per 10-tick check, scaled by temporal_rate). Arrive as adults at random settlements.
- Added `next_agent_id` field to SimState for monotonically increasing agent IDs
- Added 3 new EventTypes: AgentEmigrated, AgentImmigrated, NaturalDeath
- Added 30+ prose templates across 3 new generators in prose_gen.rs

## Decisions Made
- NaturalDeath is intentionally NOT a major event for era tracking — it's routine demographic change
- NaturalDeath DOES contribute to tension (same as AgentDied)
- Demographic checks run every 10 ticks to reduce per-tick overhead
- Birth rate uses sqrt(local_pop) to avoid exponential growth in large settlements
- Emigration samples ~20% of agents per check rather than iterating all
- Immigration agents arrive as adults (age 10-50 years) with small chance of being adventurers

## Known Issues
- Room purposes not yet referenced in prose generation (deferred from Phase 3)
- 6 compiler warnings (pre-existing, all dead_code)
- Unicode symbols require a terminal with good Unicode support

## Next Steps
- Further polish / new features as directed by player
- Distribution prep (cross-platform builds, cargo-bundle, Steam packaging) when ready
- Could tune demographic rates after observing real gameplay if oscillation needs adjustment

## Notes for Next Claude
- Player is not a programmer — explain decisions briefly, don't ask them to edit code
- All 5 phases are complete. The game is fully functional with all GDD Phase 1-5 features
- next_agent_id lives on SimState (not saved/loaded — reconstructed from max agent ID)
- frame_count lives on SimState (not saved), updated from main loop each frame — used for all animations
- Status message TTL counts down per frame (not per tick), so display duration is real-time consistent
- Speed keybindings: 0/1/2/3 (not the old 1/5/2 scheme)
- SESSION_NOTES.md should be fully rewritten each update, not appended to
