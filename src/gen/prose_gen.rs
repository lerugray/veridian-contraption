use rand::rngs::StdRng;
use rand::Rng;

use crate::sim::event::EventType;
use crate::sim::world::World;

/// Generate the prose description for an event, given the world context.
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
            let epithets = [
                "of no particular distinction",
                "formerly in good standing",
                "whose paperwork remained incomplete",
                "late of several appointments",
                "whose debts were subsequently forgiven",
                "the twice-mentioned",
                "of disputed provenance",
                "whose census entry has been struck through",
            ];
            let closings = [
                "No formal inquiry was opened.",
                "The relevant authorities were not notified in time.",
                "A clerk noted the absence in the margin of an unrelated ledger.",
                "The matter was filed under 'resolved by circumstance.'",
                "This was recorded and promptly misfiled.",
                "The vacancy has not yet been filled.",
            ];
            let ep = epithets[rng.gen_range(0..epithets.len())];
            let cl = closings[rng.gen_range(0..closings.len())];
            format!("{}, {}, ceased to be present in {}. {}", name, ep, loc, cl)
        }

        EventType::AgentArrived => {
            let manners = [
                "without prior notice or evident purpose",
                "bearing documentation of uncertain validity",
                "in a state suggesting recent travel",
                "with the air of someone who has been expected elsewhere",
                "claiming business with no one in particular",
                "and was provisionally noted in the register",
            ];
            let m = manners[rng.gen_range(0..manners.len())];
            format!("{} arrived at {} {}.", name, loc, m)
        }

        EventType::AgentDeparted => {
            let reasons = [
                "citing personal obligations of an unspecified nature",
                "without filing the customary notice of departure",
                "having concluded business that no record describes",
                "under circumstances the local clerk declined to elaborate upon",
                "leaving behind several unsigned documents",
                "in what was later described as 'an unremarkable exit'",
            ];
            let r = reasons[rng.gen_range(0..reasons.len())];
            format!("{} departed from {} {}.", name, loc, r)
        }

        EventType::SettlementGrew => {
            let details = [
                "The increase was noted and filed.",
                "A clerk expressed cautious optimism, then retracted the statement.",
                "The additional residents were assigned provisional status.",
                "This growth was attributed to factors the administration declined to specify.",
                "The housing register was updated with reluctant precision.",
            ];
            let d = details[rng.gen_range(0..details.len())];
            format!(
                "The settlement of {} recorded an increase in its registered population. {}",
                loc, d
            )
        }

        EventType::SettlementShrank => {
            let details = [
                "The decrease was attributed to 'general attrition.'",
                "Several addresses were reclassified as 'potentially occupied.'",
                "The census office noted the discrepancy but offered no correction.",
                "A minor official suggested the figures may have been previously inflated.",
                "The shortfall was absorbed into the next quarter's projections.",
            ];
            let d = details[rng.gen_range(0..details.len())];
            format!(
                "The population of {} experienced a documented reduction. {}",
                loc, d
            )
        }

        EventType::WeatherEvent => {
            let conditions = [
                "unseasonably damp", "oppressively still", "characterized by an amber haze",
                "marked by a persistent low wind", "colder than administrative guidelines suggest",
                "warm in a manner several residents described as 'suspicious'",
                "foggy beyond what the local almanac had predicted",
                "punctuated by brief intervals of something not quite rain",
            ];
            let causes = [
                "prevailing atmospheric indifference",
                "a seasonal pattern the meteorological office has yet to name",
                "conditions upstream that no one has taken responsibility for",
                "the natural consequence of geography",
                "factors the Bureau of Ambient Conditions is still reviewing",
                "what one official termed 'the usual arrangement'",
            ];
            let c = conditions[rng.gen_range(0..conditions.len())];
            let ca = causes[rng.gen_range(0..causes.len())];
            format!(
                "Conditions in the vicinity of {} became {}. This was attributed to {}.",
                loc, c, ca
            )
        }

        EventType::AgeEvent => {
            let milestones = [
                "has persisted in the world for a notable duration",
                "has survived long enough to become a matter of minor administrative interest",
                "continues to occupy their census entry with considerable tenacity",
                "has reached an age that the actuarial tables regard with skepticism",
            ];
            let m = milestones[rng.gen_range(0..milestones.len())];
            format!("{} of {} {}.", name, loc, m)
        }

        EventType::CensusReport => {
            // This one is handled directly in the tick() method with specific counts
            format!("A census was conducted. The results were filed.")
        }

        EventType::WorldGenesis => {
            format!("The world stirs into being. Somewhere, a ledger is opened.")
        }

        EventType::AgentBorn => {
            let circumstances = [
                "under circumstances the registrar described as 'standard'",
                "to the apparent surprise of the local census office",
                "and was assigned a provisional identity number",
                "amid paperwork that had already been prepared",
                "without the customary advance notification to the Bureau of New Arrivals",
            ];
            let c = circumstances[rng.gen_range(0..circumstances.len())];
            format!("{} entered the records of {} {}.", name, loc, c)
        }
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
