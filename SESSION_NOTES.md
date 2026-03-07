# SESSION NOTES — Last updated: 2026-03-06

## Current State
- Phase: Phase 5 COMPLETE + post-phase polish
- Last working feature: Faction detail dossier screen
- Build status: Compiles and runs cleanly (6 warnings, all pre-existing dead_code)

## What We Did
- Added a full faction detail screen accessible by pressing Enter on any faction in the faction list overlay (Shift+F)
- The dossier displays:
  1. Faction name, type, and founding tick
  2. Charter and doctrine (word-wrapped)
  3. Institutional health assessment (Ascendant/Stable/Declining/Diminished/Defunct)
  4. Scrollable member list (capped at 20, with "and X more")
  5. Relationships with other factions (color-coded: green=Allied, white=Neutral, red=Rival, yellow=Disputed)
  6. Artifacts held by faction members
  7. Historical record from world annals + institution chronicle entries
- Added text wrapping and truncation helpers (wrap_text, truncate_str) to prevent text overflow past overlay borders
- ESC from detail returns to faction list at the correct position
- Up/Down/PageUp/PageDown scroll the detail view

## Decisions Made
- Used word-wrapping for long prose (charter, doctrine, annals, chronicle) and truncation for short fields (names, relationships) to keep text within overlay bounds
- Multi-span colored lines were simplified to single-span truncated strings to prevent ratatui clipping issues
- Health assessment thresholds: Ascendant (power>=80, members>=8), Stable (power>=50 or members>=5), Declining (power>=20 or members>=2), Diminished (below that), Defunct (0 members)

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
- Overlay enum variant for faction detail: FactionDetail(usize, usize) = (institution index, scroll offset)
- wrap_text() and truncate_str() helpers live at the bottom of overlays.rs — reuse for any future overlays with long text
