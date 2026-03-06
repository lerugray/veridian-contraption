use rand::rngs::StdRng;
use rand::Rng;

use crate::sim::event::EventType;
use crate::sim::world::World;

// ---------------------------------------------------------------------------
// Shared word lists — bureaucratic flavor
// ---------------------------------------------------------------------------

const PROCEDURAL_VERBS: &[&str] = &[
    "filed", "disputed", "remanded", "indicated", "noted",
    "acknowledged", "declined to record", "formally objected to",
    "referred to committee", "tabled", "struck from the agenda",
    "appended to the existing dossier",
];

const BUREAUCRATIC_NOUNS: &[&str] = &[
    "inquiry", "filing", "counter-filing", "tribunal",
    "committee", "subcommittee", "provisional assessment", "formal notation",
    "memorandum", "register", "ledger entry", "supplementary docket",
    "notice of intent", "petition", "procedural review",
];

const TEMPORAL_HEDGES: &[&str] = &[
    "in due course", "pending review", "subject to revision",
    "contingent upon further inquiry", "without prejudice",
    "at some future date to be determined", "upon completion of the relevant paperwork",
    "when circumstances permit",
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
];

fn pick<'a>(options: &'a [&'a str], rng: &mut StdRng) -> &'a str {
    options[rng.gen_range(0..options.len())]
}

/// Generate a subordinate clause to embed in a longer sentence.
fn subordinate_clause(name: &str, loc: &str, rng: &mut StdRng) -> String {
    match rng.gen_range(0..12) {
        0 => format!("whose previous {} remained unresolved", pick(BUREAUCRATIC_NOUNS, rng)),
        1 => format!("about whom the {} of {} had {} concerns", pick(BUREAUCRATIC_NOUNS, rng), loc, pick(PROCEDURAL_VERBS, rng)),
        2 => format!("whose standing in {} was a matter of some administrative ambiguity", loc),
        3 => format!("who had been the subject of {} prior to the event in question", pick(BUREAUCRATIC_NOUNS, rng)),
        4 => format!("whose documentation the office of {} had {} on three prior occasions", loc, pick(PROCEDURAL_VERBS, rng)),
        5 => format!("against whom {} had been {} but never resolved", pick(BUREAUCRATIC_NOUNS, rng), pick(PROCEDURAL_VERBS, rng)),
        6 => format!("whose file contained a marginal annotation reading 'see also: {}'", pick(ABSURDIST_CAUSES, rng)),
        7 => format!("whom the records of {} listed under two distinct and contradictory entries", loc),
        8 => format!("whose relationship to the matter was described as 'provisional'"),
        9 => format!("who had been {} by the relevant authorities {}", pick(PROCEDURAL_VERBS, rng), pick(TEMPORAL_HEDGES, rng)),
        10 => format!("about whom {} had been filed regarding {}", pick(BUREAUCRATIC_NOUNS, rng), pick(ABSURDIST_CAUSES, rng)),
        _ => format!("whose presence in {} the {} had not yet formally acknowledged", loc, pick(BUREAUCRATIC_NOUNS, rng)),
    }
}

/// Generate the prose description for an event.
pub fn generate_description(
    event_type: &EventType,
    agent_name: Option<&str>,
    location_name: Option<&str>,
    _tick: u64,
    rng: &mut StdRng,
) -> String {
    let loc = location_name.unwrap_or("an unregistered locality");
    let name = agent_name.unwrap_or("an unnamed party");

    match event_type {
        EventType::AgentDied => {
            let sub = subordinate_clause(name, loc, rng);
            match rng.gen_range(0..7) {
                0 => format!("{}, {}, ceased to be present in {}. No formal {} was opened.", name, sub, loc, pick(BUREAUCRATIC_NOUNS, rng)),
                1 => format!("{}, formerly in good standing, was removed from the census of {}. A clerk {} the absence in the margin of an unrelated ledger.", name, loc, pick(PROCEDURAL_VERBS, rng)),
                2 => format!("The {} of {} confirmed that {}, {}, is no longer extant. The relevant paperwork was completed {}.", pick(BUREAUCRATIC_NOUNS, rng), loc, name, sub, pick(TEMPORAL_HEDGES, rng)),
                3 => format!("{} of {} expired, or was otherwise rendered absent. The vacancy has not yet been filled. The matter was {} under 'resolved by circumstance.'", name, loc, pick(PROCEDURAL_VERBS, rng)),
                4 => format!("{}, whose debts were subsequently forgiven, ceased to occupy their census entry in {}. This was {} and promptly misfiled.", name, loc, pick(PROCEDURAL_VERBS, rng)),
                5 => format!("The continued existence of {} in {} was downgraded from 'confirmed' to 'discontinued.' The relevant authorities were not notified {}.", name, loc, pick(TEMPORAL_HEDGES, rng)),
                _ => format!("{}, {}, was struck from the register of {}. The cause was attributed to {}.", name, sub, loc, pick(ABSURDIST_CAUSES, rng)),
            }
        }

        EventType::AgentArrived => {
            let sub = subordinate_clause(name, loc, rng);
            match rng.gen_range(0..7) {
                0 => format!("{} arrived at {} without prior notice or evident purpose. The local {} {} the arrival {}.", name, loc, pick(BUREAUCRATIC_NOUNS, rng), pick(PROCEDURAL_VERBS, rng), pick(TEMPORAL_HEDGES, rng)),
                1 => format!("{}, {}, appeared in {} bearing documentation of uncertain validity.", name, sub, loc),
                2 => format!("The {} of {} recorded the arrival of {}, though the arrival itself was {} on procedural grounds.", pick(BUREAUCRATIC_NOUNS, rng), loc, name, pick(PROCEDURAL_VERBS, rng)),
                3 => format!("{} entered {} with the air of someone who has been expected elsewhere. A {} was opened {}.", name, loc, pick(BUREAUCRATIC_NOUNS, rng), pick(TEMPORAL_HEDGES, rng)),
                4 => format!("{}, {}, was provisionally noted in the register of {}. The notation included several caveats.", name, sub, loc),
                5 => format!("{} arrived in {} claiming business with no one in particular. This claim was {} by the local office.", name, loc, pick(PROCEDURAL_VERBS, rng)),
                _ => format!("The presence of {} was detected in {} by means of {}. A {} was {} accordingly.", name, loc, pick(ABSURDIST_CAUSES, rng), pick(BUREAUCRATIC_NOUNS, rng), pick(PROCEDURAL_VERBS, rng)),
            }
        }

        EventType::AgentDeparted => {
            let sub = subordinate_clause(name, loc, rng);
            match rng.gen_range(0..7) {
                0 => format!("{} departed from {} citing personal obligations of an unspecified nature. The {} was {} accordingly.", name, loc, pick(BUREAUCRATIC_NOUNS, rng), pick(PROCEDURAL_VERBS, rng)),
                1 => format!("{}, {}, left {} without filing the customary notice of departure.", name, sub, loc),
                2 => format!("The departure of {} from {} was {} by the local office. Several unsigned documents were left behind.", name, loc, pick(PROCEDURAL_VERBS, rng)),
                3 => format!("{} concluded business in {} that no record describes and departed {}.", name, loc, pick(TEMPORAL_HEDGES, rng)),
                4 => format!("{}, {}, vacated {} under circumstances the local clerk declined to elaborate upon.", name, sub, loc),
                5 => format!("The register of {} was updated to reflect the absence of {}. The update was attributed to {}.", loc, name, pick(ABSURDIST_CAUSES, rng)),
                _ => format!("{} was no longer present in {} as of the most recent {}, a fact the office {} without comment.", name, loc, pick(BUREAUCRATIC_NOUNS, rng), pick(PROCEDURAL_VERBS, rng)),
            }
        }

        EventType::SettlementGrew => {
            match rng.gen_range(0..6) {
                0 => format!("The settlement of {} recorded an increase in its registered population. The {} was {} with reluctant precision.", loc, pick(BUREAUCRATIC_NOUNS, rng), pick(PROCEDURAL_VERBS, rng)),
                1 => format!("The population of {} expanded by a number the census office described as 'within acceptable parameters.' The growth was attributed to {}.", loc, pick(ABSURDIST_CAUSES, rng)),
                2 => format!("Additional residents were assigned provisional status in {}. A clerk expressed cautious optimism, then {} the statement.", loc, pick(PROCEDURAL_VERBS, rng)),
                3 => format!("{} experienced demographic expansion. The housing {} was updated {}.", loc, pick(BUREAUCRATIC_NOUNS, rng), pick(TEMPORAL_HEDGES, rng)),
                4 => format!("The population figures of {} were revised upward. This revision was {} to factors the administration declined to specify.", loc, pick(PROCEDURAL_VERBS, rng)),
                _ => format!("New arrivals in {} prompted the opening of a supplementary {}, to be reviewed {}.", loc, pick(BUREAUCRATIC_NOUNS, rng), pick(TEMPORAL_HEDGES, rng)),
            }
        }

        EventType::SettlementShrank => {
            match rng.gen_range(0..6) {
                0 => format!("The population of {} experienced a documented reduction. The decrease was attributed to {}.", loc, pick(ABSURDIST_CAUSES, rng)),
                1 => format!("Several addresses in {} were reclassified as 'potentially occupied.' The {} office {} the discrepancy but offered no correction.", loc, pick(BUREAUCRATIC_NOUNS, rng), pick(PROCEDURAL_VERBS, rng)),
                2 => format!("A minor official in {} suggested the population figures may have been previously inflated. The shortfall was absorbed into the next quarter's projections.", loc),
                3 => format!("The demographic {} of {} showed contraction. This was {} without ceremony.", pick(BUREAUCRATIC_NOUNS, rng), loc, pick(PROCEDURAL_VERBS, rng)),
                4 => format!("{} recorded fewer inhabitants than its {} accounted for. The discrepancy remains under review {}.", loc, pick(BUREAUCRATIC_NOUNS, rng), pick(TEMPORAL_HEDGES, rng)),
                _ => format!("The census of {} was revised downward, a correction the office described as 'overdue.' The underlying cause — {} — was not addressed.", loc, pick(ABSURDIST_CAUSES, rng)),
            }
        }

        EventType::WeatherEvent => {
            match rng.gen_range(0..6) {
                0 => format!("Conditions in the vicinity of {} became unseasonably damp. This was attributed to {}, which the Bureau of Ambient Conditions is still reviewing.", loc, pick(ABSURDIST_CAUSES, rng)),
                1 => format!("The weather near {} was {} by the meteorological office as 'within parameters,' though several residents {} the characterization.", loc, pick(PROCEDURAL_VERBS, rng), pick(PROCEDURAL_VERBS, rng)),
                2 => format!("An amber haze settled over {}. The phenomenon was attributed to {} and logged under the existing {} for atmospheric irregularities.", loc, pick(ABSURDIST_CAUSES, rng), pick(BUREAUCRATIC_NOUNS, rng)),
                3 => format!("{} experienced conditions that one official termed 'the usual arrangement.' A {} was opened {}, though expectations for its conclusion are modest.", loc, pick(BUREAUCRATIC_NOUNS, rng), pick(TEMPORAL_HEDGES, rng)),
                4 => format!("A persistent low wind in the vicinity of {} prompted the filing of a {} with the regional office. The {} was {} but not acted upon.", loc, pick(BUREAUCRATIC_NOUNS, rng), pick(BUREAUCRATIC_NOUNS, rng), pick(PROCEDURAL_VERBS, rng)),
                _ => format!("The area surrounding {} was punctuated by brief intervals of something not quite rain. Prevailing atmospheric indifference was {} as the cause.", loc, pick(PROCEDURAL_VERBS, rng)),
            }
        }

        EventType::AgeEvent => {
            let sub = subordinate_clause(name, loc, rng);
            match rng.gen_range(0..6) {
                0 => format!("{} of {} has persisted in the world for a notable duration. The actuarial tables regard this with skepticism.", name, loc),
                1 => format!("{}, {}, continues to occupy their census entry with considerable tenacity.", name, sub),
                2 => format!("The longevity of {} has become a matter of minor administrative interest in {}. A {} was {} to document the fact.", name, loc, pick(BUREAUCRATIC_NOUNS, rng), pick(PROCEDURAL_VERBS, rng)),
                3 => format!("{} of {} has survived long enough to require the opening of a supplementary {} for their records.", name, loc, pick(BUREAUCRATIC_NOUNS, rng)),
                4 => format!("{}, {}, has reached an age that the {} of {} considers 'statistically noteworthy.'", name, sub, pick(BUREAUCRATIC_NOUNS, rng), loc),
                _ => format!("The continued existence of {} in {} was {} by the census office, which amended their file {}.", name, loc, pick(PROCEDURAL_VERBS, rng), pick(TEMPORAL_HEDGES, rng)),
            }
        }

        EventType::CensusReport => {
            format!("A census was conducted. The results were filed.")
        }

        EventType::WorldGenesis => {
            format!("The world stirs into being. Somewhere, a ledger is opened.")
        }

        EventType::AgentBorn => {
            let sub = subordinate_clause(name, loc, rng);
            match rng.gen_range(0..6) {
                0 => format!("{} entered the records of {} under circumstances the registrar described as 'standard.' A provisional identity number was assigned {}.", name, loc, pick(TEMPORAL_HEDGES, rng)),
                1 => format!("{}, {}, was added to the census of {} to the apparent surprise of the local office.", name, sub, loc),
                2 => format!("The {} of {} {} the existence of {} amid paperwork that had already been prepared.", pick(BUREAUCRATIC_NOUNS, rng), loc, pick(PROCEDURAL_VERBS, rng), name),
                3 => format!("{} materialized in the records of {} without the customary advance notification to the Bureau of New Arrivals. A {} was opened {}.", name, loc, pick(BUREAUCRATIC_NOUNS, rng), pick(TEMPORAL_HEDGES, rng)),
                4 => format!("A new entry for {} was {} in the register of {}, attributed to {}.", name, pick(PROCEDURAL_VERBS, rng), loc, pick(ABSURDIST_CAUSES, rng)),
                _ => format!("{} was assigned to the census of {}. The relevant {} was completed with a speed that alarmed the processing clerk.", name, loc, pick(BUREAUCRATIC_NOUNS, rng)),
            }
        }

        // Institutional events use generate_institutional_description
        EventType::InstitutionFounded
        | EventType::InstitutionDissolved
        | EventType::SchismOccurred
        | EventType::DoctrineShifted
        | EventType::AllianceFormed
        | EventType::AllianceStrained
        | EventType::RivalryDeclared
        | EventType::MemberJoined
        | EventType::MemberDeparted
        | EventType::MemberExpelled => {
            format!("An institutional event occurred near {}.", loc)
        }
    }
}

/// Generate prose for institutional events.
pub fn generate_institutional_description(
    event_type: &EventType,
    agent_name: Option<&str>,
    inst_name: Option<&str>,
    other_name: Option<&str>,
    rng: &mut StdRng,
) -> String {
    let inst = inst_name.unwrap_or("an unnamed body");
    let agent = agent_name.unwrap_or("a party of uncertain identity");
    let other = other_name.unwrap_or("another organization");

    match event_type {
        EventType::InstitutionFounded => {
            match rng.gen_range(0..5) {
                0 => format!("{} has been formally established near {}, by the initiative of {}. Its charter has been {} and a {} opened.", inst, other, agent, pick(PROCEDURAL_VERBS, rng), pick(BUREAUCRATIC_NOUNS, rng)),
                1 => format!("A new organization, {}, was founded by {} in the vicinity of {}. The relevant authorities were notified {}.", inst, agent, other, pick(TEMPORAL_HEDGES, rng)),
                2 => format!("{} brought {} into existence near {}. The necessary paperwork was completed with a thoroughness that surprised the filing office.", agent, inst, other),
                3 => format!("The founding of {} was {} near {} by {}, who cited {} as the motivating factor.", inst, pick(PROCEDURAL_VERBS, rng), other, agent, pick(ABSURDIST_CAUSES, rng)),
                _ => format!("{} established {} in {}. A {} was immediately opened to govern its activities, though its scope remains {}.", agent, inst, other, pick(BUREAUCRATIC_NOUNS, rng), pick(TEMPORAL_HEDGES, rng)),
            }
        }
        EventType::InstitutionDissolved => {
            match rng.gen_range(0..5) {
                0 => format!("{} has ceased to function as a going concern. Its records have been transferred to the Archive of Defunct Bodies.", inst),
                1 => format!("The dissolution of {} was {} without ceremony. Its remaining assets, if any, were not enumerated.", inst, pick(PROCEDURAL_VERBS, rng)),
                2 => format!("{} was formally dissolved. The reasons given were 'insufficient membership and declining relevance.' A {} was filed {}.", inst, pick(BUREAUCRATIC_NOUNS, rng), pick(TEMPORAL_HEDGES, rng)),
                3 => format!("The {} formerly known as {} no longer exists in any administratively meaningful sense. Its {} was {} for the final time.", pick(BUREAUCRATIC_NOUNS, rng), inst, pick(BUREAUCRATIC_NOUNS, rng), pick(PROCEDURAL_VERBS, rng)),
                _ => format!("{} was struck from the register of active organizations, a consequence of {}. No successor body was designated.", inst, pick(ABSURDIST_CAUSES, rng)),
            }
        }
        EventType::SchismOccurred => {
            match rng.gen_range(0..5) {
                0 => format!("A doctrinal rupture within {} has produced irreconcilable factions. The {} was {} but could not contain the disagreement.", inst, pick(BUREAUCRATIC_NOUNS, rng), pick(PROCEDURAL_VERBS, rng)),
                1 => format!("{} suffered a schism of considerable administrative consequence. The immediate cause was {}.", inst, pick(ABSURDIST_CAUSES, rng)),
                2 => format!("Internal disagreements within {} escalated beyond the capacity of its mediation procedures. A {} was convened but dissolved before reaching conclusion.", inst, pick(BUREAUCRATIC_NOUNS, rng)),
                3 => format!("The membership of {} fractured along lines that the organization's charter had not anticipated. Each faction {} the other's legitimacy.", inst, pick(PROCEDURAL_VERBS, rng)),
                _ => format!("{} split into opposing camps over a matter that both sides describe as 'fundamental.' Outside observers {} the dispute as 'largely procedural.'", inst, pick(PROCEDURAL_VERBS, rng)),
            }
        }
        EventType::DoctrineShifted => {
            match rng.gen_range(0..5) {
                0 => format!("{} has officially revised one of its foundational positions. The previous position was stricken from the record {}.", inst, pick(TEMPORAL_HEDGES, rng)),
                1 => format!("A doctrinal adjustment within {} was announced without explanation. Members were instructed to update their personal copies of the {}.", inst, pick(BUREAUCRATIC_NOUNS, rng)),
                2 => format!("{} quietly amended its official doctrine, citing {}. The amendment was {} by the internal {}.", inst, pick(ABSURDIST_CAUSES, rng), pick(PROCEDURAL_VERBS, rng), pick(BUREAUCRATIC_NOUNS, rng)),
                3 => format!("The doctrinal {} of {} was revised. The old position, which had stood for some time, was replaced with one that the leadership described as 'more current.'", pick(BUREAUCRATIC_NOUNS, rng), inst),
                _ => format!("{} issued a correction to its stated beliefs. The correction was {} to be a minor clarification; those familiar with the matter disagree.", inst, pick(PROCEDURAL_VERBS, rng)),
            }
        }
        EventType::AllianceFormed => {
            match rng.gen_range(0..5) {
                0 => format!("{} and {} have entered into a formal arrangement of mutual benefit. The {} was ratified {}.", inst, other, pick(BUREAUCRATIC_NOUNS, rng), pick(TEMPORAL_HEDGES, rng)),
                1 => format!("An alliance between {} and {} was {} with the appropriate signatures. The terms are to be reviewed {}.", inst, other, pick(PROCEDURAL_VERBS, rng), pick(TEMPORAL_HEDGES, rng)),
                2 => format!("{} extended a hand of cooperation to {}. The hand was accepted, provisionally, and a joint {} was established.", inst, other, pick(BUREAUCRATIC_NOUNS, rng)),
                3 => format!("Following protracted negotiation, {} and {} have agreed to coordinate their activities. A shared {} was {} to formalize the arrangement.", inst, other, pick(BUREAUCRATIC_NOUNS, rng), pick(PROCEDURAL_VERBS, rng)),
                _ => format!("{} and {} announced an alliance, surprising observers who had expected continued hostility. The agreement was attributed to {}.", inst, other, pick(ABSURDIST_CAUSES, rng)),
            }
        }
        EventType::AllianceStrained => {
            match rng.gen_range(0..5) {
                0 => format!("Relations between {} and {} have deteriorated over a matter that both parties describe differently. A {} was {} but not addressed.", inst, other, pick(BUREAUCRATIC_NOUNS, rng), pick(PROCEDURAL_VERBS, rng)),
                1 => format!("{} {} a formal complaint against {}. The complaint was acknowledged {}.", inst, pick(PROCEDURAL_VERBS, rng), other, pick(TEMPORAL_HEDGES, rng)),
                2 => format!("Tensions between {} and {} reached a level that required the appointment of a mediator. No mediator was appointed, owing to {}.", inst, other, pick(ABSURDIST_CAUSES, rng)),
                3 => format!("The relationship between {} and {} was downgraded from 'cooperative' to 'under review.' The underlying cause was {} as {}.", inst, other, pick(PROCEDURAL_VERBS, rng), pick(ABSURDIST_CAUSES, rng)),
                _ => format!("{} and {} exchanged formal objections regarding {}. Neither objection was resolved. Both were filed.", inst, other, pick(ABSURDIST_CAUSES, rng)),
            }
        }
        EventType::RivalryDeclared => {
            match rng.gen_range(0..5) {
                0 => format!("{} has declared {} to be operating in opposition to its interests. A {} was {} to document the grievance.", inst, other, pick(BUREAUCRATIC_NOUNS, rng), pick(PROCEDURAL_VERBS, rng)),
                1 => format!("A state of formal rivalry now exists between {} and {}. The declaration cited {} as the precipitating factor.", inst, other, pick(ABSURDIST_CAUSES, rng)),
                2 => format!("{} publicly denounced {} in terms that left little room for diplomatic interpretation. A response is expected {}.", inst, other, pick(TEMPORAL_HEDGES, rng)),
                3 => format!("The {} between {} and {} has been officially classified as adversarial. Both organizations {} the classification.", pick(BUREAUCRATIC_NOUNS, rng), inst, other, pick(PROCEDURAL_VERBS, rng)),
                _ => format!("{} issued a formal denunciation of {}, citing grievances accumulated over a period the {} described as 'sufficient.'", inst, other, pick(BUREAUCRATIC_NOUNS, rng)),
            }
        }
        EventType::MemberJoined => {
            match rng.gen_range(0..5) {
                0 => format!("{} was admitted to the ranks of {}. The initiation {} was {}.", agent, inst, pick(BUREAUCRATIC_NOUNS, rng), pick(PROCEDURAL_VERBS, rng)),
                1 => format!("{} formally joined {}. Their provisional membership period begins immediately and extends {}.", agent, inst, pick(TEMPORAL_HEDGES, rng)),
                2 => format!("{} accepted {} as a member, following a review process described as 'perfunctory.' The relevant {} was completed.", inst, agent, pick(BUREAUCRATIC_NOUNS, rng)),
                3 => format!("The membership rolls of {} were updated to include {}, a development the {} {} without further comment.", inst, agent, pick(BUREAUCRATIC_NOUNS, rng), pick(PROCEDURAL_VERBS, rng)),
                _ => format!("{} applied for membership in {} and was accepted {}. The application cited {} as motivation.", agent, inst, pick(TEMPORAL_HEDGES, rng), pick(ABSURDIST_CAUSES, rng)),
            }
        }
        EventType::MemberDeparted => {
            match rng.gen_range(0..5) {
                0 => format!("{} departed from {}, citing reasons that were not entered into the record. The exit {} was declined.", agent, inst, pick(BUREAUCRATIC_NOUNS, rng)),
                1 => format!("{} terminated their affiliation with {}. The {} was updated {}.", agent, inst, pick(BUREAUCRATIC_NOUNS, rng), pick(TEMPORAL_HEDGES, rng)),
                2 => format!("{} quietly removed itself from the membership rolls of {}, a process the {} described as 'routine.'", agent, inst, pick(BUREAUCRATIC_NOUNS, rng)),
                3 => format!("The departure of {} from {} was {} by the internal office. The cause was attributed to {}.", agent, inst, pick(PROCEDURAL_VERBS, rng), pick(ABSURDIST_CAUSES, rng)),
                _ => format!("{} ceased to be affiliated with {}. The relevant {} was {} and the matter considered closed.", agent, inst, pick(BUREAUCRATIC_NOUNS, rng), pick(PROCEDURAL_VERBS, rng)),
            }
        }
        EventType::MemberExpelled => {
            match rng.gen_range(0..5) {
                0 => format!("{} was expelled from {} on grounds that the internal {} declined to make public.", agent, inst, pick(BUREAUCRATIC_NOUNS, rng)),
                1 => format!("{} formally removed {} from its membership for reasons described as 'procedural.' The expulsion {} cited {} infractions.", inst, agent, pick(BUREAUCRATIC_NOUNS, rng), rng.gen_range(3..=17)),
                2 => format!("{} was ejected from {}. The {} notice cited {}, only three of which were specified.", agent, inst, pick(BUREAUCRATIC_NOUNS, rng), pick(ABSURDIST_CAUSES, rng)),
                3 => format!("The membership of {} in {} was revoked following a {} that the internal {} described as 'conclusive.'", agent, inst, pick(BUREAUCRATIC_NOUNS, rng), pick(BUREAUCRATIC_NOUNS, rng)),
                _ => format!("{} {} the removal of {} from its rolls. The decision, once {}, was considered final {}.", inst, pick(PROCEDURAL_VERBS, rng), agent, pick(PROCEDURAL_VERBS, rng), pick(TEMPORAL_HEDGES, rng)),
            }
        }
        _ => format!("An institutional matter involving {} was resolved, or at least {}.", inst, pick(PROCEDURAL_VERBS, rng)),
    }
}

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
