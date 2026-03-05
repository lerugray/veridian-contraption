# VERIDIAN CONTRAPTION

*A World-Simulator of Considerable Density and Dubious Intent*

Concept & Design Document — Unified Edition

# I. OVERVIEW & DESIGN PHILOSOPHY

Veridian Contraption is an autonomous world-simulator presented through
an ASCII terminal interface. It generates and sustains living worlds of
considerable strangeness — worlds that proceed whether the player
watches or not, narrate themselves in dense and peculiar prose, and
export their histories on demand. The player is a spectator, an
occasional nudge, a curator of emergent absurdity.

The game's primary relationship with the player is not one of challenge
or mastery, but of witness. It runs in a corner of your screen.
Civilizations rise, splinter, and develop baroque bureaucracies while
you are doing something else. When you return, there is always more to
read.

> *Design axiom: The simulation should always have more going on than
> the player can fully attend to.*

The aesthetic lineage: Dwarf Fortress for systemic depth and emergent
narrative. Caves of Qud for procedural prose with genuine voice. Classic
D&D adventure modules and Judges Guild supplements for absurdist
specificity — the kind of detail that names a dungeon something
gloriously stupid and then plays it completely straight. The
Illuminatus! Trilogy for conspiratorial world-logic and the sense that
every institution has a hidden, ridiculous history. Nabokov for density
of language that remains, somehow, readable.

# II. CORE LOOP & PLAYER RELATIONSHIP

## The Simulation Loop

The world runs continuously, on a tick-based clock the player can
control:

-   PAUSE — freeze the world entirely; browse, read, configure

-   STEP — advance one tick at a time, useful for following dramatic
    moments

-   RUN — continuous simulation at configurable speed (1x, 5x, 20x)

-   FAST FORWARD — skip ahead, useful for watching long-form
    historical arcs

Each tick processes: individual agent actions and decisions,
faction/institution updates, environmental events, narrative log
generation. The simulation is designed to be fast enough in Rust to run
at 20x speed without becoming a CPU concern on modest hardware.

## Player Interaction Model

All player interaction is optional. The simulation runs fine without it.
Interaction is a menu-driven overlay, navigated with keyboard (and
optionally mouse in terminal environments that support it). The player
never issues direct commands to individual agents. Instead:

-   Set world-generation parameters before a new world begins

-   Adjust macro pressures mid-simulation (dial up political
    instability, shift weather patterns, seed a new religion)

-   Follow a specific character or faction, keeping their log in a
    persistent side-panel

-   Inspect any entity: character, city, dungeon, artifact, institution

-   Export logs and summaries to TXT at any time

The key design principle: a player who never touches anything should
have a satisfying experience of watching the world unfold. A player who
digs in should find inexhaustible depth.

## The Log & Export System

The simulation generates continuous narrative text — not data
readouts, but prose. This log is the game's primary output. It should
read like an account written by a bureaucrat who has read too much
Borges.

-   LIVE LOG — scrolling pane showing recent events, updated in real
    time

-   CHARACTER CHRONICLE — per-agent narrative history, inspectable at
    any time

-   FACTION RECORD — institutional history, political developments,
    doctrinal shifts

-   WORLD ANNALS — high-level historical summary, updated per era

-   EXPORT — dumps any of the above to a dated .txt file in a /exports
    subfolder

> *Example log entry: On the forty-third day of the Pelted Season, the
> Registrar of Ambiguous Debts for the Confraternity of the Ossified
> Scale formally disputed the citizenship of Whelm Durr-Anquist, citing
> seventeen procedural irregularities and one metaphysical objection
> that the tribunal declined to record.*

# III. WORLD GENERATION & PARAMETRIC VARIANCE

## Every World is Its Own System

This is a core design distinction from Dwarf Fortress: Veridian
Contraption does not apply the same simulation parameters to every
world. Each world generation produces not only a geography and
population but a set of rules — rates, weights, tendencies — that
govern how that particular world behaves. Two worlds generated with
identical player inputs will have meaningfully different dynamics.

Parametric variance is generated across several axes:

-   TEMPORAL RATE — how fast events unfold; some worlds run on
    geological time, others burn through centuries in seasons

-   POLITICAL CHURN — rate of institutional change, how stable or
    volatile power structures are

-   COSMOLOGICAL DENSITY — how many active supernatural/metaphysical
    forces are in play

-   ECOLOGICAL VOLATILITY — how aggressively the environment reshapes
    itself

-   NARRATIVE REGISTER — the prose style the world's log tends
    toward: clinical, lyrical, bureaucratic, ominous

-   WEIRDNESS COEFFICIENT — global absurdity dial; affects naming,
    events, institutional behavior, creature generation

## World Seeds & Preset Flavors

Players can enter a text seed (hashed to generate parameters) or choose
from named flavor presets that suggest a starting character for the
world without locking it down:

-   THE LONG BUREAUCRACY — slow time, high political complexity, dense
    institutions, dry narrative register

-   THE BURNING PROVINCES — fast time, high volatility,
    military/factional focus, terse register

-   THE DEEP TAXONOMY — focus on creature and ecological generation,
    naturalist register

-   THE CONSPIRATORIAL AGE — high cosmological density, paranoid
    register, secret societies proliferate

-   UNGUIDED — fully random parameters, maximum variance, anything
    goes

Flavor presets bias the parametric generation without guaranteeing
outcomes. A Long Bureaucracy world might still produce a catastrophic
war. It just takes longer to get there, and there will be more
paperwork.

## Multi-World Save Design

The player's intended final experience includes maintaining multiple simultaneous
active worlds — different seeds, different flavors, different histories — and being
able to switch between them from the main menu. This is the Load World screen's
primary purpose in the mature product.

**Target:** Up to 10 named saves, each representing a distinct living world.
**Limit rationale:** Keeps the save list manageable; can be raised later.
**Behavior at limit:** Oldest save is flagged in the Load screen; player must
manually delete it before creating a new world. No auto-deletion.

Phase 1 implementation: single save system works fine as foundation.
Phase 4 implementation: Load World screen fully supports switching between worlds.
No additional architectural changes needed — the save/load system already handles this.

# IV. WORLD, SETTING & AESTHETIC

## The Gonzo Fantasy Register

The world is recognizably fantasy in structure — there are
civilizations, dungeons, magical forces, adventurers, monsters — but
it is not straightforwardly Tolkienian. The aesthetic is closer to the
weird specificity of early D&D, where a supplement might describe a
dungeon called something absurd and then provide a completely earnest
room-by-room key. The humor is never winking. The world takes itself
seriously. The strangeness emerges from that.

The Illuminatus! Trilogy provides the deeper register: the sense that
every institution has a hidden, paranoid, ridiculous logic; that
conspiracies emerge not from malevolence but from the natural
self-perpetuating tendency of bureaucracies; that the cosmos is
fundamentally absurd but this does not make it any less dangerous to
live in.

## Races & Peoples

No elves. No standard dwarves (though something dwarf-adjacent may
emerge under a different name with different properties). Peoples are
generated procedurally with:

-   Generated name (phoneme-based, influenced by world seed)

-   Physiological profile — body plan, sensory apparatus,
    environmental adaptation

-   Cultural disposition — not alignment, but tendencies: relationship
    to debt, to hierarchy, to the dead, to language

-   Institutional signature — what kind of organizations do they
    naturally produce? Guilds? Hereditary courts? Distributed cell
    networks?

-   Cosmological relationship — what do they believe is happening at
    the fundamental level of reality, and are they right?

> *Example: The Grevvin are a people of considerable lateral width who
> communicate primarily through voluntary bioluminescence and have
> developed a legal tradition so elaborate that most contracts now
> require a licensed Dream Interpreter to be considered binding. They
> have been at administrative war with the Confraternity of the Ossified
> Scale for eleven generations over a boundary dispute that has since
> been resolved but which both parties continue litigating for reasons
> that have become, at this point, primarily ceremonial.*

## Language Generation

Caves of Qud demonstrates that procedural language can have genuine
voice rather than merely producing plausible-sounding nonsense. This is
the target. The system needs:

-   Phoneme tables per culture/people — names should feel internally
    consistent per group

-   Title and epithet generation — characters accumulate honorifics,
    nicknames, bureaucratic designations

-   Institutional name generation — organizations have names that
    follow consistent internal logic

-   Prose template system — log entries are generated from sentence
    templates with slotted variables, but the templates should be varied
    and stylistically tuned

-   Nested clause generation — some events require a sentence that
    contains a sentence that qualifies another sentence; this is
    intentional

## Geography & Dungeons

Geography is generated per world and evolves over simulation time. The
map view is a zoomed-out ASCII overview; the dungeon/location view zooms
in to individual-tile scale for sites of interest. Notable geographic
features:

-   Settlements of varying scale — hamlets to city-states

-   Ruins — the remains of earlier civilizations, earlier in the same
    world's history

-   Dungeons — generated as inhabited institutions, not empty
    puzzle-boxes; a dungeon has a reason it exists and an evolving
    population

-   Wilderness regions — have ecologies, weather, resources, and
    occasionally opinions

# V. SIMULATION SYSTEMS

## Agent Simulation

Individual characters (agents) are the atomic unit of the simulation.
Each agent has:

-   Identity: name, people/culture, physical traits, generated backstory
    summary

-   Disposition profile: a small set of behavioral weights (risk
    tolerance, institutional loyalty, ambition, paranoia, theological
    certainty)

-   Current situation: location, affiliation(s), current goal, ongoing
    relationships

-   Chronicle: a running narrative log of their significant actions

Agents pursue goals generated by their situation and disposition. They
join, found, betray, and dissolve institutions. They explore dungeons,
acquire artifacts, develop beliefs, and occasionally achieve things that
get recorded in the World Annals.

## Institutional Simulation

Institutions (factions, guilds, governments, cults, mercenary companies,
regulatory bodies) are the mid-level unit. They have:

-   Charter — stated purpose, which may no longer reflect actual
    function

-   Power — resources, territory, agents currently affiliated

-   Doctrine — what they officially believe, and internal dissenting
    positions

-   Relationship map — alliances, enmities, tributary arrangements,
    ongoing disputes

Institutions compete, merge, splinter, and develop baroque internal
bureaucracies independent of player involvement. A guild founded to
regulate the wool trade may, over several generations, become primarily
focused on suppressing a rival guild and may have largely forgotten
about wool.

## Event System

Events are the mechanism by which the simulation moves. Events are
triggered by agent states, institutional states, environmental
conditions, and random chance weighted by world parameters. Events
generate log entries, modify agent/institutional states, and may trigger
further events. Categories:

-   Personal events — births, deaths, conversions, discoveries,
    betrayals, achievements

-   Institutional events — elections, schisms, mergers, founding,
    dissolution, doctrine shifts

-   Political events — wars, treaties, embargoes, territorial changes,
    succession crises

-   Environmental events — weather, geological shifts, migrations,
    ecological changes

-   Cosmological events — divine interventions, metaphysical
    anomalies, prophecy fulfillments

## The Eschaton System

"Immanentize the Eschaton" is both a player-triggered option and an autonomous
simulation event. The name is taken directly from the Illuminatus! Trilogy, where
it describes the act of bringing about the end of history — except in Veridian
Contraption, history does not end. It is merely reorganized, sometimes violently.

### Trigger Conditions

**Player-triggered:**
- Player opens the Eschaton menu (ESC key, with confirmation prompt)
- The confirmation screen presents the current Eschaton type the world is
  primed for (based on world state) and warns that the effects are permanent
- Player confirms or cancels

**Autonomous trigger:**
- Fires when all three of the following are true:
  1. cosmological_density parameter is above 0.65
  2. An internal "tension meter" (accumulated from political instability,
     unresolved institutional conflicts, and metaphysical events) crosses a threshold
  3. A random check passes (weighted by tension level)
- When triggered autonomously, the log announces it with a distinct event entry
  before the effects resolve

### Eschaton Types

World state determines which fires; the player can see which is primed:

- **THE RECKONING OF DEBTS** — All outstanding institutional disputes resolve simultaneously,
  catastrophically. Institutions collapse, merge, or achieve pyrrhic victories.
  Political map rewrites itself. Economy (abstract) resets.

- **THE TAXONOMIC CORRECTION** — The world's ecology undergoes violent reorganization.
  Terrain types shift in regions. Several creature/people populations are drastically
  reduced or displaced. New ecological pressures emerge.

- **THE ADMINISTRATIVE SINGULARITY** — Bureaucratic complexity reaches critical mass.
  A single institution briefly achieves total administrative control, then
  immediately collapses under its own weight. Leaves behind a power vacuum and
  seventeen competing successor bodies.

- **THE GEOLOGICAL ARGUMENT** — The land itself disagrees with recent history.
  Geography reshapes: coastlines shift, mountains are added or subtracted,
  rivers change course. Settlements in affected areas are damaged or destroyed
  (but not all of them).

- **THE DOCTRINAL CASCADE** — A religious/philosophical idea propagates through the
  world's institutions at impossible speed. Every institution must either adopt it,
  violently reject it, or splinter. Many do all three.

- **THE ARRIVAL OF SOMETHING OWED** — An entity, force, or debt arrives that the world
  has been accumulating. Nature determined by world history. Effects are unique
  per playthrough. This is the rarest and most dramatic Eschaton type.

### Post-Eschaton

- A new named Era begins immediately
- World Annals records the Eschaton as the defining event of the concluded Era
- Several world parameters shift (the world is changed, not reset)
- The simulation continues — agents adapt, institutions rebuild, the log goes on
- A second Eschaton cannot fire for at least 500 ticks after the first

# VI. TERMINAL UI & ASCII DISPLAY

## Display Philosophy

The interface is a terminal application — fullscreen, no window
chrome. The aesthetic reference points are classic Dwarf Fortress
(information-dense, ASCII-based) and Brogue (ASCII used expressively,
with color and symbol choice doing visual work). The interface should
feel alive: log text scrolls, map tiles flicker with activity,
highlighted entities pulse.

Rust terminal library: ratatui (formerly tui-rs) is the primary
candidate — it provides a robust widget system for terminal UIs and
has good support for colored text, borders, and panels.

## Display Layout — Main View

> *┌─────────────────────────────┬──────────────────┐*
>
> *│ WORLD MAP (ASCII, colored) │ LIVE LOG PANE │*
>
> *│ │ (scrolling text) │*
>
> *│ \[zoomed-in site view here\] │ │*
>
> *├─────────────────────────────┴──────────────────┤*
>
> *│ STATUS BAR: World name \| Date \| Speed \| Focus │*
>
> *└─────────────────────────────────────────────────┘*

Panels are resizable. The log pane can be expanded to full-screen for
reading. The map can be zoomed from world-level to settlement-level to
dungeon-tile-level.

## Input Model

-   Keyboard-primary: all functions accessible without mouse

-   Mouse support: click to select/inspect entities on map, scroll log
    pane

-   ? — context-sensitive help at any time

-   SPACE — pause/unpause

-   . — step one tick

-   1/5/2 — set speed 1x/5x/20x

-   F — follow mode: select an agent or faction to track in side panel

-   I — inspect: open full detail view for entity under cursor

-   E — export menu: choose what to dump to TXT

-   W — world parameters: adjust macro pressures

-   ESC — immanentize the eschaton menu (with confirmation)

-   Ctrl+S — save

-   Q — quit (with confirmation)

## Main Menu

The game launches to a main menu before the simulation begins:

```
VERIDIAN CONTRAPTION
[ascii art title]

  > New World
    Continue (autosave)
    Load World
    Quit
```

Navigation: arrow keys, Enter to select.

### New World Screen

- Choose flavor preset (arrow keys, description shown for highlighted option)
- Enter optional text seed (blank = random)
- Confirm — shows brief world generation summary before sim starts

### Load World Screen

- Lists all saves in /saves/ by name and last-modified date
- Up to 10 named saves displayed (oldest is overwrite candidate when at limit)
- Select to load, Delete key to remove a save (with confirmation)

### In-Game Save

- Ctrl+S: prompts for save name, writes to /saves/{name}.json
- Autosave fires every 500 ticks, always to /saves/autosave.json
- Save name is displayed in the status bar

### Status Bar

- Show current save name (or "unsaved" if new world not yet saved)
- Show autosave indicator briefly when autosave fires ("~ autosaved")

# VII. BUILD PLAN — PHASED APPROACH

## Philosophy

This is a large project. The phased plan below is designed so that each
phase produces something that is itself interesting and runnable — not
just scaffolding for future work. Each phase is a complete vertical
slice that gets expanded by the next phase.

## Phase 1 — The Foundation (MVP)

Goal: A running terminal application that generates a world and narrates
it in real time. Playable and interesting on its own.

-   Rust project setup with ratatui for TUI

-   Basic world generator: geography (heightmap to ASCII), 2-4 peoples,
    6-12 starting settlements

-   Agent simulation: ~50-200 agents, simple goal-pursuit, basic
    relationships

-   Event system: ~20 event types, log generation with prose templates

-   Simulation controls: pause/step/run/speed

-   Basic display: world map pane + scrolling log pane

-   Inspect: click/select agent or settlement for basic detail view

-   TXT export: dump current log to file

-   Main menu and basic save/load system (Prompt 1-F)

## Phase 2 — Institutions & Language

Goal: Add the institutional layer that makes the world feel like it has
history rather than just events.

-   Institution simulation: factions, guilds, political entities

-   Institutional event types: schisms, elections, doctrinal disputes,
    diplomatic events

-   Phoneme-based name generation per people — distinct cultural
    naming conventions

-   Title and epithet accumulation for agents

-   Faction Record export

-   Follow mode: track specific agent or faction in side panel

## Phase 3 — Dungeons, Artifacts & Adventurers

Goal: Add the dungeon layer — sites of interest with their own
populations and histories.

-   Dungeon generation: procedural sites with generated purpose and
    population

-   Artifact generation: objects with histories and properties

-   Adventurer class agents: agents who pursue dungeon-delving goals

-   Zoomed dungeon view: tile-level ASCII display for active sites

-   Character Chronicle export

## Phase 4 — World Parametric Variance & Multiple Worlds

Goal: Each new world generates its own ruleset. Multiple world sessions
manageable.

-   Parametric world generation: temporal rate, political churn,
    weirdness coefficient, etc.

-   Preset flavors: The Long Bureaucracy, The Burning Provinces, etc.

-   World seed input

-   Session management: save/load worlds, start new world without losing
    old one

-   Multi-world support: Load World screen fully supports switching between
    up to 10 simultaneous active worlds, each fully independent

-   World Annals: high-level historical summary document, auto-generated
    per era

## Phase 5 — Polish, Depth & Voice

Goal: Make the prose and aesthetic genuinely distinctive. This is the
phase where the game acquires its personality.

-   Prose template expansion: significantly more varied log entry styles

-   Nested clause and subordinate clause generation for complex events

-   Narrative register variation per world parameter

-   Color and symbol tuning for map and UI — Brogue-quality ASCII
    expressiveness

-   Sound (optional): minimal ambient terminal sounds if platform
    supports it

-   Full export system: all log types, formatted output

# VIII. TECHNICAL NOTES

## Language & Stack

**Primary language:** Rust

**TUI library:** ratatui (with crossterm backend for cross-platform
support)

**Target platforms:** Windows (PowerShell/Windows Terminal), macOS
(Terminal/iTerm2), Linux

**Serialization:** serde + JSON or RON for world state save/load

**RNG:** rand crate with seedable RNG for reproducible world generation

Rust is well-suited here: it is fast enough for a simulation of this
complexity to run comfortably at high speed, memory-safe by default
(important for a long-running simulation), and has a mature ecosystem
for terminal UI work. The learning curve is real but Claude Code handles
most of the Rust-specific complexity.

## Performance Targets

-   Phase 1 sim at 20x speed: no perceptible lag on modest hardware
    (5-year-old mid-range laptop)

-   Phase 3 sim (with dungeons) at 5x speed: same target

-   World generation: under 3 seconds for full world gen

-   Log text generation: non-blocking; prose generation should not stall
    the sim tick

## Distribution & Cross-Platform (Future — Phase 5+)

Rust's cross-compilation support makes multi-platform distribution achievable
without rewriting any code. Target platforms are Windows, macOS, and Linux.

Steam distribution is a viable long-term target. Terminal/text-mode games have
shipped on Steam successfully (Dwarf Fortress is the primary precedent).

When the time comes:
- Tool: `cargo-bundle` for packaging per-platform executables
- Steam: submit as a standard application; the terminal window is the game window
- No changes to the game's code should be required for distribution builds
- A simple launcher script may be needed to ensure the correct terminal emulator
  opens on each platform with appropriate font/color settings

This is a post-Phase-5 concern. The architecture requires no advance preparation.

## File Structure

> */veridian-contraption*
>
> */src*
>
> *main.rs — entry point, app loop*
>
> *sim/ — simulation engine (world, agents, events, institutions)*
>
> *gen/ — generators (world, names, prose, dungeons)*
>
> *ui/ — ratatui layout, panels, input handling*
>
> *export/ — TXT export logic*
>
> *data/ — static data: phoneme tables, prose templates, event defs*
>
> */exports — player-exported TXT files land here*
>
> *Cargo.toml*

# IX. APPENDIX — SAMPLE OUTPUTS

## Sample Log Entries

> *The Registrar of Ambiguous Debts for the Confraternity of the
> Ossified Scale formally disputed the citizenship of Whelm
> Durr-Anquist, citing seventeen procedural irregularities and one
> metaphysical objection that the tribunal declined to record.*
>
> *Orrith the Twice-Appointed, having completed the Survey of the
> Insolvent Reaches, submitted a report of considerable length to the
> Bureau of Provisional Territories. The Bureau acknowledged receipt of
> the report and indicated it would be reviewed in due course. This was
> the forty-first year of the due course in question.*
>
> *A creature of uncertain taxonomy emerged from the Grevvin
> administrative district of Pelmwick and ate fourteen registered
> contracts before being subdued by a hastily convened committee. The
> committee subsequently voted to classify the incident as a clerical
> error.*

## Sample Agent Inspect View

> *═══════════════════════════════════════*
>
> *WHELM DURR-ANQUIST*
>
> *Formerly of the Grevvin Lateral Compact*
>
> *Disputed Citizen, Third Class (contested)*
>
> *═══════════════════════════════════════*
>
> *Age: 34 | Health: Fair | Location: Pelmwick*
>
> *Affiliated: Confraternity of the Ossified Scale (provisional)*
>
> *Current goal: Resolve citizenship dispute*
>
> *Disposition: Risk-averse | Highly institutional | Moderately
> paranoid*
>
> *─────────────────────────────────────*
>
> *RECENT CHRONICLE*
>
> *Day 43: Citizenship disputed on procedural grounds.*
>
> *Day 44: Filed counter-filing. Filing disputed.*
>
> *Day 45: Retained a licensed Dream Interpreter.*
>
> *Day 48: Dream interpretation inconclusive.*
>
> *═══════════════════════════════════════*
