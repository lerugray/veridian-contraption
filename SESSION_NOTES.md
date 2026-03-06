# SESSION NOTES — Last updated: 2026-03-06

## Current State
- Phase: Phase 4 IN PROGRESS (4-A complete — Parametric World Generation)
- Last working feature: Full parametric world generation with flavor presets
- Build status: Compiles and runs cleanly (6 warnings, all pre-existing)

## What's Working
- **NEW: Parametric World Generation (Phase 4-A)**
  - WorldParams struct: temporal_rate, political_churn, cosmological_density, ecological_volatility, narrative_register, weirdness_coefficient
  - NarrativeRegister enum: Clinical, Lyrical, Bureaucratic, Ominous, Conspiratorial
  - WorldFlavor enum with 5 presets that bias parameter generation with variance ranges
  - Presets: TheLongBureaucracy, TheBurningProvinces, TheDeepTaxonomy, TheConspiratorialAge, Unguided
  - Seed input on new game screen (text hashed to u64, blank = random)
  - Resolved seed displayed in world report
  - Params applied to simulation:
    - temporal_rate scales all event intervals (weather, settlement, census, institutional)
    - ecological_volatility scales weather event frequency
    - political_churn scales institutional event frequency and starting institution count
    - cosmological_density biases institution kinds toward SecretSociety/Cult
    - narrative_register selects register-specific vocabulary in prose generation
    - weirdness_coefficient controls absurdist vs mundane cause selection in prose
  - World report shows real parameter values with descriptive labels
  - Reroll preserves flavor preset
  - Register-specific word lists: Clinical, Lyrical, Ominous, Conspiratorial, Bureaucratic
- World generation with peoples, settlements, institutions, sites, artifacts, adventurer agents
- Full TUI with map, log, status bar, overlays
- Main menu with New World, Continue, Load World, Quit
- Save/load system with autosave every 500 ticks

## What We Did This Session
Implemented Phase 4-A — Parametric World Generation:
1. Added WorldParams struct and NarrativeRegister enum to world.rs
2. Added WorldFlavor enum with 5 presets to world_gen.rs, each with biased ranges
3. Updated generate_world() to accept flavor and generate params
4. Passed flavor through AppMode::Generating and WorldReport states
5. Applied temporal_rate to all periodic event intervals in sim/mod.rs
6. Added register-specific word lists to prose_gen.rs (5 registers × verbs + nouns)
7. Updated generate_description() and subordinate_clause() to use register/weirdness
8. Updated world report to show real params with descriptive labels
9. Updated menu flavor descriptions

## Decisions Made
- WorldParams uses #[serde(default)] for backward compatibility with old saves
- Flavor presets use biased ranges (center ± spread) for organic variance
- Event intervals use formula: base_interval / (temporal_rate * modifier) with .max() floor
- Register-specific vocabulary replaces BUREAUCRATIC_NOUNS/PROCEDURAL_VERBS when non-Bureaucratic register is active
- pick_cause() uses weirdness as probability threshold for absurdist vs mundane causes

## Known Issues
- Room purposes not yet referenced in prose generation
- No delete-save on Load World screen (minor QOL)
- 6 compiler warnings (pre-existing, all dead_code)
- Phase 3 polish items deferred (room purposes in prose, more artifact interactions)

## Next Steps
- Phase 4-B: Multiple world support, Load World screen fully supporting switching between worlds
- Phase 4-C: World Annals — auto-generated historical summary per era

## Notes for Next Claude
- Player is not a programmer — explain decisions briefly, don't ask them to edit code
- generate_world() now takes (seed, WorldFlavor) — second param is the flavor preset
- WorldFlavor::from_index(i) maps menu selection index to flavor enum
- AppMode::WorldReport now has { scroll, flavor } fields
- AppMode::Generating now has { seed, flavor, frames_shown } fields
- prose_gen::generate_description() now takes 8 params (added register + weirdness at end)
- SESSION_NOTES.md should be fully rewritten each update, not appended to
