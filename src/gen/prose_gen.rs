use rand::rngs::StdRng;
use rand::Rng;

use crate::sim::event::EventType;
use crate::sim::world::{NarrativeRegister, World};

// ===========================================================================
// WORD POOLS — organized by narrative register
// ===========================================================================

// --- Bureaucratic (default) ---
const BUREAUCRATIC_VERBS: &[&str] = &[
    "filed", "disputed", "remanded", "indicated", "noted",
    "acknowledged", "tabled", "deferred", "classified",
    "cross-referenced", "stamped", "countersigned", "amended",
    "appended", "logged", "archived", "processed",
    "ratified", "formalized",
];

const BUREAUCRATIC_NOUNS: &[&str] = &[
    "filing", "counter-filing", "tribunal",
    "committee", "subcommittee", "provisional review", "formal notation",
    "memorandum", "register", "ledger entry", "supplementary docket",
    "notice of intent", "petition", "procedural review",
    "brief", "disposition", "writ", "preliminary finding",
];

// --- Clinical ---
const CLINICAL_VERBS: &[&str] = &[
    "observed", "documented", "catalogued", "recorded", "measured",
    "classified", "assessed", "registered", "indexed", "annotated",
    "enumerated", "tabulated", "quantified", "surveyed", "logged",
];

const CLINICAL_NOUNS: &[&str] = &[
    "field report", "specimen report", "field notation", "data entry",
    "survey", "classification record", "classification", "specimen", "sample",
    "survey record", "catalogue entry", "preliminary finding", "case file",
    "taxonomic summary", "diagnostic summary",
];

// --- Lyrical ---
const LYRICAL_VERBS: &[&str] = &[
    "whispered", "murmured", "held", "unfolded", "stirred",
    "settled", "scattered", "gathered", "received",
    "absorbed", "noted", "traced", "folded",
    "carried", "dissolved",
];

const LYRICAL_NOUNS: &[&str] = &[
    "murmur", "fragment", "remnant", "trace",
    "silence", "tide", "passage", "breath",
    "threshold", "margin", "distance", "stillness",
    "shadow", "whisper", "pause",
];

// --- Ominous ---
const OMINOUS_VERBS: &[&str] = &[
    "darkened", "consumed", "unraveled", "severed", "condemned",
    "obliterated", "sealed", "annulled", "extinguished", "foreclosed",
    "revoked", "terminated", "silenced", "buried", "erased",
];

const OMINOUS_NOUNS: &[&str] = &[
    "decree", "portent", "reckoning", "judgment",
    "sentence", "proscription", "terminus", "notice",
    "warning", "silence", "void", "finality",
    "shadow", "weight", "mark",
];

// --- Conspiratorial ---
const CONSPIRATORIAL_VERBS: &[&str] = &[
    "intercepted", "concealed", "redirected", "substituted", "encrypted",
    "falsified", "redacted", "implicated", "surveilled", "compromised",
    "rerouted", "classified", "leaked", "suppressed", "obscured",
];

const CONSPIRATORIAL_NOUNS: &[&str] = &[
    "dossier", "dead drop", "cipher", "cell",
    "communiqué", "cover file", "safe house", "handler",
    "front organization", "surveillance report", "sealed file",
    "contact", "back channel", "cutout",
];

// --- Shared pools ---
const TEMPORAL_HEDGES: &[&str] = &[
    "in due course", "pending review", "subject to revision",
    "contingent upon further inquiry", "without prejudice",
    "at some future date to be determined",
    "upon completion of the relevant paperwork",
    "when circumstances permit", "at the discretion of the office",
    "following the customary delays",
];

const MUNDANE_CAUSES: &[&str] = &[
    "routine factors", "natural causes", "expected developments",
    "ordinary circumstances", "unremarkable conditions",
    "the usual administrative processes", "seasonal pressures",
];

const ABSURDIST_CAUSES: &[&str] = &[
    "a metaphysical irregularity", "an unresolved taxonomic dispute",
    "seventeen procedural objections", "a clerical error of uncertain origin",
    "the continued existence of the previous filing",
    "an unpaid obligation dating to the previous era",
    "a boundary dispute that has since been resolved but not acknowledged",
    "the unauthorized relocation of a surveyor's benchmark",
    "a prophecy that was officially retracted but not forgotten",
    "the spontaneous reclassification of adjacent terrain",
    "an audit that discovered more entries than there were things to audit",
    "a notarized complaint about the process of notarizing complaints",
    "the discovery that the relevant form had been printed in a language not yet invented",
    "a census that counted the same individual three times under different headings",
    "the physical relocation of a building that the records insist has not moved",
    "a jurisdictional overlap that assigned two offices authority over the same void",
    "the retirement of the only person who understood the filing system",
    "a calendar dispute that placed the same day in two different weeks",
];

// High-weirdness impossible causes — reported as mundane bureaucratic fact
const IMPOSSIBLE_CAUSES: &[&str] = &[
    "the retroactive non-occurrence of the event in question",
    "the discovery that the relevant location had been elsewhere at the time",
    "a weight discrepancy caused by the temporary absence of gravity in the filing room",
    "the formal reclassification of time as 'non-sequential' in the affected district",
    "the refusal of the river to continue flowing in the agreed-upon direction",
    "a surveyor's report indicating that the territory in question had declined to exist",
    "the observation that two buildings had exchanged positions without documentation",
    "the conclusion that the debt in question predated the existence of the debtor",
    "a geological fault that the arbitration board found to be acting in bad faith",
    "the determination that the document had been signed before it was written",
    "an accounting error caused by the number seven temporarily meaning something else",
    "the discovery that the meeting had occurred in a room that the building does not contain",
];

// ===========================================================================
// Core picking functions
// ===========================================================================

fn pick<'a>(options: &'a [&'a str], rng: &mut StdRng) -> &'a str {
    options[rng.gen_range(0..options.len())]
}


fn pick_verb<'a>(register: NarrativeRegister, rng: &mut StdRng) -> &'a str {
    match register {
        NarrativeRegister::Clinical => pick(CLINICAL_VERBS, rng),
        NarrativeRegister::Lyrical => pick(LYRICAL_VERBS, rng),
        NarrativeRegister::Ominous => pick(OMINOUS_VERBS, rng),
        NarrativeRegister::Conspiratorial => pick(CONSPIRATORIAL_VERBS, rng),
        NarrativeRegister::Bureaucratic => pick(BUREAUCRATIC_VERBS, rng),
    }
}

fn pick_noun<'a>(register: NarrativeRegister, rng: &mut StdRng) -> &'a str {
    match register {
        NarrativeRegister::Clinical => pick(CLINICAL_NOUNS, rng),
        NarrativeRegister::Lyrical => pick(LYRICAL_NOUNS, rng),
        NarrativeRegister::Ominous => pick(OMINOUS_NOUNS, rng),
        NarrativeRegister::Conspiratorial => pick(CONSPIRATORIAL_NOUNS, rng),
        NarrativeRegister::Bureaucratic => pick(BUREAUCRATIC_NOUNS, rng),
    }
}

/// Pick a cause — higher weirdness increases absurdist/impossible causes.
fn pick_cause(weirdness: f32, rng: &mut StdRng) -> String {
    if weirdness > 0.8 && rng.gen_bool(0.4) {
        pick(IMPOSSIBLE_CAUSES, rng).to_string()
    } else if weirdness > 0.4 || rng.gen_bool(weirdness as f64) {
        pick(ABSURDIST_CAUSES, rng).to_string()
    } else {
        pick(MUNDANE_CAUSES, rng).to_string()
    }
}

// ===========================================================================
// SUBORDINATE CLAUSE SYSTEM
// ===========================================================================

/// Generate a subordinate clause about an agent, sensitive to register and weirdness.
/// Returns None ~70% of the time — only ~30% of sentences get a clause.
fn maybe_subordinate_clause(
    name: &str,
    loc: &str,
    reg: NarrativeRegister,
    weirdness: f32,
    rng: &mut StdRng,
) -> Option<String> {
    if !rng.gen_bool(0.30) {
        return None;
    }
    Some(subordinate_clause(name, loc, reg, weirdness, rng))
}

/// Always generates a subordinate clause (for templates that require one).
fn subordinate_clause(
    _name: &str,
    loc: &str,
    reg: NarrativeRegister,
    weirdness: f32,
    rng: &mut StdRng,
) -> String {
    // Register-specific clause pools
    match reg {
        NarrativeRegister::Bureaucratic => bureaucratic_clause(loc, weirdness, rng),
        NarrativeRegister::Clinical => clinical_clause(loc, weirdness, rng),
        NarrativeRegister::Lyrical => lyrical_clause(loc, weirdness, rng),
        NarrativeRegister::Ominous => ominous_clause(loc, weirdness, rng),
        NarrativeRegister::Conspiratorial => conspiratorial_clause(loc, weirdness, rng),
    }
}

fn bureaucratic_clause(loc: &str, weirdness: f32, rng: &mut StdRng) -> String {
    match rng.gen_range(0..14) {
        0 => format!("whose previous {} remained unresolved", pick_noun(NarrativeRegister::Bureaucratic, rng)),
        1 => format!("whose standing in {} was a matter of some administrative ambiguity", loc),
        2 => format!("who had been the subject of a {} prior to the event in question", pick_noun(NarrativeRegister::Bureaucratic, rng)),
        3 => format!("whose documentation the office of {} had {} on three prior occasions", loc, pick_verb(NarrativeRegister::Bureaucratic, rng)),
        4 => format!("against whom a {} had been {} but never resolved", pick_noun(NarrativeRegister::Bureaucratic, rng), pick_verb(NarrativeRegister::Bureaucratic, rng)),
        5 => format!("whose file contained a marginal annotation reading 'see also: {}'", pick_cause(weirdness, rng)),
        6 => format!("whom the records of {} listed under two distinct and contradictory entries", loc),
        7 => format!("whose relationship to the matter was described as 'provisional'"),
        8 => format!("who had been {} by the relevant authorities {}", pick_verb(NarrativeRegister::Bureaucratic, rng), pick(TEMPORAL_HEDGES, rng)),
        9 => format!("having previously declined three formal invitations to do so"),
        10 => format!("a decision that would later be described as administratively convenient"),
        11 => format!("whose citizenship remained the subject of seventeen outstanding objections"),
        12 => format!("which the committee declined to record on grounds it found metaphysically objectionable"),
        _ => format!("whose presence in {} the {} had not yet formally acknowledged", loc, pick_noun(NarrativeRegister::Bureaucratic, rng)),
    }
}

fn clinical_clause(loc: &str, weirdness: f32, rng: &mut StdRng) -> String {
    match rng.gen_range(0..10) {
        0 => format!("whose physiological profile deviated from the documented baseline by a margin the {} termed 'notable'", pick_noun(NarrativeRegister::Clinical, rng)),
        1 => format!("whose prior {} in the records of {} contained several unresolved annotations", pick_noun(NarrativeRegister::Clinical, rng), loc),
        2 => "a subject whose behavioral patterns had been flagged for longitudinal study".to_string(),
        3 => format!("whose presence in {} had been {} in the field notes of three separate surveys", loc, pick_verb(NarrativeRegister::Clinical, rng)),
        4 => format!("a case previously cross-referenced under {}", pick_cause(weirdness, rng)),
        5 => "whose taxonomic classification remained, at best, provisional".to_string(),
        6 => format!("an individual the {} of {} had placed under ongoing observation", pick_noun(NarrativeRegister::Clinical, rng), loc),
        7 => "a subject exhibiting no fewer than four anomalous characteristics".to_string(),
        8 => format!("whose {} contained data that contradicted its own methodology", pick_noun(NarrativeRegister::Clinical, rng)),
        _ => "a specimen the literature had not anticipated".to_string(),
    }
}

fn lyrical_clause(loc: &str, _weirdness: f32, rng: &mut StdRng) -> String {
    match rng.gen_range(0..10) {
        0 => format!("whose name had once carried weight in {}, though the weight had since shifted", loc),
        1 => "who moved as though the ground remembered a different arrangement".to_string(),
        2 => format!("in whom {} had invested a kind of quiet expectation that was never quite fulfilled", loc),
        3 => "whose shadow arrived slightly before the rest".to_string(),
        4 => "about whom songs had been composed and then, for reasons unexplained, abandoned".to_string(),
        5 => format!("who had been absent from {} long enough for the absence to acquire a character of its own", loc),
        6 => "whose presence altered the quality of the available silence".to_string(),
        7 => "who carried the particular certainty of someone who has been wrong before in exactly this way".to_string(),
        8 => format!("for whom {} had always been more of a direction than a destination", loc),
        _ => "whose name, spoken aloud, seemed to occupy slightly more space than it should".to_string(),
    }
}

fn ominous_clause(loc: &str, _weirdness: f32, rng: &mut StdRng) -> String {
    match rng.gen_range(0..10) {
        0 => format!("whose name had been struck from the records of {} once before", loc),
        1 => "about whom the authorities had received warnings they chose not to file".to_string(),
        2 => "whose arrival had been anticipated by no one who would admit to it".to_string(),
        3 => format!("whom {} had cause to remember, and cause to wish otherwise", loc),
        4 => "whose prior dealings had left marks that did not heal in the conventional sense".to_string(),
        5 => "against whom the evidence was considerable but the witnesses were fewer".to_string(),
        6 => format!("whose {} was sealed for reasons the office would not discuss", pick_noun(NarrativeRegister::Ominous, rng)),
        7 => "who had survived events that the records describe only as 'concluded'".to_string(),
        8 => "whose shadow, according to one account, had been seen arriving separately".to_string(),
        _ => format!("whose previous time in {} had ended in a manner the survivors declined to specify", loc),
    }
}

fn conspiratorial_clause(loc: &str, weirdness: f32, rng: &mut StdRng) -> String {
    match rng.gen_range(0..10) {
        0 => format!("whose official records in {} contradicted at least two unofficial ones", loc),
        1 => format!("about whom a {} had been circulated among parties who deny receiving it", pick_noun(NarrativeRegister::Conspiratorial, rng)),
        2 => "whose stated purpose was, according to certain sources, a cover for the actual one".to_string(),
        3 => format!("who had been {} by at least one organization that claims not to exist", pick_verb(NarrativeRegister::Conspiratorial, rng)),
        4 => format!("whose connections to {} were, officially, coincidental (the word 'officially' doing considerable work in that sentence)", loc),
        5 => format!("a detail that several parties found interesting for reasons they were not prepared to share"),
        6 => format!("who maintained affiliations that overlapped in ways the {} described as 'merely geometrical'", pick_noun(NarrativeRegister::Conspiratorial, rng)),
        7 => format!("whose movements had been {} by parties operating under {}", pick_verb(NarrativeRegister::Conspiratorial, rng), pick_cause(weirdness, rng)),
        8 => "whose name appeared on three separate lists maintained by organizations unaware of each other's existence".to_string(),
        _ => format!("about whom the less said the better, though this has not prevented the {} of {} from saying quite a lot", pick_noun(NarrativeRegister::Conspiratorial, rng), loc),
    }
}

/// Generate a subordinate clause about an event/situation (not agent-specific).
fn event_subordinate_clause(reg: NarrativeRegister, weirdness: f32, rng: &mut StdRng) -> String {
    match reg {
        NarrativeRegister::Bureaucratic => match rng.gen_range(0..8) {
            0 => format!("a development the office attributed to {}", pick_cause(weirdness, rng)),
            1 => format!("which was {} without further comment", pick_verb(NarrativeRegister::Bureaucratic, rng)),
            2 => format!("a matter the relevant {} considered closed", pick_noun(NarrativeRegister::Bureaucratic, rng)),
            3 => format!("the consequences of which remain {}", pick(TEMPORAL_HEDGES, rng)),
            4 => "a fact that surprised no one who had been paying attention to the paperwork".to_string(),
            5 => format!("which the {} described as 'within the anticipated range of outcomes'", pick_noun(NarrativeRegister::Bureaucratic, rng)),
            6 => "an outcome that the relevant procedures had technically permitted all along".to_string(),
            _ => format!("though the {} has yet to issue a formal response", pick_noun(NarrativeRegister::Bureaucratic, rng)),
        },
        NarrativeRegister::Clinical => match rng.gen_range(0..6) {
            0 => format!("consistent with the patterns {} in the most recent {}", pick_verb(NarrativeRegister::Clinical, rng), pick_noun(NarrativeRegister::Clinical, rng)),
            1 => "a data point that warranted inclusion in the ongoing study".to_string(),
            2 => format!("which the {} noted without interpretation", pick_noun(NarrativeRegister::Clinical, rng)),
            3 => "an outcome that fell within the third standard deviation".to_string(),
            4 => "a result the methodology had not been designed to produce".to_string(),
            _ => format!("which was {} for future reference", pick_verb(NarrativeRegister::Clinical, rng)),
        },
        NarrativeRegister::Lyrical => match rng.gen_range(0..6) {
            0 => "as though the world had briefly considered a different arrangement and then thought better of it".to_string(),
            1 => "a change that settled into the landscape like a word one has been trying to remember".to_string(),
            2 => "though whether this constituted an ending or merely a pause remained, like most things, unclear".to_string(),
            3 => "which carried the particular weight of things that happen exactly once".to_string(),
            4 => "a fact that the local silence absorbed without complaint".to_string(),
            _ => "in the manner of events that are significant only to those who were not watching".to_string(),
        },
        NarrativeRegister::Ominous => match rng.gen_range(0..6) {
            0 => "and the implications were not lost on those who understood such things".to_string(),
            1 => "a development that certain parties had been expecting for some time".to_string(),
            2 => format!("which the {} received in a silence that was itself a kind of statement", pick_noun(NarrativeRegister::Ominous, rng)),
            3 => "an outcome that had the quality of inevitability about it".to_string(),
            4 => "and what followed was precisely what follows such things".to_string(),
            _ => "though the full consequences would not become apparent for some time".to_string(),
        },
        NarrativeRegister::Conspiratorial => match rng.gen_range(0..6) {
            0 => format!("though the official account (which differs from at least two unofficial ones) attributes it to {}", pick_cause(weirdness, rng)),
            1 => "a coincidence that required, by one estimate, the coordination of at least four parties".to_string(),
            2 => format!("which the {} found interesting enough to {} but not quite interesting enough to act upon", pick_noun(NarrativeRegister::Conspiratorial, rng), pick_verb(NarrativeRegister::Conspiratorial, rng)),
            3 => "a development that certain observers had predicted in documents that have since been misplaced".to_string(),
            4 => "the timing of which was, at minimum, suggestive".to_string(),
            _ => "and one is entitled to draw one's own conclusions".to_string(),
        },
    }
}

// ===========================================================================
// Helper: insert optional clause into a sentence about an agent
// ===========================================================================

/// Format "Name, CLAUSE," or just "Name" depending on whether a clause was generated.
fn name_with_optional_clause(
    name: &str, loc: &str, reg: NarrativeRegister, weirdness: f32, rng: &mut StdRng,
) -> String {
    match maybe_subordinate_clause(name, loc, reg, weirdness, rng) {
        Some(clause) => format!("{}, {},", name, clause),
        None => name.to_string(),
    }
}

// ===========================================================================
// MAIN PROSE GENERATION — generate_description
// ===========================================================================

/// Generate the prose description for an event.
pub fn generate_description(
    event_type: &EventType,
    agent_name: Option<&str>,
    location_name: Option<&str>,
    _tick: u64,
    rng: &mut StdRng,
    register: NarrativeRegister,
    weirdness: f32,
) -> String {
    let loc = location_name.unwrap_or("an unregistered locality");
    let name = agent_name.unwrap_or("an unnamed party");

    match event_type {
        EventType::AgentDied => gen_agent_died(name, loc, register, weirdness, rng),
        EventType::AgentArrived => gen_agent_arrived(name, loc, register, weirdness, rng),
        EventType::AgentDeparted => gen_agent_departed(name, loc, register, weirdness, rng),
        EventType::SettlementGrew => gen_settlement_grew(loc, register, weirdness, rng),
        EventType::SettlementShrank => gen_settlement_shrank(loc, register, weirdness, rng),
        EventType::WeatherEvent => gen_weather(loc, register, weirdness, rng),
        EventType::AgeEvent => gen_age_event(name, loc, register, weirdness, rng),
        EventType::AgentBorn => gen_agent_born(name, loc, register, weirdness, rng),
        EventType::NaturalDeath => gen_natural_death(name, loc, register, weirdness, rng),
        EventType::AgentEmigrated => gen_agent_emigrated(name, loc, register, weirdness, rng),
        EventType::AgentImmigrated => gen_agent_immigrated(name, loc, register, weirdness, rng),
        EventType::CensusReport => gen_census(loc, register, weirdness, rng),
        EventType::WorldGenesis => gen_genesis(register, rng),

        // These use their own dedicated generators
        EventType::AgentEnteredSite | EventType::AgentLeftSite =>
            format!("{} had business at a site near {}.", name, loc),
        EventType::ArtifactAcquired | EventType::ArtifactDelivered =>
            format!("{} was involved in an artifact transaction near {}.", name, loc),
        EventType::AdventurerDiedInSite =>
            format!("{} met their end in a site near {}.", name, loc),
        EventType::InstitutionFounded | EventType::InstitutionDissolved
        | EventType::SchismOccurred | EventType::DoctrineShifted
        | EventType::AllianceFormed | EventType::AllianceStrained
        | EventType::RivalryDeclared | EventType::MemberJoined
        | EventType::MemberDeparted | EventType::MemberExpelled
        | EventType::FactionDisbanded =>
            format!("An institutional event occurred near {}.", loc),
        EventType::InhabitantInteraction =>
            format!("{} had an encounter within a site near {}.", name, loc),
        EventType::EschatonFired =>
            "The world has been fundamentally reorganized.".to_string(),
        EventType::SeasonalTransition =>
            "The season has changed.".to_string(), // handled by dedicated generator
        EventType::RelationshipFormed | EventType::RelationshipChanged =>
            format!("{} has had a change in personal associations near {}.", name, loc),
    }
}

// ---------------------------------------------------------------------------
// Per-event-type generators (8+ variants each, register-sensitive)
// ---------------------------------------------------------------------------

fn gen_agent_died(name: &str, loc: &str, reg: NarrativeRegister, w: f32, rng: &mut StdRng) -> String {
    let nwc = name_with_optional_clause(name, loc, reg, w, rng);
    match rng.gen_range(0..10) {
        0 => format!("{} ceased to be present in {}. No formal {} was opened.", nwc, loc, pick_noun(reg, rng)),
        1 => format!("{} was removed from the census of {}. A clerk {} the absence in the margin of an unrelated ledger.", nwc, loc, pick_verb(reg, rng)),
        2 => format!("The {} of {} confirmed that {} is no longer extant. The relevant paperwork was completed {}.", pick_noun(reg, rng), loc, nwc, pick(TEMPORAL_HEDGES, rng)),
        3 => format!("{} expired, or was otherwise rendered absent. The vacancy has not yet been filled. The matter was {} under 'resolved by circumstance.'", nwc, pick_verb(reg, rng)),
        4 => format!("{} ceased to occupy their census entry in {}. This was {} and promptly misfiled.", nwc, loc, pick_verb(reg, rng)),
        5 => format!("The continued existence of {} in {} was downgraded from 'confirmed' to 'discontinued.' The relevant authorities were not notified {}.", name, loc, pick(TEMPORAL_HEDGES, rng)),
        6 => format!("{} was struck from the register of {}. The cause was attributed to {}.", nwc, loc, pick_cause(w, rng)),
        7 => match reg {
            NarrativeRegister::Ominous => format!("{} is dead. The {} of {} notes this without elaboration.", name, pick_noun(reg, rng), loc),
            NarrativeRegister::Lyrical => format!("The space that {} had occupied in {} remained, briefly, the shape of an absence — then that, too, was gone.", name, loc),
            NarrativeRegister::Clinical => format!("Subject {} was removed from the active {} of {} following confirmed cessation of vital indicators. Cause: {}.", name, pick_noun(reg, rng), loc, pick_cause(w, rng)),
            NarrativeRegister::Conspiratorial => format!("The official cause of death for {} was recorded as {}. The unofficial cause, which at least two parties are aware of, has not been committed to paper.", name, pick_cause(w, rng)),
            _ => format!("{} is no longer among the registered inhabitants of {}. A {} was {} to memorialize the administrative adjustment.", name, loc, pick_noun(reg, rng), pick_verb(reg, rng)),
        },
        8 => match reg {
            NarrativeRegister::Conspiratorial => format!("The {} of {} was updated to remove {}. The timing is, one supposes, coincidental.", pick_noun(reg, rng), loc, name),
            NarrativeRegister::Ominous => format!("{} ended. The {} does not specify how. It does not need to.", name, pick_noun(reg, rng)),
            NarrativeRegister::Clinical => format!("Mortality event {} in {}: subject {}, {}. Filed under standard protocols.", pick_noun(reg, rng), loc, name, event_subordinate_clause(reg, w, rng)),
            NarrativeRegister::Lyrical => format!("{} departed {} in the only way that admits no return. The ledger absorbed the fact with its customary patience.", name, loc),
            _ => format!("The file on {} was closed by the office of {}, {}. No appeal is anticipated.", name, loc, event_subordinate_clause(reg, w, rng)),
        },
        _ => format!("{} was formally {} from the rolls of {}. The {}, once completed, was not reviewed.", name, pick_verb(reg, rng), loc, pick_noun(reg, rng)),
    }
}

fn gen_agent_arrived(name: &str, loc: &str, reg: NarrativeRegister, w: f32, rng: &mut StdRng) -> String {
    let nwc = name_with_optional_clause(name, loc, reg, w, rng);
    match rng.gen_range(0..10) {
        0 => format!("{} arrived at {} without prior notice or evident purpose. The local {} {} the arrival {}.", nwc, loc, pick_noun(reg, rng), pick_verb(reg, rng), pick(TEMPORAL_HEDGES, rng)),
        1 => format!("{} appeared in {} bearing documentation of uncertain validity.", nwc, loc),
        2 => format!("The {} of {} recorded the arrival of {}, though the arrival itself was {} on procedural grounds.", pick_noun(reg, rng), loc, name, pick_verb(reg, rng)),
        3 => format!("{} entered {} with the air of someone who has been expected elsewhere. A {} was opened {}.", nwc, loc, pick_noun(reg, rng), pick(TEMPORAL_HEDGES, rng)),
        4 => format!("{} was provisionally noted in the register of {}. The notation included several caveats.", nwc, loc),
        5 => format!("{} arrived in {} claiming business with no one in particular. This claim was {} by the local office.", name, loc, pick_verb(reg, rng)),
        6 => format!("The presence of {} was detected in {} by means of {}. A {} was {} accordingly.", name, loc, pick_cause(w, rng), pick_noun(reg, rng), pick_verb(reg, rng)),
        7 => match reg {
            NarrativeRegister::Ominous => format!("{} arrived in {}. Those who saw the arrival did not speak of it afterward.", name, loc),
            NarrativeRegister::Lyrical => format!("{} came to {} in the manner of rain: unannounced, and with an air of having been inevitable.", name, loc),
            NarrativeRegister::Clinical => format!("New entry in the {} of {}: {}, origin undetermined, behavioral profile pending initial {}.", pick_noun(reg, rng), loc, name, pick_noun(reg, rng)),
            NarrativeRegister::Conspiratorial => format!("{} appeared in {}, which is to say: was allowed to appear in {}. The distinction matters to those who maintain the relevant {}.", name, loc, loc, pick_noun(reg, rng)),
            _ => format!("{} materialized in the administrative jurisdiction of {}, prompting the creation of a provisional {}.", nwc, loc, pick_noun(reg, rng)),
        },
        8 => format!("{} was {} in the vicinity of {}, {}.", name, pick_verb(reg, rng), loc, event_subordinate_clause(reg, w, rng)),
        _ => format!("The {} of {} now includes {}, a fact the office absorbed with its customary lack of enthusiasm.", pick_noun(reg, rng), loc, nwc),
    }
}

fn gen_agent_departed(name: &str, loc: &str, reg: NarrativeRegister, w: f32, rng: &mut StdRng) -> String {
    let nwc = name_with_optional_clause(name, loc, reg, w, rng);
    match rng.gen_range(0..10) {
        0 => format!("{} departed from {} citing personal obligations of an unspecified nature. The {} was {} accordingly.", nwc, loc, pick_noun(reg, rng), pick_verb(reg, rng)),
        1 => format!("{} left {} without filing the customary notice of departure.", nwc, loc),
        2 => format!("The departure of {} from {} was {} by the local office. Several unsigned documents were left behind.", name, loc, pick_verb(reg, rng)),
        3 => format!("{} concluded business in {} that no record describes and departed {}.", name, loc, pick(TEMPORAL_HEDGES, rng)),
        4 => format!("{} vacated {} under circumstances the local clerk declined to elaborate upon.", nwc, loc),
        5 => format!("The register of {} was updated to reflect the absence of {}. The update was attributed to {}.", loc, name, pick_cause(w, rng)),
        6 => format!("{} was no longer present in {} as of the most recent {}, a fact the office {} without comment.", name, loc, pick_noun(reg, rng), pick_verb(reg, rng)),
        7 => match reg {
            NarrativeRegister::Ominous => format!("{} is no longer in {}. The manner of departure does not bear examination.", name, loc),
            NarrativeRegister::Lyrical => format!("{} left {}, and the place — one might say — contracted slightly around the gap.", name, loc),
            NarrativeRegister::Clinical => format!("Subject {} exited the survey area of {}. Departure vector: undocumented. {} updated.", name, loc, pick_noun(reg, rng)),
            NarrativeRegister::Conspiratorial => format!("{} left {}. Or was removed from {}. The available evidence supports both readings.", name, loc, loc),
            _ => format!("{} departed from {}, {}.", nwc, loc, event_subordinate_clause(reg, w, rng)),
        },
        8 => format!("The {} of {} was amended to reflect the departure of {}, a correction the office described as 'overdue.'", pick_noun(reg, rng), loc, name),
        _ => format!("{} exited the administrative boundaries of {}. The reason given was {}, which the {} {} without visible reaction.", name, loc, pick_cause(w, rng), pick_noun(reg, rng), pick_verb(reg, rng)),
    }
}

fn gen_settlement_grew(loc: &str, reg: NarrativeRegister, w: f32, rng: &mut StdRng) -> String {
    match rng.gen_range(0..10) {
        0 => format!("The settlement of {} recorded an increase in its registered population. The {} was {} with reluctant precision.", loc, pick_noun(reg, rng), pick_verb(reg, rng)),
        1 => format!("The population of {} expanded by a number the census office described as 'within acceptable parameters.' The growth was attributed to {}.", loc, pick_cause(w, rng)),
        2 => format!("Additional residents were assigned provisional status in {}. A clerk expressed cautious optimism, then {} the statement.", loc, pick_verb(reg, rng)),
        3 => format!("{} experienced demographic expansion. The housing {} was updated {}.", loc, pick_noun(reg, rng), pick(TEMPORAL_HEDGES, rng)),
        4 => format!("The population figures of {} were revised upward, {}.", loc, event_subordinate_clause(reg, w, rng)),
        5 => format!("New arrivals in {} prompted the opening of a supplementary {}, to be reviewed {}.", loc, pick_noun(reg, rng), pick(TEMPORAL_HEDGES, rng)),
        6 => match reg {
            NarrativeRegister::Ominous => format!("{} grew. The growth was not requested.", loc),
            NarrativeRegister::Lyrical => format!("{} acquired new inhabitants the way a shore acquires driftwood — without intention, and with a sense of accumulation that only becomes apparent in retrospect.", loc),
            NarrativeRegister::Clinical => format!("Population delta for {} (current period): positive. Contributing factors: {}. {} updated.", loc, pick_cause(w, rng), pick_noun(reg, rng)),
            NarrativeRegister::Conspiratorial => format!("The population of {} increased. The official explanation is migration. The unofficial explanation, shared among those who pay attention to such things, involves {}.", loc, pick_cause(w, rng)),
            _ => format!("The census of {} was adjusted upward, a development the {} greeted with the enthusiasm typically reserved for additional paperwork.", loc, pick_noun(reg, rng)),
        },
        7 => format!("{} reported growth that the local office described as 'statistically unremarkable,' a phrase it deploys when it does not wish to be asked follow-up questions.", loc),
        8 => format!("The boundaries of {} now accommodate a larger population than the original {} had anticipated. An addendum was {}.", loc, pick_noun(reg, rng), pick_verb(reg, rng)),
        _ => format!("New names were added to the rolls of {}. The ink was still wet when the ledger was {} for the next review cycle.", loc, pick_verb(reg, rng)),
    }
}

fn gen_settlement_shrank(loc: &str, reg: NarrativeRegister, w: f32, rng: &mut StdRng) -> String {
    match rng.gen_range(0..10) {
        0 => format!("The population of {} experienced a documented reduction. The decrease was attributed to {}.", loc, pick_cause(w, rng)),
        1 => format!("Several addresses in {} were reclassified as 'potentially occupied.' The {} office {} the discrepancy but offered no correction.", loc, pick_noun(reg, rng), pick_verb(reg, rng)),
        2 => format!("A minor official in {} suggested the population figures may have been previously inflated. The shortfall was absorbed into the next quarter's projections.", loc),
        3 => format!("The demographic {} of {} showed contraction, {}.", pick_noun(reg, rng), loc, event_subordinate_clause(reg, w, rng)),
        4 => format!("{} recorded fewer inhabitants than its {} accounted for. The discrepancy remains under review {}.", loc, pick_noun(reg, rng), pick(TEMPORAL_HEDGES, rng)),
        5 => format!("The census of {} was revised downward, a correction the office described as 'overdue.' The underlying cause — {} — was not addressed.", loc, pick_cause(w, rng)),
        6 => match reg {
            NarrativeRegister::Ominous => format!("There are fewer in {} now. The {} does not record where they went.", loc, pick_noun(reg, rng)),
            NarrativeRegister::Lyrical => format!("{} thinned, the way a forest thins — not all at once, but with a cumulative quiet that only later registers as loss.", loc),
            NarrativeRegister::Clinical => format!("Negative population delta recorded for {}. Primary factor: {}. No intervention recommended at this time.", loc, pick_cause(w, rng)),
            NarrativeRegister::Conspiratorial => format!("The population of {} declined. Those who left did so without filing the usual documentation, which is either an oversight or something else entirely.", loc),
            _ => format!("The rolls of {} grew shorter. The {} attributed the change to {} and moved on to other matters.", loc, pick_noun(reg, rng), pick_cause(w, rng)),
        },
        7 => format!("{} contracted. The remaining population {} the development with an air of practiced indifference.", loc, pick_verb(reg, rng)),
        8 => format!("Certain census entries in {} were reclassified from 'active' to 'indeterminate,' {}.", loc, event_subordinate_clause(reg, w, rng)),
        _ => format!("The {} of {} was forced to acknowledge a reduction in headcount. The acknowledgment was itself {} {}.", pick_noun(reg, rng), loc, pick_verb(reg, rng), pick(TEMPORAL_HEDGES, rng)),
    }
}

fn gen_weather(loc: &str, reg: NarrativeRegister, w: f32, rng: &mut StdRng) -> String {
    match rng.gen_range(0..10) {
        0 => format!("Conditions in the vicinity of {} became unseasonably damp. This was attributed to {}, which the Bureau of Ambient Conditions is still reviewing.", loc, pick_cause(w, rng)),
        1 => format!("The weather near {} was classified by the meteorological office as 'within parameters,' though several residents {} the characterization.", loc, pick_verb(reg, rng)),
        2 => format!("An amber haze settled over {}. The phenomenon was attributed to {} and logged under the existing {} for atmospheric irregularities.", loc, pick_cause(w, rng), pick_noun(reg, rng)),
        3 => format!("{} experienced conditions that one official termed 'the usual arrangement.' A {} was opened {}, though expectations for its conclusion are modest.", loc, pick_noun(reg, rng), pick(TEMPORAL_HEDGES, rng)),
        4 => format!("A persistent low wind in the vicinity of {} prompted the filing of a {} with the regional office. The {} was {} but not acted upon.", loc, pick_noun(reg, rng), pick_noun(reg, rng), pick_verb(reg, rng)),
        5 => format!("The area surrounding {} was punctuated by brief intervals of something not quite rain. The cause was {} by the local office as 'atmospheric indifference.'", loc, pick_verb(reg, rng)),
        6 => match reg {
            NarrativeRegister::Ominous => format!("The sky above {} changed. It did not change back.", loc),
            NarrativeRegister::Lyrical => format!("Weather visited {} in the manner of an old acquaintance — familiar, unwelcome, and impossible to turn away at the door.", loc),
            NarrativeRegister::Clinical => format!("Atmospheric event logged near {}: parameters deviated from seasonal norms by a measurable but non-critical margin. {} {}.", loc, pick_noun(reg, rng), pick_verb(reg, rng)),
            NarrativeRegister::Conspiratorial => format!("The weather in {} shifted in a way that would be unremarkable if it had not coincided precisely with {}. Draw your own conclusions.", loc, pick_cause(w, rng)),
            _ => format!("The meteorological {} of {} {} a disturbance that the office classified as 'atmospheric in nature,' a category that technically includes everything.", pick_noun(reg, rng), loc, pick_verb(reg, rng)),
        },
        7 => format!("Conditions near {} became briefly extraordinary before returning to their customary mediocrity. The {} was {} accordingly.", loc, pick_noun(reg, rng), pick_verb(reg, rng)),
        8 => format!("The climate in the region of {} expressed an opinion that the local {} classified as 'seasonal.' This categorization was not contested.", loc, pick_noun(reg, rng)),
        _ => format!("Something fell from the sky near {}. Whether it was rain, ash, or the residue of {} was not determined before the {} was closed.", loc, pick_cause(w, rng), pick_noun(reg, rng)),
    }
}

fn gen_age_event(name: &str, loc: &str, reg: NarrativeRegister, w: f32, rng: &mut StdRng) -> String {
    let nwc = name_with_optional_clause(name, loc, reg, w, rng);
    match rng.gen_range(0..10) {
        0 => format!("{} of {} has persisted in the world for a notable duration. The actuarial tables regard this with skepticism.", name, loc),
        1 => format!("{} continues to occupy their census entry with considerable tenacity.", nwc),
        2 => format!("The longevity of {} has become a matter of minor administrative interest in {}. A {} was {} to document the fact.", name, loc, pick_noun(reg, rng), pick_verb(reg, rng)),
        3 => format!("{} has survived long enough to require the opening of a supplementary {} for their records.", nwc, pick_noun(reg, rng)),
        4 => format!("{} has reached an age that the {} of {} considers 'statistically noteworthy.'", nwc, pick_noun(reg, rng), loc),
        5 => format!("The continued existence of {} in {} was {} by the census office, which amended their file {}.", name, loc, pick_verb(reg, rng), pick(TEMPORAL_HEDGES, rng)),
        6 => match reg {
            NarrativeRegister::Ominous => format!("{} endures. This is not necessarily a comfort to those around them.", name),
            NarrativeRegister::Lyrical => format!("{} has been in {} long enough for the fact of their presence to become a kind of geography — unremarkable until one tries to imagine the place without it.", name, loc),
            NarrativeRegister::Clinical => format!("Subject {} exceeds the median lifespan for the relevant demographic cohort in {}. Further {} recommended.", name, loc, pick_noun(reg, rng)),
            NarrativeRegister::Conspiratorial => format!("The continued survival of {} is, depending on whom you ask, either unremarkable or deeply suspicious. The {} of {} keeps a separate file on the matter.", name, pick_noun(reg, rng), loc),
            _ => format!("{} has persisted in the records of {} for a period the {} describes as 'extended.' The description is technically accurate.", name, loc, pick_noun(reg, rng)),
        },
        7 => format!("The file on {} in {} has grown thick enough to require a secondary binder, {}.", name, loc, event_subordinate_clause(reg, w, rng)),
        8 => format!("{} was {} by the census of {} for the purpose of confirming continued existence. Existence was confirmed.", name, pick_verb(reg, rng), loc),
        _ => format!("The age of {} has exceeded the original projections of the {} of {}, an outcome the office attributed to {}.", name, pick_noun(reg, rng), loc, pick_cause(w, rng)),
    }
}

fn gen_agent_born(name: &str, loc: &str, reg: NarrativeRegister, w: f32, rng: &mut StdRng) -> String {
    let nwc = name_with_optional_clause(name, loc, reg, w, rng);
    match rng.gen_range(0..10) {
        0 => format!("{} entered the records of {} under circumstances the registrar described as 'standard.' A provisional identity number was assigned {}.", name, loc, pick(TEMPORAL_HEDGES, rng)),
        1 => format!("{} was added to the census of {} to the apparent surprise of the local office.", nwc, loc),
        2 => format!("The {} of {} {} the existence of {} amid paperwork that had already been prepared.", pick_noun(reg, rng), loc, pick_verb(reg, rng), name),
        3 => format!("{} materialized in the records of {} without the customary advance notification. A {} was opened {}.", nwc, loc, pick_noun(reg, rng), pick(TEMPORAL_HEDGES, rng)),
        4 => format!("A new entry for {} was {} in the register of {}, attributed to {}.", name, pick_verb(reg, rng), loc, pick_cause(w, rng)),
        5 => format!("{} was assigned to the census of {}. The relevant {} was completed with a speed that alarmed the processing clerk.", name, loc, pick_noun(reg, rng)),
        6 => match reg {
            NarrativeRegister::Ominous => format!("{} entered the world in {}. The {} took note.", name, loc, pick_noun(reg, rng)),
            NarrativeRegister::Lyrical => format!("{} arrived in {}, small and specific, the way all new things arrive — with an air of having been expected by no one and needed by everyone.", name, loc),
            NarrativeRegister::Clinical => format!("New subject {} registered in {}. Initial {} pending. File created.", name, loc, pick_noun(reg, rng)),
            NarrativeRegister::Conspiratorial => format!("A new individual, {}, appeared in the records of {}. Whether the appearance was natural or arranged is a question the {} has not yet addressed.", name, loc, pick_noun(reg, rng)),
            _ => format!("{} was enrolled in the records of {}. The clerk on duty described the process as 'unremarkable,' a word they apply to everything.", name, loc),
        },
        7 => format!("The census of {} has been expanded to include {}, {}.", loc, name, event_subordinate_clause(reg, w, rng)),
        8 => format!("{} was {} in {} under the heading 'new entries,' a category the office maintains with resignation.", name, pick_verb(reg, rng), loc),
        _ => format!("The {} of {} recorded a new constituent: {}. The paperwork is expected to be finalized {}.", pick_noun(reg, rng), loc, nwc, pick(TEMPORAL_HEDGES, rng)),
    }
}

fn gen_natural_death(name: &str, loc: &str, reg: NarrativeRegister, w: f32, rng: &mut StdRng) -> String {
    let nwc = name_with_optional_clause(name, loc, reg, w, rng);
    match rng.gen_range(0..10) {
        0 => format!("The census office of {} noted the cessation of {} with neither surprise nor sentiment. The file was closed {}.", loc, name, pick(TEMPORAL_HEDGES, rng)),
        1 => format!("{} was removed from the active rolls of {} under the heading 'expected attrition.' The {} was {} without ceremony.", nwc, loc, pick_noun(reg, rng), pick_verb(reg, rng)),
        2 => format!("The longevity of {} in {} reached its administrative conclusion. The actuarial tables registered no objection.", name, loc),
        3 => format!("{} expired in {}. The local office described the event as 'consistent with demographic projections' and allocated no further resources to the matter.", nwc, loc),
        4 => format!("The {} of {} was amended to reflect that {} has concluded their tenure among the living. The amendment was processed {}.", pick_noun(reg, rng), loc, name, pick(TEMPORAL_HEDGES, rng)),
        5 => format!("{} departed the census of {} by the only exit that requires no documentation.", name, loc),
        6 => match reg {
            NarrativeRegister::Ominous => format!("{} is gone. The {} of {} does not mourn. It was not designed to.", name, pick_noun(reg, rng), loc),
            NarrativeRegister::Lyrical => format!("{} left {} the way a candle leaves a room — not by going anywhere, but by ceasing to be the thing it was.", name, loc),
            NarrativeRegister::Clinical => format!("Subject {} ({}): vital functions terminated. Duration within expected parameters. {} closed.", name, loc, pick_noun(reg, rng)),
            NarrativeRegister::Conspiratorial => format!("The death of {} was recorded as natural, which it may well have been. The {} of {} has no reason to investigate further, and is therefore not investigating.", name, pick_noun(reg, rng), loc),
            _ => format!("{} was reclassified from 'living resident' to 'archival entry' in the records of {}. The clerk assigned to the transition described it as 'routine.'", name, loc),
        },
        7 => format!("The {} of {} {} the natural conclusion of {}. A supplementary {} was prepared but not distributed.", pick_noun(reg, rng), loc, pick_verb(reg, rng), nwc, pick_noun(reg, rng)),
        8 => format!("{} is no longer enumerated among the inhabitants of {}. The cause — advanced persistence in the world — was {} by the attending clerk.", name, loc, pick_verb(reg, rng)),
        _ => format!("The office of {} struck {} from the census with the efficiency reserved for outcomes that surprise no one. The space in the ledger was reallocated {}.", loc, nwc, pick(TEMPORAL_HEDGES, rng)),
    }
}

/// Generate emigration prose — agents departing for unknown regions.
pub fn generate_emigration(name: &str, loc: &str, reg: NarrativeRegister, w: f32, rng: &mut StdRng) -> String {
    gen_agent_emigrated(name, loc, reg, w, rng)
}

fn gen_agent_emigrated(name: &str, loc: &str, reg: NarrativeRegister, w: f32, rng: &mut StdRng) -> String {
    let nwc = name_with_optional_clause(name, loc, reg, w, rng);
    match rng.gen_range(0..10) {
        0 => format!("{} departed {} for regions not described in any current {}, citing obligations the office could not verify.", nwc, loc, pick_noun(reg, rng)),
        1 => format!("The {} of {} was updated to reflect the departure of {} toward territories the cartographic office does not acknowledge.", pick_noun(reg, rng), loc, name),
        2 => format!("{} left {} heading in a direction the compass declines to name. No forwarding address was provided.", name, loc),
        3 => format!("{} withdrew from {} and from all subsequent record-keeping. The clerk {} the departure under 'voluntary disappearance.'", nwc, loc, pick_verb(reg, rng)),
        4 => format!("{} indicated an intention to travel beyond the surveyed boundaries and was {} from the rolls of {} accordingly.", name, pick_verb(reg, rng), loc),
        5 => format!("The last reliable sighting of {} was at the border of {}'s jurisdiction. Subsequent reports are contradictory and have not been filed.", name, loc),
        6 => match reg {
            NarrativeRegister::Ominous => format!("{} walked out of {} and into whatever exists beyond the edge of the register. No one followed.", name, loc),
            NarrativeRegister::Lyrical => format!("{} left {}, choosing the horizon over the ledger — an exchange the census office considers, on balance, a net loss.", name, loc),
            NarrativeRegister::Clinical => format!("Subject {} exited the survey area of {} via unmapped route. Tracking discontinued. {} marked 'inconclusive.'", name, loc, pick_noun(reg, rng)),
            NarrativeRegister::Conspiratorial => format!("{} departed {}, supposedly for 'personal reasons.' Where they went, and who was waiting for them there, is a matter the {} has declined to record.", name, loc, pick_noun(reg, rng)),
            _ => format!("{} announced a departure from {} to 'elsewhere,' a destination the postal service does not serve.", nwc, loc),
        },
        7 => format!("{} vacated {} for parts unknown. The office assumes they still exist, but this assumption is not binding.", name, loc),
        8 => format!("{} left the known world by way of {}. Whether they arrived anywhere is outside the scope of this {}.", nwc, loc, pick_noun(reg, rng)),
        _ => format!("The file on {} was transferred to the 'departed for uncharted regions' drawer of {}, which is beginning to require a larger cabinet.", name, loc),
    }
}

/// Generate immigration prose — agents arriving from unknown regions.
pub fn generate_immigration(name: &str, loc: &str, reg: NarrativeRegister, w: f32, rng: &mut StdRng) -> String {
    gen_agent_immigrated(name, loc, reg, w, rng)
}

fn gen_agent_immigrated(name: &str, loc: &str, reg: NarrativeRegister, w: f32, rng: &mut StdRng) -> String {
    let nwc = name_with_optional_clause(name, loc, reg, w, rng);
    match rng.gen_range(0..10) {
        0 => format!("{} arrived in {} from territories not referenced in any active {}. Documentation was requested but not produced.", nwc, loc, pick_noun(reg, rng)),
        1 => format!("A new individual identifying as {} appeared at the boundary of {} bearing no credentials the local office recognized.", name, loc),
        2 => format!("{} materialized in the jurisdiction of {} from an origin the cartographic office has no record of. A provisional {} was opened.", name, loc, pick_noun(reg, rng)),
        3 => format!("The {} of {} {} a new arrival: {}, whose prior address is listed only as 'beyond the survey.'", pick_noun(reg, rng), loc, pick_verb(reg, rng), nwc),
        4 => format!("{} entered {} from the direction of unmapped territory. The immigration clerk {} the event with practiced disinterest.", name, loc, pick_verb(reg, rng)),
        5 => format!("An individual presenting as {} was added to the census of {}. Their account of their origins was heard, {} and not revisited.", name, loc, pick_verb(reg, rng)),
        6 => match reg {
            NarrativeRegister::Ominous => format!("{} came to {} from somewhere that does not appear on any map. The {} accepted them without question. Questions would have been unwise.", name, loc, pick_noun(reg, rng)),
            NarrativeRegister::Lyrical => format!("{} arrived in {} carrying nothing but a name and the faint suggestion of distance — the kind that accumulates in the posture of those who have walked a long time.", name, loc),
            NarrativeRegister::Clinical => format!("New subject {} registered at {}. Origin: undocumented territory. Initial {} pending. Quarantine waived per standard protocol.", name, loc, pick_noun(reg, rng)),
            NarrativeRegister::Conspiratorial => format!("{} appeared in {} claiming to have come from 'elsewhere.' The {} accepted this at face value, which is either negligent or deliberate.", name, loc, pick_noun(reg, rng)),
            _ => format!("{} was enrolled in the census of {} under the heading 'arrivals from uncharted regions,' a category the office had hoped would remain empty.", nwc, loc),
        },
        7 => format!("{} crossed into the jurisdiction of {} with no prior notification. The {} was {} to accommodate them, {}.", name, loc, pick_noun(reg, rng), pick_verb(reg, rng), pick(TEMPORAL_HEDGES, rng)),
        8 => format!("The border of {} admitted {}, who offered an explanation of their origin that the receiving clerk described as 'technically a sentence.'", loc, nwc),
        _ => format!("{} arrived at {} from beyond the known world. They were assigned a provisional identity number and a census entry, in that order.", name, loc),
    }
}

fn gen_census(loc: &str, reg: NarrativeRegister, w: f32, rng: &mut StdRng) -> String {
    match rng.gen_range(0..8) {
        0 => "A census was conducted. The results were filed.".to_string(),
        1 => format!("The periodic census was completed {}, {}.", pick(TEMPORAL_HEDGES, rng), event_subordinate_clause(reg, w, rng)),
        2 => format!("A {} was conducted to establish the current population. The figures were {} without enthusiasm.", pick_noun(reg, rng), pick_verb(reg, rng)),
        3 => format!("The count of inhabitants in {} revealed a number that the office described as 'a number.' Further characterization was declined.", loc),
        4 => match reg {
            NarrativeRegister::Conspiratorial => format!("The census was completed. Whether it counted what it was supposed to count is a matter of some debate among those who were paying attention."),
            NarrativeRegister::Ominous => "The census was conducted. The total was not what had been expected. The expectation was not revised.".to_string(),
            _ => format!("Census {} completed. The {} was {} for the permanent record.", pick_noun(reg, rng), pick_noun(reg, rng), pick_verb(reg, rng)),
        },
        5 => format!("The regular enumeration of the population was carried out with the usual combination of thoroughness and futility."),
        6 => format!("A count was taken. The count was {}, and the results were stored alongside previous counts that no one has consulted since.", pick_verb(reg, rng)),
        _ => format!("The census office published its findings, which confirmed that people exist, that they can be counted, and that the count is subject to revision {}", pick(TEMPORAL_HEDGES, rng)),
    }
}

fn gen_genesis(reg: NarrativeRegister, rng: &mut StdRng) -> String {
    match rng.gen_range(0..5) {
        0 => "The world stirs into being. Somewhere, a ledger is opened.".to_string(),
        1 => match reg {
            NarrativeRegister::Ominous => "The world began. It will not be asked whether it consented.".to_string(),
            NarrativeRegister::Lyrical => "The world opened like a sentence that has not yet decided what it means.".to_string(),
            NarrativeRegister::Conspiratorial => "The world began, or appeared to begin, which amounts to the same thing for practical purposes.".to_string(),
            NarrativeRegister::Clinical => "World instance initialized. Parameters within expected ranges. Observation commences.".to_string(),
            _ => "A new world was established. The first entry in its ledger reads: 'Begun.'".to_string(),
        },
        2 => "The world materialized with the quiet confidence of something that has always existed and merely needed to be noticed.".to_string(),
        3 => format!("In the beginning, there was paperwork. The {} was {}.", pick_noun(reg, rng), pick_verb(reg, rng)),
        _ => "The world came into existence. Administration followed immediately.".to_string(),
    }
}

// ===========================================================================
// INSTITUTIONAL PROSE — now register-sensitive
// ===========================================================================

/// Generate prose for institutional events.
pub fn generate_institutional_description(
    event_type: &EventType,
    agent_name: Option<&str>,
    inst_name: Option<&str>,
    other_name: Option<&str>,
    rng: &mut StdRng,
    register: NarrativeRegister,
    weirdness: f32,
) -> String {
    let inst = inst_name.unwrap_or("an unnamed body");
    let agent = agent_name.unwrap_or("a party of uncertain identity");
    let other = other_name.unwrap_or("another organization");

    match event_type {
        EventType::InstitutionFounded => gen_inst_founded(agent, inst, other, register, weirdness, rng),
        EventType::InstitutionDissolved => gen_inst_dissolved(inst, register, weirdness, rng),
        EventType::SchismOccurred => gen_schism(inst, register, weirdness, rng),
        EventType::DoctrineShifted => gen_doctrine_shift(inst, register, weirdness, rng),
        EventType::AllianceFormed => gen_alliance_formed(inst, other, register, weirdness, rng),
        EventType::AllianceStrained => gen_alliance_strained(inst, other, register, weirdness, rng),
        EventType::RivalryDeclared => gen_rivalry(inst, other, register, weirdness, rng),
        EventType::MemberJoined => gen_member_joined(agent, inst, register, weirdness, rng),
        EventType::MemberDeparted => gen_member_departed(agent, inst, register, weirdness, rng),
        EventType::MemberExpelled => gen_member_expelled(agent, inst, register, weirdness, rng),
        _ => format!("An institutional matter involving {} was resolved, or at least {}.", inst, pick_verb(register, rng)),
    }
}

fn gen_inst_founded(agent: &str, inst: &str, loc: &str, reg: NarrativeRegister, w: f32, rng: &mut StdRng) -> String {
    match rng.gen_range(0..10) {
        0 => format!("{} has been formally established near {}, by the initiative of {}. Its charter has been {} and a {} opened.", inst, loc, agent, pick_verb(reg, rng), pick_noun(reg, rng)),
        1 => format!("A new organization, {}, was founded by {} in the vicinity of {}. The relevant authorities were notified {}.", inst, agent, loc, pick(TEMPORAL_HEDGES, rng)),
        2 => format!("{} brought {} into existence near {}. The necessary paperwork was completed with a thoroughness that surprised the filing office.", agent, inst, loc),
        3 => format!("The founding of {} was {} near {} by {}, who cited {} as the motivating factor.", inst, pick_verb(reg, rng), loc, agent, pick_cause(w, rng)),
        4 => format!("{} established {} in {}. A {} was immediately opened to govern its activities, though its scope remains {}.", agent, inst, loc, pick_noun(reg, rng), pick(TEMPORAL_HEDGES, rng)),
        5 => match reg {
            NarrativeRegister::Ominous => format!("{} now exists in {}. {} is responsible. The consequences have not yet been measured.", inst, loc, agent),
            NarrativeRegister::Lyrical => format!("{} coalesced in {} around {}, the way institutions do — not quite deliberately, but with a momentum that, once begun, admits no reversal.", inst, loc, agent),
            NarrativeRegister::Clinical => format!("New institutional entity {} registered in {}. Founding agent: {}. Initial {} pending.", inst, loc, agent, pick_noun(reg, rng)),
            NarrativeRegister::Conspiratorial => format!("{} was officially founded by {} in {}. Unofficially, the groundwork had been laid considerably earlier by parties who prefer not to be credited.", inst, agent, loc),
            _ => format!("{} established {} near {}, {}.", agent, inst, loc, event_subordinate_clause(reg, w, rng)),
        },
        6 => format!("{}, acting near {}, created {}. The first order of business was the creation of a {} to handle subsequent orders of business.", agent, loc, inst, pick_noun(reg, rng)),
        7 => format!("The {} of {} was {} in the records as the location where {} brought {} into formal existence.", pick_noun(reg, rng), loc, pick_verb(reg, rng), agent, inst),
        8 => format!("{} was chartered near {} by {}, {}. The charter runs to several pages, most of which concern procedure.", inst, loc, agent, event_subordinate_clause(reg, w, rng)),
        _ => format!("{} founded {} in the vicinity of {}. The world now contains one more organization than it did previously. The paperwork to confirm this has been {}.", agent, inst, loc, pick_verb(reg, rng)),
    }
}

fn gen_inst_dissolved(inst: &str, reg: NarrativeRegister, w: f32, rng: &mut StdRng) -> String {
    match rng.gen_range(0..10) {
        0 => format!("{} has ceased to function as a going concern. Its records have been transferred to the Archive of Defunct Bodies.", inst),
        1 => format!("The dissolution of {} was {} without ceremony. Its remaining assets, if any, were not enumerated.", inst, pick_verb(reg, rng)),
        2 => format!("{} was formally dissolved. The reasons given were 'insufficient membership and declining relevance.' A {} was filed {}.", inst, pick_noun(reg, rng), pick(TEMPORAL_HEDGES, rng)),
        3 => format!("The {} formerly known as {} no longer exists in any administratively meaningful sense. Its {} was {} for the final time.", pick_noun(reg, rng), inst, pick_noun(reg, rng), pick_verb(reg, rng)),
        4 => format!("{} was struck from the register of active organizations, a consequence of {}. No successor body was designated.", inst, pick_cause(w, rng)),
        5 => match reg {
            NarrativeRegister::Ominous => format!("{} ended. The silence where it had been is noticeable.", inst),
            NarrativeRegister::Lyrical => format!("{} dissolved the way salt dissolves — completely, and leaving only the faintest change in what remains.", inst),
            NarrativeRegister::Clinical => format!("Institutional entity {} classified as dissolved. Cause: {}. Records archived.", inst, pick_cause(w, rng)),
            NarrativeRegister::Conspiratorial => format!("{} was officially dissolved. Whether it has actually ceased operations is, of course, another question entirely.", inst),
            _ => format!("{} was formally dissolved, {}.", inst, event_subordinate_clause(reg, w, rng)),
        },
        6 => format!("The final act of {} was to file the paperwork for its own dissolution. The irony was {} by no one.", inst, pick_verb(reg, rng)),
        7 => format!("{} collapsed under the weight of {}, which had been accumulating for longer than anyone in the organization cared to admit.", inst, pick_cause(w, rng)),
        8 => format!("The {} of {} was closed for the last time. The clerk who performed this duty described the experience as 'procedurally unremarkable.'", pick_noun(reg, rng), inst),
        _ => format!("{} no longer exists. The {} confirms this. So does the absence of anyone willing to dispute it.", inst, pick_noun(reg, rng)),
    }
}

fn gen_schism(inst: &str, reg: NarrativeRegister, w: f32, rng: &mut StdRng) -> String {
    match rng.gen_range(0..10) {
        0 => format!("A doctrinal rupture within {} has produced irreconcilable factions. The {} was {} but could not contain the disagreement.", inst, pick_noun(reg, rng), pick_verb(reg, rng)),
        1 => format!("{} suffered a schism of considerable administrative consequence. The immediate cause was {}.", inst, pick_cause(w, rng)),
        2 => format!("Internal disagreements within {} escalated beyond the capacity of its mediation procedures. A {} was convened but dissolved before reaching conclusion.", inst, pick_noun(reg, rng)),
        3 => format!("The membership of {} fractured along lines that the organization's charter had not anticipated. Each faction {} the other's legitimacy.", inst, pick_verb(reg, rng)),
        4 => format!("{} split into opposing camps over a matter that both sides describe as 'fundamental.' Outside observers characterized the dispute as 'largely procedural.'", inst),
        5 => match reg {
            NarrativeRegister::Ominous => format!("{} broke. What remains is not what was.", inst),
            NarrativeRegister::Lyrical => format!("{} divided against itself, and the division — like all divisions — created two things where one had been, each smaller and more certain than the whole.", inst),
            NarrativeRegister::Clinical => format!("Organizational fission event in {}. Precipitating factor: {}. Two successor entities projected.", inst, pick_cause(w, rng)),
            NarrativeRegister::Conspiratorial => format!("{} experienced a schism that, according to well-placed sources, had been engineered by a faction within the organization that had been planning this for some time.", inst),
            _ => format!("{} fractured, {}.", inst, event_subordinate_clause(reg, w, rng)),
        },
        6 => format!("A disagreement within {} over {} became irreconcilable. The original point of contention has since been forgotten; the animosity has not.", inst, pick_cause(w, rng)),
        7 => format!("The internal {} of {} collapsed when a minority faction {} the majority's position. The majority {} the minority's right to do so.", pick_noun(reg, rng), inst, pick_verb(reg, rng), pick_verb(reg, rng)),
        8 => format!("{} split. Both halves claim to be the whole. Neither has the membership to prove it.", inst),
        _ => format!("A schism within {} was attributed to {}, though the underlying tensions had been {} for considerably longer.", inst, pick_cause(w, rng), pick_verb(reg, rng)),
    }
}

fn gen_doctrine_shift(inst: &str, reg: NarrativeRegister, w: f32, rng: &mut StdRng) -> String {
    match rng.gen_range(0..10) {
        0 => format!("{} has officially revised one of its foundational positions. The previous position was stricken from the record {}.", inst, pick(TEMPORAL_HEDGES, rng)),
        1 => format!("A doctrinal adjustment within {} was announced without explanation. Members were instructed to update their personal copies of the {}.", inst, pick_noun(reg, rng)),
        2 => format!("{} quietly amended its official doctrine, citing {}. The amendment was {} by the internal {}.", inst, pick_cause(w, rng), pick_verb(reg, rng), pick_noun(reg, rng)),
        3 => format!("The doctrinal {} of {} was revised. The old position, which had stood for some time, was replaced with one that the leadership described as 'more current.'", pick_noun(reg, rng), inst),
        4 => format!("{} issued a correction to its stated beliefs, {}.", inst, event_subordinate_clause(reg, w, rng)),
        5 => match reg {
            NarrativeRegister::Ominous => format!("{} changed what it believes. What it believed before has been made to not have existed.", inst),
            NarrativeRegister::Lyrical => format!("The doctrines of {} shifted, as doctrines do — not with the violence of collapse but with the slow inevitability of a river finding a new course.", inst),
            NarrativeRegister::Clinical => format!("Doctrinal revision logged for {}. Previous position: deprecated. Current position: see updated {}.", inst, pick_noun(reg, rng)),
            NarrativeRegister::Conspiratorial => format!("{} revised its official doctrine. Interestingly, the new position closely resembles one that had been circulating in certain {} for months.", inst, pick_noun(reg, rng)),
            _ => format!("{} altered its official position. The alteration was described as a 'clarification,' a word that does not quite describe what occurred.", inst),
        },
        6 => format!("The official beliefs of {} were updated to account for {}. Members who held the previous beliefs were given a grace period of unspecified duration.", inst, pick_cause(w, rng)),
        7 => format!("{} announced that what it had previously held to be true was now held to be less true. The new truth was {} without dissent, or at least without audible dissent.", inst, pick_verb(reg, rng)),
        8 => format!("A doctrinal correction within {} was {} so quietly that several members did not notice the change until the following {}.", inst, pick_verb(reg, rng), pick_noun(reg, rng)),
        _ => format!("{} revised its foundational principles. The revision was attributed to {} and was expected to have consequences {}.", inst, pick_cause(w, rng), pick(TEMPORAL_HEDGES, rng)),
    }
}

fn gen_alliance_formed(inst: &str, other: &str, reg: NarrativeRegister, w: f32, rng: &mut StdRng) -> String {
    match rng.gen_range(0..10) {
        0 => format!("{} and {} have entered into a formal arrangement of mutual benefit. The {} was ratified {}.", inst, other, pick_noun(reg, rng), pick(TEMPORAL_HEDGES, rng)),
        1 => format!("An alliance between {} and {} was {} with the appropriate signatures. The terms are to be reviewed {}.", inst, other, pick_verb(reg, rng), pick(TEMPORAL_HEDGES, rng)),
        2 => format!("{} extended a hand of cooperation to {}. The hand was accepted, provisionally, and a joint {} was established.", inst, other, pick_noun(reg, rng)),
        3 => format!("Following protracted negotiation, {} and {} have agreed to coordinate their activities. A shared {} was {} to formalize the arrangement.", inst, other, pick_noun(reg, rng), pick_verb(reg, rng)),
        4 => format!("{} and {} announced an alliance, surprising observers who had expected continued hostility. The agreement was attributed to {}.", inst, other, pick_cause(w, rng)),
        5 => match reg {
            NarrativeRegister::Ominous => format!("{} and {} have aligned. The combined weight of their intentions has not yet been directed.", inst, other),
            NarrativeRegister::Lyrical => format!("{} and {} found common ground — or perhaps the ground found them, the way it sometimes does when organizations have exhausted every other option.", inst, other),
            NarrativeRegister::Clinical => format!("Cooperative agreement formalized between {} and {}. Terms: see attached {}. Duration: indeterminate.", inst, other, pick_noun(reg, rng)),
            NarrativeRegister::Conspiratorial => format!("{} and {} are now officially allied. The unofficial alliance, which predates the official one by some margin, is not discussed.", inst, other),
            _ => format!("{} and {} formalized their cooperation, {}.", inst, other, event_subordinate_clause(reg, w, rng)),
        },
        6 => format!("The alliance between {} and {} was signed with the kind of ceremony that suggests both parties expect to need the document as evidence later.", inst, other),
        7 => format!("{} and {} agreed to mutual cooperation, a decision the {} of both organizations {} with varying degrees of sincerity.", inst, other, pick_noun(reg, rng), pick_verb(reg, rng)),
        8 => format!("{} joined forces with {}, or at least joined {}. The forces may follow {}.", inst, other, pick_noun(reg, rng), pick(TEMPORAL_HEDGES, rng)),
        _ => format!("A formal pact between {} and {} was achieved. The process required {} and a quantity of paperwork that the {} described as 'regrettable but necessary.'", inst, other, pick_cause(w, rng), pick_noun(reg, rng)),
    }
}

fn gen_alliance_strained(inst: &str, other: &str, reg: NarrativeRegister, w: f32, rng: &mut StdRng) -> String {
    match rng.gen_range(0..10) {
        0 => format!("Relations between {} and {} have deteriorated over a matter that both parties describe differently. A {} was {} but not addressed.", inst, other, pick_noun(reg, rng), pick_verb(reg, rng)),
        1 => format!("{} {} a formal complaint against {}. The complaint was acknowledged {}.", inst, pick_verb(reg, rng), other, pick(TEMPORAL_HEDGES, rng)),
        2 => format!("Tensions between {} and {} reached a level that required the appointment of a mediator. No mediator was appointed, owing to {}.", inst, other, pick_cause(w, rng)),
        3 => format!("The relationship between {} and {} was downgraded from 'cooperative' to 'under review,' {}.", inst, other, event_subordinate_clause(reg, w, rng)),
        4 => format!("{} and {} exchanged formal objections regarding {}. Neither objection was resolved. Both were filed.", inst, other, pick_cause(w, rng)),
        5 => match reg {
            NarrativeRegister::Ominous => format!("The accord between {} and {} is deteriorating. Neither party admits it. Both can feel it.", inst, other),
            NarrativeRegister::Lyrical => format!("What had been cooperation between {} and {} began to resemble something else — not quite hostility, but a careful, mutual withdrawal of goodwill.", inst, other),
            NarrativeRegister::Clinical => format!("Alliance degradation between {} and {}: satisfaction indices below threshold. Precipitating factor: {}.", inst, other, pick_cause(w, rng)),
            NarrativeRegister::Conspiratorial => format!("The rift between {} and {} deepened, which is exactly what certain parties had intended when they introduced {} into the proceedings.", inst, other, pick_cause(w, rng)),
            _ => format!("The alliance between {} and {} showed signs of strain, {}.", inst, other, event_subordinate_clause(reg, w, rng)),
        },
        6 => { let cause = pick_cause(w, rng); format!("{} and {} are no longer on speaking terms, or more precisely, are no longer on {} terms. The proximate cause — {} — was the last straw.", inst, other, pick_noun(reg, rng), cause) },
        7 => format!("A dispute between {} and {} escalated to the level where both parties began maintaining separate {}.", inst, other, pick_noun(reg, rng)),
        8 => format!("{} formally expressed dissatisfaction with {}. The expression was {} and filed under 'pending grievances,' a category of growing size.", inst, other, pick_verb(reg, rng)),
        _ => format!("The understanding between {} and {} was tested by {} and found wanting.", inst, other, pick_cause(w, rng)),
    }
}

fn gen_rivalry(inst: &str, other: &str, reg: NarrativeRegister, w: f32, rng: &mut StdRng) -> String {
    match rng.gen_range(0..10) {
        0 => format!("{} has declared {} to be operating in opposition to its interests. A {} was {} to document the grievance.", inst, other, pick_noun(reg, rng), pick_verb(reg, rng)),
        1 => format!("A state of formal rivalry now exists between {} and {}. The declaration cited {} as the precipitating factor.", inst, other, pick_cause(w, rng)),
        2 => format!("{} publicly denounced {} in terms that left little room for diplomatic interpretation. A response is expected {}.", inst, other, pick(TEMPORAL_HEDGES, rng)),
        3 => format!("The {} between {} and {} has been officially classified as adversarial. Both organizations {} the classification.", pick_noun(reg, rng), inst, other, pick_verb(reg, rng)),
        4 => format!("{} issued a formal denunciation of {}, citing grievances accumulated over a period the {} described as 'sufficient.'", inst, other, pick_noun(reg, rng)),
        5 => match reg {
            NarrativeRegister::Ominous => format!("{} and {} are now enemies. The word 'enemies' is used here in its most complete sense.", inst, other),
            NarrativeRegister::Lyrical => format!("Between {} and {}, the distance widened — not in geography but in the more consequential dimensions of intent and regard.", inst, other),
            NarrativeRegister::Clinical => format!("Rivalry status confirmed between {} and {}. Cause classification: {}. Projected duration: indeterminate.", inst, other, pick_cause(w, rng)),
            NarrativeRegister::Conspiratorial => format!("{} and {} have become open rivals, which is to say: their longstanding private rivalry has become public. The transition surprised no one who had been monitoring the relevant {}.", inst, other, pick_noun(reg, rng)),
            _ => format!("{} and {} declared formal rivalry, {}.", inst, other, event_subordinate_clause(reg, w, rng)),
        },
        6 => format!("{} named {} as an adversary, a designation that carries administrative consequences neither party has fully considered.", inst, other),
        7 => format!("The animosity between {} and {} was formalized. Both organizations agree on nothing except the depth of their disagreement.", inst, other),
        8 => format!("{} and {} have moved from 'strained relations' to 'open opposition,' a reclassification the {} {} with a finality that suggests further reclassification is unlikely.", inst, other, pick_noun(reg, rng), pick_verb(reg, rng)),
        _ => format!("Rivalry between {} and {} was declared on the grounds of {}, though the roots of the dispute almost certainly predate the stated cause.", inst, other, pick_cause(w, rng)),
    }
}

fn gen_member_joined(agent: &str, inst: &str, reg: NarrativeRegister, w: f32, rng: &mut StdRng) -> String {
    match rng.gen_range(0..10) {
        0 => format!("{} was admitted to the ranks of {}. The initiation {} was {}.", agent, inst, pick_noun(reg, rng), pick_verb(reg, rng)),
        1 => format!("{} formally joined {}. Their provisional membership period begins immediately and extends {}.", agent, inst, pick(TEMPORAL_HEDGES, rng)),
        2 => format!("{} accepted {} as a member, following a review process described as 'perfunctory.' The relevant {} was completed.", inst, agent, pick_noun(reg, rng)),
        3 => format!("The membership rolls of {} were updated to include {}, a development the {} {} without further comment.", inst, agent, pick_noun(reg, rng), pick_verb(reg, rng)),
        4 => format!("{} applied for membership in {} and was accepted {}. The application cited {} as motivation.", agent, inst, pick(TEMPORAL_HEDGES, rng), pick_cause(w, rng)),
        5 => match reg {
            NarrativeRegister::Ominous => format!("{} joined {}. This is noted without comment, though comments suggest themselves.", agent, inst),
            NarrativeRegister::Lyrical => format!("{} entered {}, the way one enters a conversation already in progress — with the mild disorientation of someone who has missed the opening but intends to stay for the conclusion.", agent, inst),
            NarrativeRegister::Clinical => format!("Membership event: {} added to roster of {}. Affiliation type: standard. {} created.", agent, inst, pick_noun(reg, rng)),
            NarrativeRegister::Conspiratorial => format!("{} joined {}, or was placed there. The distinction between voluntary membership and strategic insertion is, in this case, unclear.", agent, inst),
            _ => format!("{} became a member of {}, {}.", agent, inst, event_subordinate_clause(reg, w, rng)),
        },
        6 => format!("{} enrolled in {} with the confidence of someone who has read the charter. Whether they read the bylaws is another matter.", agent, inst),
        7 => format!("The {} of {} now includes {}, who was {} after a review the office described as 'adequate.'", pick_noun(reg, rng), inst, agent, pick_verb(reg, rng)),
        8 => format!("{} joined {}. The organization's internal {} was amended to reflect the addition.", agent, inst, pick_noun(reg, rng)),
        _ => format!("{} was added to the membership of {}. The processing clerk {} the paperwork and moved on to the next item.", agent, inst, pick_verb(reg, rng)),
    }
}

fn gen_member_departed(agent: &str, inst: &str, reg: NarrativeRegister, w: f32, rng: &mut StdRng) -> String {
    match rng.gen_range(0..10) {
        0 => format!("{} departed from {}, citing reasons that were not entered into the record. The exit {} was declined.", agent, inst, pick_noun(reg, rng)),
        1 => format!("{} terminated their affiliation with {}. The {} was updated {}.", agent, inst, pick_noun(reg, rng), pick(TEMPORAL_HEDGES, rng)),
        2 => format!("{} quietly removed itself from the membership rolls of {}, a process the {} described as 'routine.'", agent, inst, pick_noun(reg, rng)),
        3 => format!("The departure of {} from {} was {} by the internal office. The cause was attributed to {}.", agent, inst, pick_verb(reg, rng), pick_cause(w, rng)),
        4 => format!("{} ceased to be affiliated with {}. The relevant {} was {} and the matter considered closed.", agent, inst, pick_noun(reg, rng), pick_verb(reg, rng)),
        5 => match reg {
            NarrativeRegister::Ominous => format!("{} left {}. The reasons are known to those who need to know them.", agent, inst),
            NarrativeRegister::Lyrical => format!("{} departed from {} with the particular quiet of someone who has been leaving for some time and has only now made it official.", agent, inst),
            NarrativeRegister::Clinical => format!("Membership termination: {} disaffiliated from {}. Exit classification: voluntary. {} archived.", agent, inst, pick_noun(reg, rng)),
            NarrativeRegister::Conspiratorial => format!("{} left {}, citing personal reasons. Whether these reasons are personal or were made to appear personal is a question the {} has not addressed.", agent, inst, pick_noun(reg, rng)),
            _ => format!("{} withdrew from {}, {}.", agent, inst, event_subordinate_clause(reg, w, rng)),
        },
        6 => format!("{} resigned from {} in a manner the {} described as 'without incident,' a description that may be technically accurate.", agent, inst, pick_noun(reg, rng)),
        7 => format!("The name of {} was removed from the rolls of {}. The removal was {} with the efficiency of an office that has performed this procedure before.", agent, inst, pick_verb(reg, rng)),
        8 => format!("{} is no longer a member of {}, a fact that both parties seem to regard with equanimity.", agent, inst),
        _ => format!("The affiliation between {} and {} was dissolved. The {} attributes this to {} and considers the matter closed.", agent, inst, pick_noun(reg, rng), pick_cause(w, rng)),
    }
}

fn gen_member_expelled(agent: &str, inst: &str, reg: NarrativeRegister, w: f32, rng: &mut StdRng) -> String {
    match rng.gen_range(0..10) {
        0 => format!("{} was expelled from {} on grounds that the internal {} declined to make public.", agent, inst, pick_noun(reg, rng)),
        1 => format!("{} formally removed {} from its membership for reasons described as 'procedural.' The expulsion {} cited {} infractions.", inst, agent, pick_noun(reg, rng), rng.gen_range(3..=17)),
        2 => format!("{} was ejected from {}. The {} notice cited {}, only three of which were specified.", agent, inst, pick_noun(reg, rng), pick_cause(w, rng)),
        3 => format!("The membership of {} in {} was revoked following a {} that the internal {} described as 'conclusive.'", agent, inst, pick_noun(reg, rng), pick_noun(reg, rng)),
        4 => format!("{} {} the removal of {} from its rolls. The decision, once {}, was considered final {}.", inst, pick_verb(reg, rng), agent, pick_verb(reg, rng), pick(TEMPORAL_HEDGES, rng)),
        5 => match reg {
            NarrativeRegister::Ominous => format!("{} cast out {}. The door closed. It will not reopen.", inst, agent),
            NarrativeRegister::Lyrical => format!("{} was expelled from {}, and the expulsion carried the weight of something that had been decided long before it was announced.", agent, inst),
            NarrativeRegister::Clinical => format!("Involuntary membership termination: {} removed from {}. Grounds: see {} (classified). No appeal filed.", agent, inst, pick_noun(reg, rng)),
            NarrativeRegister::Conspiratorial => format!("{} was expelled from {}, ostensibly for {}. The actual reason, which several members are aware of but none will confirm, involves matters the {} prefers to keep internal.", agent, inst, pick_cause(w, rng), pick_noun(reg, rng)),
            _ => format!("{} was expelled from {}, {}.", agent, inst, event_subordinate_clause(reg, w, rng)),
        },
        6 => format!("{} removed {} from its membership with a speed that suggested the decision had been reached before the {} was convened.", inst, agent, pick_noun(reg, rng)),
        7 => format!("{} was expelled from {} for infractions the {} {} as 'numerous and, in at least one case, architectural.'", agent, inst, pick_noun(reg, rng), pick_verb(reg, rng)),
        8 => format!("The {} of {} voted to expel {}. The vote was unanimous, a fact the {} recorded with evident satisfaction.", pick_noun(reg, rng), inst, agent, pick_noun(reg, rng)),
        _ => format!("{} is no longer welcome in {}. The stated grounds — {} — were accepted by the relevant authorities with a finality that discourages inquiry.", agent, inst, pick_cause(w, rng)),
    }
}

// ===========================================================================
// SITE PROSE — now register-sensitive
// ===========================================================================

/// Generate prose for site entry/exit events, optionally referencing a room purpose.
#[allow(dead_code)]
pub fn generate_site_description(
    event_type: &EventType,
    agent_name: &str,
    site_name: &str,
    rng: &mut StdRng,
    register: NarrativeRegister,
    weirdness: f32,
) -> String {
    generate_site_description_with_room(event_type, agent_name, site_name, None, rng, register, weirdness)
}

/// Generate prose for site entry/exit events with optional room purpose.
pub fn generate_site_description_with_room(
    event_type: &EventType,
    agent_name: &str,
    site_name: &str,
    room_purpose: Option<&str>,
    rng: &mut StdRng,
    register: NarrativeRegister,
    weirdness: f32,
) -> String {
    let base = match event_type {
        EventType::AgentEnteredSite => gen_site_entered(agent_name, site_name, register, weirdness, rng),
        EventType::AgentLeftSite => gen_site_left(agent_name, site_name, register, weirdness, rng),
        _ => format!("{} had dealings with {}. The nature of these dealings was not recorded.", agent_name, site_name),
    };
    // Append room purpose context ~40% of the time when available
    if let Some(purpose) = room_purpose {
        if rng.gen_bool(0.4) {
            let room_clause = room_purpose_clause(purpose, register, rng);
            format!("{} {}", base, room_clause)
        } else {
            base
        }
    } else {
        base
    }
}

fn gen_site_entered(name: &str, site: &str, reg: NarrativeRegister, w: f32, rng: &mut StdRng) -> String {
    match rng.gen_range(0..10) {
        0 => format!("{} entered {}, having filed no advance notice with the relevant authorities. The {} at the threshold was {}.", name, site, pick_noun(reg, rng), pick_verb(reg, rng)),
        1 => format!("{} descended into {}. A {} was opened to document the incursion, though no one expected it to be read.", name, site, pick_noun(reg, rng)),
        2 => format!("{} crossed the boundary of {} with the conviction of someone who has not read the warnings. The warnings were extensive.", name, site),
        3 => format!("The records of {} now include the arrival of {}, who entered citing {}.", site, name, pick_cause(w, rng)),
        4 => format!("{} ventured into {}, a decision the local {} {} as 'inadvisable but not explicitly prohibited.'", name, site, pick_noun(reg, rng), pick_verb(reg, rng)),
        5 => format!("{} was observed entering {}. The observation was {} but not acted upon {}.", name, site, pick_verb(reg, rng), pick(TEMPORAL_HEDGES, rng)),
        6 => format!("{} proceeded into {} despite the condition of the masonry. A {} was {} to mark the occasion.", name, site, pick_noun(reg, rng), pick_verb(reg, rng)),
        7 => match reg {
            NarrativeRegister::Ominous => format!("{} entered {}. Those who enter {} do not always leave. This was known.", name, site, site),
            NarrativeRegister::Lyrical => format!("{} passed into {} the way one passes into sleep — not all at once, but with a gradual surrender of the boundary between here and there.", name, site),
            NarrativeRegister::Clinical => format!("Subject {} entered site {}. Ingress logged. Expected duration: undetermined. {} initiated.", name, site, pick_noun(reg, rng)),
            NarrativeRegister::Conspiratorial => format!("{} entered {}, an act that at least two organizations had been expecting and one had been trying to prevent.", name, site),
            _ => format!("{} entered {}, {}.", name, site, event_subordinate_clause(reg, w, rng)),
        },
        8 => format!("{} stepped into {} with the confidence of someone who believes the architecture is on their side.", name, site),
        _ => format!("The {} of {} was breached by {}, who offered {} as justification. The {} was {} without further inquiry.", pick_noun(reg, rng), site, name, pick_cause(w, rng), pick_noun(reg, rng), pick_verb(reg, rng)),
    }
}

fn gen_site_left(name: &str, site: &str, reg: NarrativeRegister, w: f32, rng: &mut StdRng) -> String {
    match rng.gen_range(0..10) {
        0 => format!("{} emerged from {} bearing an expression the local clerk declined to categorize. The exit {} was completed {}.", name, site, pick_noun(reg, rng), pick(TEMPORAL_HEDGES, rng)),
        1 => format!("{} departed {}, offering no account of what transpired within. The {} was {} accordingly.", name, site, pick_noun(reg, rng), pick_verb(reg, rng)),
        2 => format!("The departure of {} from {} was recorded in the margins of an unrelated {}. No further details were provided.", name, site, pick_noun(reg, rng)),
        3 => format!("{} left {} under circumstances the administration classified as 'concluded.' Whether anything was accomplished remains {}.", name, site, pick(TEMPORAL_HEDGES, rng)),
        4 => format!("{} resurfaced from {} with documentation of uncertain provenance. The {} was {} without enthusiasm.", name, site, pick_noun(reg, rng), pick_verb(reg, rng)),
        5 => format!("{} exited {}, having spent a period within that the records describe as 'of indeterminate purpose.' A {} was filed.", name, site, pick_noun(reg, rng)),
        6 => format!("{} was no longer present in {} as of the latest inspection. The exit was attributed to {}.", name, site, pick_cause(w, rng)),
        7 => match reg {
            NarrativeRegister::Ominous => format!("{} left {}. Whatever they found inside, they brought it with them.", name, site),
            NarrativeRegister::Lyrical => format!("{} returned from {} changed in some way that resists description — not larger or smaller, but differently weighted.", name, site),
            NarrativeRegister::Clinical => format!("Subject {} exited site {}. Condition: intact. Debriefing: not conducted. {} filed.", name, site, pick_noun(reg, rng)),
            NarrativeRegister::Conspiratorial => format!("{} emerged from {} after a duration that does not match the official timetable. The discrepancy has been {} but not explained.", name, site, pick_verb(reg, rng)),
            _ => format!("{} emerged from {}, {}.", name, site, event_subordinate_clause(reg, w, rng)),
        },
        8 => format!("{} exited {} with the unhurried pace of someone who has seen everything inside and found most of it disappointing.", name, site),
        _ => format!("The {} of {} was updated to reflect the departure of {}, {}.", pick_noun(reg, rng), site, name, event_subordinate_clause(reg, w, rng)),
    }
}

// ===========================================================================
// ARTIFACT PROSE — now register-sensitive
// ===========================================================================

/// Generate prose for artifact acquisition/delivery events.
pub fn generate_artifact_event(
    event_type: &EventType,
    agent_name: &str,
    artifact_name: &str,
    location_name: &str,
    rng: &mut StdRng,
    register: NarrativeRegister,
    weirdness: f32,
) -> String {
    match event_type {
        EventType::ArtifactAcquired => gen_artifact_acquired(agent_name, artifact_name, location_name, register, weirdness, rng),
        EventType::ArtifactDelivered => gen_artifact_delivered(agent_name, artifact_name, location_name, register, weirdness, rng),
        _ => format!("{} had dealings involving {} near {}.", agent_name, artifact_name, location_name),
    }
}

fn gen_artifact_acquired(name: &str, art: &str, loc: &str, reg: NarrativeRegister, w: f32, rng: &mut StdRng) -> String {
    match rng.gen_range(0..10) {
        0 => format!("{} recovered {} from {}. A {} was opened to document the acquisition, though the item's provenance remains {}.", name, art, loc, pick_noun(reg, rng), pick(TEMPORAL_HEDGES, rng)),
        1 => format!("{} emerged from {} bearing {}. The relevant authorities were not immediately notified.", name, loc, art),
        2 => format!("The acquisition of {} by {} was accomplished within {}. Whether the previous custodian would have objected is a question the {} declined to entertain.", art, name, loc, pick_noun(reg, rng)),
        3 => format!("{} took possession of {} in {}. The act was {} by no one, as there was no one present to {} it.", name, art, loc, pick_verb(reg, rng), pick_verb(reg, rng)),
        4 => format!("{}, having located {} in the depths of {}, claimed it under the principle of administrative salvage.", name, art, loc),
        5 => format!("{} secured {} from {}. The item's condition was {} as 'consistent with prolonged neglect.'", name, art, loc, pick_verb(reg, rng)),
        6 => format!("From {} came {}, bearing {}. A {} was {} to mark the occasion, though its filing priority remains undetermined.", loc, name, art, pick_noun(reg, rng), pick_verb(reg, rng)),
        7 => match reg {
            NarrativeRegister::Ominous => format!("{} took {} from {}. The {} did not resist. This is not reassuring.", name, art, loc, art),
            NarrativeRegister::Lyrical => format!("{} found {} in {}, or perhaps it is more accurate to say that {} allowed itself to be found, having waited in {} for precisely this purpose.", name, art, loc, art, loc),
            NarrativeRegister::Clinical => format!("Artifact acquisition logged: {} retrieved {} from {}. Condition: see {}. Chain of custody updated.", name, art, loc, pick_noun(reg, rng)),
            NarrativeRegister::Conspiratorial => format!("{} acquired {} from {}, an item that at least one other party had been seeking. The timing of the acquisition is, at minimum, interesting.", name, art, loc),
            _ => format!("{} acquired {} in {}, {}.", name, art, loc, event_subordinate_clause(reg, w, rng)),
        },
        8 => format!("{} claimed {} from within {}. The {} involved was minimal but has been {} for the record.", name, art, loc, pick_noun(reg, rng), pick_verb(reg, rng)),
        _ => format!("{} obtained {} from {}. The transaction — if one can call the removal of an item from a ruin a transaction — was {} by the nearest {}.", name, art, loc, pick_verb(reg, rng), pick_noun(reg, rng)),
    }
}

fn gen_artifact_delivered(name: &str, art: &str, loc: &str, reg: NarrativeRegister, w: f32, rng: &mut StdRng) -> String {
    match rng.gen_range(0..10) {
        0 => format!("{} delivered {} to {}. The receiving clerk {} the item and assigned it a temporary reference number.", name, art, loc, pick_verb(reg, rng)),
        1 => format!("{} arrived in {} carrying {}. Its deposit was {} with a formality that surprised the local office.", name, loc, art, pick_verb(reg, rng)),
        2 => format!("The arrival of {} in {} brought with it {}, which was {} into the local {} under 'miscellaneous acquisitions.'", name, loc, art, pick_verb(reg, rng), pick_noun(reg, rng)),
        3 => format!("{} deposited {} at {}. The {} of custody was completed {}, to the relief of all parties involved.", name, art, loc, pick_noun(reg, rng), pick(TEMPORAL_HEDGES, rng)),
        4 => format!("{} presented {} to the authorities of {}. They accepted it with the enthusiasm of an office receiving {}.", name, art, loc, pick_cause(w, rng)),
        5 => format!("{} was entrusted to the keeping of {} by {}, who {} the transfer and departed without further comment.", art, loc, name, pick_verb(reg, rng)),
        6 => format!("{} relinquished {} in {}. The item was {} to a secure location. The definition of 'secure' was not elaborated upon.", name, art, loc, pick_verb(reg, rng)),
        7 => match reg {
            NarrativeRegister::Ominous => format!("{} brought {} to {}. The {} accepted it. They should not have.", name, art, loc, pick_noun(reg, rng)),
            NarrativeRegister::Lyrical => format!("{} returned {} to {}, and the act of returning it felt less like delivery and more like the completion of a sentence that had been waiting for its final word.", name, art, loc),
            NarrativeRegister::Clinical => format!("Artifact delivery logged: {} deposited {} at {}. Receipt confirmed. {} updated.", name, art, loc, pick_noun(reg, rng)),
            NarrativeRegister::Conspiratorial => format!("{} delivered {} to {}, completing an errand that several parties had been monitoring with interest they would describe as 'academic.'", name, art, loc),
            _ => format!("{} delivered {} to {}, {}.", name, art, loc, event_subordinate_clause(reg, w, rng)),
        },
        8 => format!("The {} of {} received {} from {}, a transfer that the {} {} with its usual thoroughness.", pick_noun(reg, rng), loc, art, name, pick_noun(reg, rng), pick_verb(reg, rng)),
        _ => format!("{} surrendered {} to the custody of {}. The item was {} and stored. The storage location was {} for security purposes.", name, art, loc, pick_verb(reg, rng), pick_verb(reg, rng)),
    }
}

// ===========================================================================
// ADVENTURER DEATH PROSE — now register-sensitive
// ===========================================================================

/// Generate prose for an adventurer dying in a site.
pub fn generate_adventurer_death(
    agent_name: &str,
    site_name: &str,
    rng: &mut StdRng,
    register: NarrativeRegister,
    weirdness: f32,
) -> String {
    match rng.gen_range(0..10) {
        0 => format!("{} was not seen again after entering {}. The {} that was opened on their behalf was subsequently closed, unfiled.", agent_name, site_name, pick_noun(register, rng)),
        1 => format!("The expedition of {} into {} concluded in a manner the records describe only as 'terminal.' A {} was {} {}.", agent_name, site_name, pick_noun(register, rng), pick_verb(register, rng), pick(TEMPORAL_HEDGES, rng)),
        2 => format!("{} perished within {}, a development attributed to {}. The body has not been recovered, and the relevant paperwork remains incomplete.", agent_name, site_name, pick_cause(weirdness, rng)),
        3 => format!("The census entry for {} was amended following their final visit to {}. The amendment was terse.", agent_name, site_name),
        4 => format!("{} met their end in the depths of {}. The cause of death was described by the coroner as 'architecturally mediated.'", agent_name, site_name),
        5 => format!("{} entered {} and did not emerge. The {} recorded this outcome under 'anticipated losses.'", agent_name, site_name, pick_noun(register, rng)),
        6 => format!("The file on {} was closed following an incident in {} that the investigating clerk described as 'unambiguously fatal.'", agent_name, site_name),
        7 => match register {
            NarrativeRegister::Ominous => format!("{} died in {}. The {} had been expecting this. So had {}.", agent_name, site_name, pick_noun(register, rng), site_name),
            NarrativeRegister::Lyrical => format!("{} came to rest in {}, which is a polite way of saying what happened, and politeness is all one can offer the dead.", agent_name, site_name),
            NarrativeRegister::Clinical => format!("Mortality event in {}: subject {}. Cause: environmental factors consistent with site hazard profile. {} closed.", site_name, agent_name, pick_noun(register, rng)),
            NarrativeRegister::Conspiratorial => format!("{} died in {}, according to the official account. Certain details of the account are, however, inconsistent with the condition of the remains.", agent_name, site_name),
            _ => format!("{} succumbed to conditions within {} that had been {} in an earlier {} but never addressed.", agent_name, site_name, pick_verb(register, rng), pick_noun(register, rng)),
        },
        8 => format!("{} was {} from the census following events in {} that the {} classified as 'irreversible.'", agent_name, pick_verb(register, rng), site_name, pick_noun(register, rng)),
        _ => format!("{} perished in {}, {}. The {} was filed without delay, which is more than can be said for most of the office's work.", agent_name, site_name, event_subordinate_clause(register, weirdness, rng), pick_noun(register, rng)),
    }
}

// ===========================================================================
// Utility
// ===========================================================================

/// Find the nearest settlement name for a given position.
pub fn nearest_settlement_name(x: u32, y: u32, world: &World) -> String {
    let mut best_name = "the uncategorized hinterlands".to_string();
    let mut best_dist = u32::MAX;

    for s in &world.settlements {
        let dx = (s.x as i32 - x as i32).unsigned_abs();
        let dy = (s.y as i32 - y as i32).unsigned_abs();
        let dist = dx + dy;
        if dist < best_dist {
            best_dist = dist;
            best_name = s.name.clone();
        }
    }

    best_name
}

// ===========================================================================
// ROOM PURPOSE PROSE
// ===========================================================================

/// Generate a clause referencing a room's purpose.
fn room_purpose_clause(purpose: &str, reg: NarrativeRegister, rng: &mut StdRng) -> String {
    match purpose {
        "Storage" => match rng.gen_range(0..4) {
            0 => format!("The room in question served as storage — crates stacked with the {} of an office that has forgotten what it stored.", pick_noun(reg, rng)),
            1 => "The chamber had been designated for storage, a purpose it fulfilled with the mute patience of furniture.".to_string(),
            2 => "Shelves lined the walls, bearing objects whose inventory tags had outlived their legibility.".to_string(),
            _ => format!("The storage chamber contained items the {} described only as 'miscellaneous,' a category broad enough to include everything.", pick_noun(reg, rng)),
        },
        "Ritual" => match rng.gen_range(0..4) {
            0 => "The chamber bore the unmistakable markings of ritual use — stains that formed patterns no cleaning could fully address.".to_string(),
            1 => format!("The ritual chamber's purpose was evident from the arrangement of objects the {} had declined to catalogue.", pick_noun(reg, rng)),
            2 => "The room's ceremonial purpose was attested to by inscriptions that contradicted each other with impressive thoroughness.".to_string(),
            _ => "A ritual chamber, the rites of which had been discontinued but whose atmosphere had not yet received the memo.".to_string(),
        },
        "Administrative" => match rng.gen_range(0..4) {
            0 => format!("The administrative office still contained desks arranged for a {} that would never convene.", pick_noun(reg, rng)),
            1 => "The room was unmistakably administrative — the air itself tasted faintly of old paper and institutional regret.".to_string(),
            2 => format!("An administrative chamber, its filing cabinets {} with a finality that suggested the files had won.", pick_verb(reg, rng)),
            _ => "The office retained the organized desolation of a workspace whose purpose had concluded but whose furniture had not been informed.".to_string(),
        },
        "Habitation" => match rng.gen_range(0..4) {
            0 => "The room showed signs of habitation — or rather, signs that habitation had once occurred and then thought better of it.".to_string(),
            1 => "A residential chamber whose last occupant had departed with more haste than tidiness.".to_string(),
            2 => "The quarters were arranged for comfort of a kind that no longer applied to anyone present.".to_string(),
            _ => format!("The living quarters were {} by the {} as 'formerly occupied,' a designation that raised no questions because no one was present to ask them.", pick_verb(reg, rng), pick_noun(reg, rng)),
        },
        "Trophy" => match rng.gen_range(0..4) {
            0 => "The trophy room displayed achievements that the current occupants could neither verify nor explain.".to_string(),
            1 => "A chamber of trophies, each commemorating a victory whose nature the accompanying plaques had been carefully vague about.".to_string(),
            2 => format!("The trophy hall's displays were {} by a {} that had not been updated since the last era of coherent record-keeping.", pick_verb(reg, rng), pick_noun(reg, rng)),
            _ => "The trophies lining the walls represented accomplishments that ranged from the military to the taxonomic, with several that defied either category.".to_string(),
        },
        "Disputed" => match rng.gen_range(0..4) {
            0 => "The room's purpose was itself the subject of an ongoing dispute between parties who had never occupied it.".to_string(),
            1 => format!("A chamber whose designation was the subject of a {} that had outlasted the room's structural integrity.", pick_noun(reg, rng)),
            2 => "The room's function was disputed — three competing plaques on the door offered contradictory explanations with equal confidence.".to_string(),
            _ => "A disputed chamber, claimed simultaneously as a meeting room, a reliquary, and a broom closet by factions who had never visited.".to_string(),
        },
        _ => String::new(),
    }
}

// ===========================================================================
// INHABITANT INTERACTION PROSE
// ===========================================================================

/// Generate prose for an interaction between an adventurer and a site inhabitant.
pub fn generate_inhabitant_interaction(
    agent_name: &str,
    inhabitant_name: &str,
    _inhabitant_desc: &str,
    site_name: &str,
    room_purpose: Option<&str>,
    rng: &mut StdRng,
    register: NarrativeRegister,
    weirdness: f32,
) -> String {
    let outcome = rng.gen_range(0..4); // 0=ignored, 1=questioned, 2=assisted, 3=driven out
    let base = match outcome {
        0 => gen_inhabitant_ignored(agent_name, inhabitant_name, site_name, register, weirdness, rng),
        1 => gen_inhabitant_questioned(agent_name, inhabitant_name, site_name, register, weirdness, rng),
        2 => gen_inhabitant_assisted(agent_name, inhabitant_name, site_name, register, weirdness, rng),
        _ => gen_inhabitant_drove_out(agent_name, inhabitant_name, site_name, register, weirdness, rng),
    };
    // Optionally append room context
    if let Some(purpose) = room_purpose {
        if rng.gen_bool(0.35) {
            let clause = room_purpose_clause(purpose, register, rng);
            format!("{} {}", base, clause)
        } else {
            base
        }
    } else {
        base
    }
}

fn gen_inhabitant_ignored(name: &str, inhab: &str, site: &str, reg: NarrativeRegister, _w: f32, rng: &mut StdRng) -> String {
    match rng.gen_range(0..6) {
        0 => format!("{} encountered {} within {}. Neither party acknowledged the other, which appeared to be the preferred protocol.", name, inhab, site),
        1 => format!("{} passed {} in the corridors of {} without incident. The {} continued whatever it was doing, which was not immediately apparent.", name, inhab, site, inhab),
        2 => format!("{} and {} occupied the same chamber in {} for a period the {} would later describe as 'uneventful,' a word doing considerable work.", name, inhab, site, pick_noun(reg, rng)),
        3 => format!("{} was present when {} entered the room, but gave no sign of having noticed. This may have been intentional. It may also have been something else.", inhab, name),
        4 => format!("In {}, {} encountered {}, who declined all forms of interaction with a thoroughness that bordered on artistry.", site, name, inhab),
        _ => format!("{} moved through {} where {} resided. The resident's indifference was comprehensive and, one suspects, practiced.", name, site, inhab),
    }
}

fn gen_inhabitant_questioned(name: &str, inhab: &str, site: &str, reg: NarrativeRegister, w: f32, rng: &mut StdRng) -> String {
    match rng.gen_range(0..6) {
        0 => format!("{} was questioned by {} upon entering {}. The questions concerned matters of {} that {} was not equipped to answer.", name, inhab, site, pick_cause(w, rng), name),
        1 => format!("{} demanded credentials from {} within {}. The credentials produced were {} by the occupant and returned without comment.", inhab, name, site, pick_verb(reg, rng)),
        2 => format!("{} interrogated {} regarding their business in {}. The business, once explained, was {} by the occupant as 'administratively improbable.'", inhab, name, site, pick_verb(reg, rng)),
        3 => format!("Upon encountering {} in {}, {} asked a series of questions whose answers were apparently unsatisfactory, as they were asked again.", name, site, inhab),
        4 => format!("{} was challenged by {} in {}. The challenge was procedural in nature and concerned documentation that {} did not possess.", name, inhab, site, name),
        _ => format!("{} met {} in the lower reaches of {}. A brief but intense exchange of questions followed, none of which were answered to anyone's satisfaction.", name, inhab, site),
    }
}

fn gen_inhabitant_assisted(name: &str, inhab: &str, site: &str, reg: NarrativeRegister, _w: f32, rng: &mut StdRng) -> String {
    match rng.gen_range(0..6) {
        0 => format!("{} provided {} with directions through {} that were, against all expectation, accurate.", inhab, name, site),
        1 => format!("{} received unexpected assistance from {} within {}. The assistance consisted of information the {} would later classify as 'technically useful.'", name, inhab, site, pick_noun(reg, rng)),
        2 => format!("{} guided {} through a section of {} that the maps had neglected to include. Whether this constituted helpfulness or something more ambiguous remained unclear.", inhab, name, site),
        3 => format!("{} offered {} safe passage through their chamber in {}. The passage was indeed safe. The chamber was another matter.", inhab, name, site),
        4 => format!("Within {}, {} indicated to {} an exit that the architecture had attempted to conceal. The motivation for this assistance was not explained.", site, inhab, name),
        _ => format!("{} shared provisions with {} in {}, an act of generosity the {} {} with evident surprise.", inhab, name, site, pick_noun(reg, rng), pick_verb(reg, rng)),
    }
}

fn gen_inhabitant_drove_out(name: &str, inhab: &str, site: &str, reg: NarrativeRegister, w: f32, rng: &mut StdRng) -> String {
    match rng.gen_range(0..6) {
        0 => format!("{} was driven from a section of {} by {}, who objected to the intrusion on grounds the {} classified as {}.", name, site, inhab, pick_noun(reg, rng), pick_cause(w, rng)),
        1 => format!("{} made it clear — through means that were not entirely verbal — that {} was not welcome in this part of {}.", inhab, name, site),
        2 => format!("{} retreated from an encounter with {} in {}, a tactical decision the {} would later {} as 'prudent.'", name, inhab, site, pick_noun(reg, rng), pick_verb(reg, rng)),
        3 => format!("{} expressed territorial displeasure at the presence of {} in {}. The expression was persuasive.", inhab, name, site),
        4 => format!("The deeper corridors of {} proved inhospitable, largely due to the efforts of {}, who regarded {} with what the {} described as 'active disapproval.'", site, inhab, name, pick_noun(reg, rng)),
        _ => format!("{} was expelled from a chamber of {} by {}, who had been there longer and intended to remain longer still.", name, site, inhab),
    }
}

// ===========================================================================
// FACTION DISBANDING PROSE
// ===========================================================================

/// Generate prose for a faction being disbanded due to zero members and low power.
pub fn generate_faction_disbanded(
    faction_name: &str,
    rng: &mut StdRng,
    register: NarrativeRegister,
    weirdness: f32,
) -> String {
    match rng.gen_range(0..8) {
        0 => format!("The {} was formally dissolved, its membership having preceded it into nonexistence. The {} was {} without ceremony.", faction_name, pick_noun(register, rng), pick_verb(register, rng)),
        1 => format!("The dissolution of the {} was recorded in the ledger with the particular efficiency reserved for things that have already happened. No objections were raised, as there was no one remaining to raise them.", faction_name),
        2 => format!("The {} ceased to exist, a development the administrative record {} with characteristic indifference. Its charter was archived under 'concluded entities.'", faction_name, pick_verb(register, rng)),
        3 => format!("The last trace of the {} was a filing cabinet that no one claimed. The {} was closed {}.", faction_name, pick_noun(register, rng), pick(TEMPORAL_HEDGES, rng)),
        4 => match register {
            NarrativeRegister::Ominous => format!("The {} is gone. It will not be missed, because there is no one left to miss it.", faction_name),
            NarrativeRegister::Lyrical => format!("The {} dissolved the way institutions dissolve — not with a declaration, but with a silence that no one noticed was silence until it had been going on for some time.", faction_name),
            NarrativeRegister::Clinical => format!("Entity dissolution: {}. Contributing factors: membership depletion, resource exhaustion. {} closed.", faction_name, pick_noun(register, rng)),
            NarrativeRegister::Conspiratorial => format!("The {} was officially disbanded, though certain {} suggest it may continue to exist in a form its former members would not recognize.", faction_name, pick_noun(register, rng)),
            _ => format!("The {} was struck from the register of active institutions, {}.", faction_name, event_subordinate_clause(register, weirdness, rng)),
        },
        5 => format!("The {} was disbanded after a final audit revealed zero members, zero assets, and a filing backlog that exceeded the institution's entire operational history.", faction_name),
        6 => format!("The {} concluded its existence with the quiet finality of a door closing in an empty building. The {} {} the closure and moved on to more pressing vacancies.", faction_name, pick_noun(register, rng), pick_verb(register, rng)),
        _ => format!("The administrative record of the {} was transferred to the archives, where it will join other concluded entities in a silence the archivist describes as 'comprehensive.'", faction_name),
    }
}

// ===========================================================================
// SEASONAL TRANSITION PROSE
// ===========================================================================

/// Generate a log entry for the transition to a new season.
pub fn generate_seasonal_transition(
    season: crate::sim::world::Season,
    rng: &mut StdRng,
    register: NarrativeRegister,
    _weirdness: f32,
) -> String {
    use crate::sim::world::Season;
    match season {
        Season::Spring => match rng.gen_range(0..5) {
            0 => format!("The Bureau of Meteorological Affairs {} the onset of Spring. The ledgers have been updated accordingly.", pick_verb(register, rng)),
            1 => "Spring has been declared. The thaw proceeds on schedule, pending the usual approvals.".to_string(),
            2 => format!("The season has turned to Spring. Several {} were filed regarding the brightness of the light.", pick_noun(register, rng)),
            3 => "The registrar noted the arrival of Spring with a fresh requisition for ink, the previous supply having been depleted by the demands of winter correspondence.".to_string(),
            _ => "Spring. The earth softens. The administrative calendar advances. Both events are treated with equal formality.".to_string(),
        },
        Season::Summer => match rng.gen_range(0..5) {
            0 => format!("Summer has been {} by the relevant authorities. Heat advisories have been posted in the customary locations.", pick_verb(register, rng)),
            1 => "The longest days have arrived. The Bureau of Seasonal Compliance reports no irregularities, which is itself irregular.".to_string(),
            2 => format!("Summer commences. The {} indicates conditions favorable to expeditions and unfavorable to concentrated thought.", pick_noun(register, rng)),
            3 => "The warm season has begun. Several officials have relocated their desks nearer to windows, a development the facilities committee is monitoring.".to_string(),
            _ => "Summer. The heat settles over the land like a memorandum that no one has read but everyone has acknowledged.".to_string(),
        },
        Season::Autumn => match rng.gen_range(0..5) {
            0 => format!("Autumn has been {} in the official record. The leaves have been notified.", pick_verb(register, rng)),
            1 => "The season turns to Autumn. The harvest assessors have been dispatched. Several have already filed preliminary complaints about the quality of the roads.".to_string(),
            2 => format!("Autumn arrives, bringing with it the annual surge in institutional activity and a {} of unusual length.", pick_noun(register, rng)),
            3 => "The Bureau of Meteorological Affairs acknowledges Autumn. The days shorten. The paperwork does not.".to_string(),
            _ => "Autumn. The world contracts. Institutions expand to fill the reduced daylight with twice the deliberation.".to_string(),
        },
        Season::Winter => match rng.gen_range(0..5) {
            0 => format!("Winter has been formally {} by the meteorological office. Travel advisories are in effect.", pick_verb(register, rng)),
            1 => "The cold season descends. The registrar has requisitioned additional firewood and noted, with characteristic precision, that the previous winter also occurred.".to_string(),
            2 => format!("Winter. The {} has been updated to reflect conditions of reduced visibility and increased administrative contemplation.", pick_noun(register, rng)),
            3 => "Winter arrives. Movement across the territory slows to a pace the bureaucracy finds companionable.".to_string(),
            _ => "The season turns to Winter. The frost settles with the quiet authority of a regulation that has always existed.".to_string(),
        },
    }
}

/// Generate prose for a relationship event (formation or change).
pub fn generate_relationship_event(
    agent_name: &str,
    other_name: &str,
    kind: &str,
    is_formation: bool,
    rng: &mut StdRng,
    register: NarrativeRegister,
    _weirdness: f32,
) -> String {
    let verb = pick_verb(register, rng);
    if is_formation {
        match kind {
            "Friend" => match rng.gen_range(0..4) {
                0 => format!("{} and {} have developed what the registry classifies as a mutual regard.", agent_name, other_name),
                1 => format!("A friendship has been {} between {} and {}. The paperwork is pending.", verb, agent_name, other_name),
                2 => format!("{} and {} have been observed in each other's company with increasing regularity. The registrar has made a note.", agent_name, other_name),
                _ => format!("{} has formed an association with {} that the office has tentatively categorized as amicable.", agent_name, other_name),
            },
            "Rival" => match rng.gen_range(0..4) {
                0 => format!("{} and {} have entered into a state of mutual professional antagonism.", agent_name, other_name),
                1 => format!("A rivalry has been {} between {} and {}. Both parties deny it with equal conviction.", verb, agent_name, other_name),
                2 => format!("{} has developed a pointed disagreement with {} that shows no sign of administrative resolution.", agent_name, other_name),
                _ => format!("The records now reflect a formal rivalry between {} and {}. The causes are disputed.", agent_name, other_name),
            },
            "Partner" => match rng.gen_range(0..4) {
                0 => format!("{} and {} have entered into a partnership of a personal nature. The registrar has updated the relevant forms.", agent_name, other_name),
                1 => format!("A romantic attachment has been {} between {} and {}. The bureau offers no comment.", verb, agent_name, other_name),
                2 => format!("{} and {} have formed a domestic arrangement that the census office has acknowledged with characteristic brevity.", agent_name, other_name),
                _ => format!("{} has taken {} as a partner. The filing was completed without incident.", agent_name, other_name),
            },
            "Mentor" => match rng.gen_range(0..3) {
                0 => format!("{} has taken {} under advisement in matters of institutional significance.", agent_name, other_name),
                1 => format!("A mentorship has been {} between {} and {}, the former instructing the latter in the customs of the office.", verb, agent_name, other_name),
                _ => format!("{} has assumed the role of mentor to {}. The arrangement appears to be functional.", agent_name, other_name),
            },
            "Estranged" => match rng.gen_range(0..3) {
                0 => format!("The relationship between {} and {} has been {} as estranged. Both parties maintain separate filing systems.", agent_name, other_name, verb),
                1 => format!("{} and {} have entered a period of mutual avoidance that the registrar has classified as indefinite.", agent_name, other_name),
                _ => format!("The association between {} and {} has soured. The relevant documents have been archived.", agent_name, other_name),
            },
            _ => format!("{} and {} have formed a relationship of uncertain classification.", agent_name, other_name),
        }
    } else {
        // Relationship changed
        match kind {
            "Estranged" => match rng.gen_range(0..3) {
                0 => format!("The bond between {} and {} has deteriorated beyond administrative repair.", agent_name, other_name),
                1 => format!("{} and {} are now estranged. The records have been updated accordingly.", agent_name, other_name),
                _ => format!("What was once a functional association between {} and {} has been reclassified as estranged.", agent_name, other_name),
            },
            "Friend" => format!("{} and {} have reconciled. The registrar has re-opened the relevant file.", agent_name, other_name),
            _ => format!("The relationship between {} and {} has been reclassified. The paperwork is ongoing.", agent_name, other_name),
        }
    }
}
