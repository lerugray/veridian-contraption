# SESSION NOTES — Last updated: 2026-03-06

## Current State
- Phase: Phase 4 IN PROGRESS (4-A + 4-B complete)
- Last working feature: World Annals system with era tracking, overlay, and export
- Build status: Compiles and runs cleanly (6 warnings, all pre-existing)

## What's Working
- **NEW: World Annals System (Phase 4-B)**
  - AnnalsEntry struct: era_name, start_tick, end_tick, summary, notable_agents, notable_institutions, defining_event
  - Era system: eras end after ERA_THRESHOLD (15) major events accumulate
  - Major events tracked: AgentDied, InstitutionFounded/Dissolved, SchismOccurred, AllianceFormed, RivalryDeclared, ArtifactAcquired/Delivered, AdventurerDiedInSite
  - Era transition generates prose summary paragraph and logs the transition
  - Era names generated procedurally: "The First Dispensation", "The Second Accounting", etc.
  - 'a' key opens scrollable World Annals overlay showing all completed eras + current era
  - Annals export added to export menu as option [4]
  - export_world_annals() writes formatted historical document to /exports/
  - Annals saved/loaded via SaveData with serde(default) for backward compatibility
  - Help screen updated with 'a' keybinding
  - Status bar shows a=annals hint

- **EXISTING: Save/Load System (already complete from earlier phases)**
  - save_world(), load_world() in export/mod.rs — working since Phase 1-F
  - Autosave every 500 ticks to /saves/autosave.json
  - Ctrl+S manual save with name prompt
  - Load World screen, Continue from autosave
  - All structs already had Serialize/Deserialize derives

- **EXISTING: Parametric World Generation (Phase 4-A)**
  - WorldParams, NarrativeRegister, WorldFlavor presets
  - Params applied to simulation intervals and prose generation

## Decisions Made
- ERA_THRESHOLD = 15 major events per era (balances frequent enough transitions without spam)
- Era names use ordinals + bureaucratic nouns ("The Third Registry")
- Era summaries are 2-4 sentences generated from world state at transition time
- Annals overlay uses 75%×85% of screen, scrollable
- Backward-compatible: old saves without annals fields load fine via serde(default)
- Notable agents/institutions capped at 8/6 per era to keep summaries readable

## Known Issues
- Room purposes not yet referenced in prose generation
- No delete-save on Load World screen (minor QOL)
- 6 compiler warnings (pre-existing, all dead_code)
- Phase 3 polish items deferred (room purposes in prose, more artifact interactions)

## Next Steps
- Phase 4-C: Multiple world support — Load World screen fully supporting switching between up to 10 worlds
- Phase 5: Polish, depth, and voice

## Notes for Next Claude
- Player is not a programmer — explain decisions briefly, don't ask them to edit code
- AnnalsEntry is in sim/mod.rs alongside SimState
- ERA_THRESHOLD is pub const in sim/mod.rs
- transition_era() is called from tick() when era_major_events >= ERA_THRESHOLD
- generate_era_summary() produces the prose paragraph
- generate_era_name() is in gen/name_gen.rs
- Overlay::Annals(scroll) handled in main.rs handle_annals_input()
- Export option [4] in export menu triggers export_world_annals()
- SESSION_NOTES.md should be fully rewritten each update, not appended to
