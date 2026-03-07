// Dungeon and site generation.

use rand::rngs::StdRng;
use rand::Rng;

use crate::gen::name_gen;
use crate::sim::site::*;
use crate::sim::world::{MAP_HEIGHT, MAP_WIDTH, SettlementSize, Terrain};

/// Generate 4-8 sites placed on varied terrain across the world map.
pub fn generate_sites(
    terrain: &[Vec<Terrain>],
    phonemes: &[name_gen::PhonemeSet],
    institutions: &[(u64, String)],
    rng: &mut StdRng,
) -> Vec<Site> {
    let count = rng.gen_range(4..=8);
    let mut sites = Vec::new();

    // First 2 sites are always dungeons so the player has multi-floor sites to explore.
    // Remaining sites are drawn from the full pool.
    let guaranteed = [SiteKind::Dungeon, SiteKind::Dungeon];
    let random_pool = [
        SiteKind::Dungeon,
        SiteKind::Ruin,
        SiteKind::Ruin,
        SiteKind::Shrine,
        SiteKind::BureaucraticAnnex,
        SiteKind::ControversialTombsite,
        SiteKind::TaxonomicallyAmbiguousRegion,
        SiteKind::AbandonedInstitution,
    ];

    // Collect valid placement positions (not deep water, not shallow water)
    let mut candidates: Vec<(u32, u32)> = Vec::new();
    for y in 1..MAP_HEIGHT - 1 {
        for x in 1..MAP_WIDTH - 1 {
            let t = terrain[y][x];
            if t != Terrain::DeepWater && t != Terrain::ShallowWater {
                candidates.push((x as u32, y as u32));
            }
        }
    }

    let min_dist: u32 = 6;
    let mut attempts = 0;

    while sites.len() < count && attempts < 500 {
        attempts += 1;
        let (x, y) = candidates[rng.gen_range(0..candidates.len())];

        // Check distance from existing sites
        let too_close = sites.iter().any(|s: &Site| {
            let dx = (s.grid_x as i32 - x as i32).unsigned_abs();
            let dy = (s.grid_y as i32 - y as i32).unsigned_abs();
            dx + dy < min_dist
        });
        if too_close {
            continue;
        }

        let kind = if sites.len() < guaranteed.len() {
            guaranteed[sites.len()].clone()
        } else {
            random_pool[rng.gen_range(0..random_pool.len())].clone()
        };
        let id = sites.len() as u64;

        // Name the site
        let phoneme_idx = rng.gen_range(0..phonemes.len());
        let name = generate_site_name(&kind, &phonemes[phoneme_idx], rng);
        let origin = generate_origin(&kind, rng);

        // Dungeons get 2-4 floors; other sites get 1
        let floor_count = match kind {
            SiteKind::Dungeon => rng.gen_range(2..=4),
            SiteKind::Ruin => rng.gen_range(1..=2),
            _ => 1,
        };

        let mut floors = Vec::with_capacity(floor_count);
        for depth in 0..floor_count {
            floors.push(generate_floor(depth, depth == floor_count - 1, rng));
        }

        // Some sites are controlled by an existing institution
        let controlling_faction = if !institutions.is_empty() && rng.gen_bool(0.4) {
            let (id, _) = &institutions[rng.gen_range(0..institutions.len())];
            Some(*id)
        } else {
            None
        };

        let history_entry = format!("Discovered at tick 0. {}", origin);

        let inhabitants = generate_inhabitants(&kind, &floors, &phonemes[phoneme_idx], rng);

        sites.push(Site {
            id,
            name,
            kind,
            origin,
            grid_x: x,
            grid_y: y,
            floors,
            population: Vec::new(),
            artifacts: Vec::new(),
            history: vec![history_entry],
            controlling_faction,
            inhabitants,
        });
    }

    sites
}

/// Generate a name for a site based on its kind.
fn generate_site_name(kind: &SiteKind, set: &name_gen::PhonemeSet, rng: &mut StdRng) -> String {
    let cultural_word = name_gen::generate_name_part_public(set, 1, 2, rng);

    match kind {
        SiteKind::Dungeon => {
            let prefixes = [
                "The Vaults of", "The Warrens of", "The Crypts of",
                "The Deeps of", "The Cellars of", "The Labyrinth of",
            ];
            format!("{} {}", prefixes[rng.gen_range(0..prefixes.len())], cultural_word)
        }
        SiteKind::Ruin => {
            let prefixes = [
                "The Ruins of", "The Remnants of", "Old",
                "The Fallen Halls of", "The Wreckage of",
            ];
            format!("{} {}", prefixes[rng.gen_range(0..prefixes.len())], cultural_word)
        }
        SiteKind::Shrine => {
            let prefixes = [
                "The Shrine of", "The Sanctum of", "The Altar of",
                "The Chapel of the", "The Reliquary of",
            ];
            let adj = ["Ossified", "Provisional", "Accumulated", "Persistent", "Undisclosed"];
            format!("{} {} {}", prefixes[rng.gen_range(0..prefixes.len())],
                    adj[rng.gen_range(0..adj.len())], cultural_word)
        }
        SiteKind::BureaucraticAnnex => {
            let prefixes = [
                "The Annex of", "The Sub-Office of", "The Auxiliary Bureau of",
                "The Satellite Registry of", "The Outpost of the Department of",
            ];
            let nouns = ["Permits", "Reclassifications", "Deferred Obligations", "Archival Disputes", "Procedural Overflow"];
            format!("{} {}", prefixes[rng.gen_range(0..prefixes.len())], nouns[rng.gen_range(0..nouns.len())])
        }
        SiteKind::ControversialTombsite => {
            let prefixes = [
                "The Contested Tomb of", "The Disputed Resting Place of",
                "The Grave of", "The Mausoleum of",
            ];
            format!("{} {}", prefixes[rng.gen_range(0..prefixes.len())], cultural_word)
        }
        SiteKind::TaxonomicallyAmbiguousRegion => {
            let forms = [
                format!("The {} Anomaly", cultural_word),
                format!("The Unclassified Expanse of {}", cultural_word),
                format!("Zone {}: Taxonomic Status Pending", rng.gen_range(7..99)),
                format!("The {} Irregularity", cultural_word),
            ];
            forms[rng.gen_range(0..forms.len())].clone()
        }
        SiteKind::AbandonedInstitution => {
            let prefixes = [
                "The Former Offices of the", "The Defunct", "The Abandoned Chambers of the",
                "What Remains of the", "The Shuttered",
            ];
            let bodies = [
                "Bureau of Unresolved Matters", "Commission on Prior Obligations",
                "Registry of Forgotten Claims", "Board of Discontinued Services",
                "Office of Terminal Appointments",
            ];
            format!("{} {}", prefixes[rng.gen_range(0..prefixes.len())], bodies[rng.gen_range(0..bodies.len())])
        }
    }
}

/// Generate an origin story for a site.
fn generate_origin(kind: &SiteKind, rng: &mut StdRng) -> String {
    match kind {
        SiteKind::Dungeon => {
            let origins = [
                "Excavated by an institution that no longer exists, for purposes that were never formally documented.",
                "Originally a mine. The miners found something. The records do not specify what.",
                "Constructed as a secure repository for objects of disputed provenance.",
                "Built by parties unknown, at a date the archaeologists continue to argue about.",
                "A natural cave system that was expanded with evident purpose but unclear intent.",
            ];
            origins[rng.gen_range(0..origins.len())].to_string()
        }
        SiteKind::Ruin => {
            let origins = [
                "Once a settlement of some consequence. Its decline was neither sudden nor well-documented.",
                "Destroyed during an institutional dispute that escalated beyond administrative resolution.",
                "Abandoned after a census revealed the population had already left.",
                "Collapsed due to what the official report describes as 'structural disagreement.'",
            ];
            origins[rng.gen_range(0..origins.len())].to_string()
        }
        SiteKind::Shrine => {
            let origins = [
                "Erected to honor a principle that the builders could not fully articulate.",
                "Built at the site of an event whose nature is disputed by all surviving accounts.",
                "Maintained by a succession of custodians who each understood its purpose differently.",
            ];
            origins[rng.gen_range(0..origins.len())].to_string()
        }
        SiteKind::BureaucraticAnnex => {
            let origins = [
                "Established when the primary office ran out of filing space.",
                "Created to process a category of requests that no other office would accept.",
                "Founded during an administrative reorganization that was itself reorganized before completion.",
            ];
            origins[rng.gen_range(0..origins.len())].to_string()
        }
        SiteKind::ControversialTombsite => {
            let origins = [
                "The occupant's identity is disputed. Three factions each claim it as their own.",
                "Burial here was conducted without the required permits. The permits remain unfiled.",
                "The tomb predates the civilization that claims to have built it.",
            ];
            origins[rng.gen_range(0..origins.len())].to_string()
        }
        SiteKind::TaxonomicallyAmbiguousRegion => {
            let origins = [
                "The terrain here defies standard classification. Several surveying expeditions have returned with contradictory reports.",
                "Something happened here that altered the local environment in ways that remain formally undescribed.",
                "The region was omitted from official maps, reportedly by accident, for several consecutive editions.",
            ];
            origins[rng.gen_range(0..origins.len())].to_string()
        }
        SiteKind::AbandonedInstitution => {
            let origins = [
                "Its staff departed when their mandate expired. The mandate was never formally concluded.",
                "Closed during budget reconciliation. The reconciliation is technically still in progress.",
                "Abandoned after the last employee was transferred to a department that did not exist.",
            ];
            origins[rng.gen_range(0..origins.len())].to_string()
        }
    }
}

/// Generate a single floor using room-and-corridor algorithm.
fn generate_floor(depth: usize, is_last: bool, rng: &mut StdRng) -> Floor {
    let mut tiles = vec![vec![Tile::Wall; FLOOR_WIDTH]; FLOOR_HEIGHT];
    let mut rooms = Vec::new();

    let room_count = rng.gen_range(4..=8);
    let mut attempts = 0;

    while rooms.len() < room_count && attempts < 200 {
        attempts += 1;

        let w = rng.gen_range(4..=10);
        let h = rng.gen_range(3..=6);
        let x = rng.gen_range(1..FLOOR_WIDTH.saturating_sub(w + 1));
        let y = rng.gen_range(1..FLOOR_HEIGHT.saturating_sub(h + 1));

        // Check overlap with existing rooms (with 1-tile padding)
        let overlaps = rooms.iter().any(|r: &Room| {
            x < r.x + r.w + 1 && x + w + 1 > r.x && y < r.y + r.h + 1 && y + h + 1 > r.y
        });
        if overlaps {
            continue;
        }

        let purpose = match rng.gen_range(0..6) {
            0 => RoomPurpose::Storage,
            1 => RoomPurpose::Ritual,
            2 => RoomPurpose::Administrative,
            3 => RoomPurpose::Habitation,
            4 => RoomPurpose::Trophy,
            _ => RoomPurpose::Disputed,
        };

        // Carve the room
        for ry in y..y + h {
            for rx in x..x + w {
                tiles[ry][rx] = Tile::Floor;
            }
        }

        rooms.push(Room { x, y, w, h, purpose });
    }

    // Connect rooms with corridors
    for i in 1..rooms.len() {
        let (cx1, cy1) = rooms[i - 1].center();
        let (cx2, cy2) = rooms[i].center();
        carve_corridor(&mut tiles, cx1, cy1, cx2, cy2, rng);
    }

    // Place doors at corridor-room junctions
    place_doors(&mut tiles, &rooms, rng);

    // Place stairs: up on non-first floors, down on non-last floors
    if depth > 0 {
        if let Some(room) = rooms.first() {
            let (cx, cy) = room.center();
            tiles[cy][cx] = Tile::StairUp;
        }
    }
    if !is_last {
        if let Some(room) = rooms.last() {
            let (cx, cy) = room.center();
            tiles[cy][cx] = Tile::StairDown;
        }
    }

    // Scatter a few water/pit hazards
    let hazard_count = rng.gen_range(0..=3);
    for _ in 0..hazard_count {
        let rx = rng.gen_range(1..FLOOR_WIDTH - 1);
        let ry = rng.gen_range(1..FLOOR_HEIGHT - 1);
        if tiles[ry][rx] == Tile::Floor {
            tiles[ry][rx] = if rng.gen_bool(0.5) { Tile::Water } else { Tile::Pit };
        }
    }

    Floor {
        depth,
        tiles,
        rooms,
    }
}

/// Carve an L-shaped corridor between two points.
fn carve_corridor(tiles: &mut Vec<Vec<Tile>>, x1: usize, y1: usize, x2: usize, y2: usize, rng: &mut StdRng) {
    // Randomly choose horizontal-first or vertical-first
    if rng.gen_bool(0.5) {
        carve_h(tiles, x1, x2, y1);
        carve_v(tiles, y1, y2, x2);
    } else {
        carve_v(tiles, y1, y2, x1);
        carve_h(tiles, x1, x2, y2);
    }
}

fn carve_h(tiles: &mut Vec<Vec<Tile>>, x1: usize, x2: usize, y: usize) {
    let (start, end) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
    for x in start..=end {
        if y < FLOOR_HEIGHT && x < FLOOR_WIDTH {
            if tiles[y][x] == Tile::Wall {
                tiles[y][x] = Tile::Floor;
            }
        }
    }
}

fn carve_v(tiles: &mut Vec<Vec<Tile>>, y1: usize, y2: usize, x: usize) {
    let (start, end) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
    for y in start..=end {
        if y < FLOOR_HEIGHT && x < FLOOR_WIDTH {
            if tiles[y][x] == Tile::Wall {
                tiles[y][x] = Tile::Floor;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn dungeon_floors_are_2_to_4() {
        let phonemes = crate::gen::name_gen::load_phoneme_data();
        let terrain = vec![vec![Terrain::Plains; MAP_WIDTH]; MAP_HEIGHT];
        let institutions = vec![];

        // Test 20 different seeds
        for seed in 0..20 {
            let mut rng = StdRng::seed_from_u64(seed);
            let sites = generate_sites(&terrain, &phonemes, &institutions, &mut rng);

            for site in &sites {
                let floors = site.floors.len();
                match site.kind {
                    SiteKind::Dungeon => {
                        assert!(
                            floors >= 2 && floors <= 4,
                            "Seed {}: Dungeon '{}' has {} floors (expected 2-4)",
                            seed, site.name, floors
                        );
                    }
                    SiteKind::Ruin => {
                        assert!(
                            floors >= 1 && floors <= 2,
                            "Seed {}: Ruin '{}' has {} floors (expected 1-2)",
                            seed, site.name, floors
                        );
                    }
                    _ => {
                        assert_eq!(
                            floors, 1,
                            "Seed {}: {} '{}' has {} floors (expected 1)",
                            seed, site.kind.label(), site.name, floors
                        );
                    }
                }
            }

            // Verify at least 2 dungeons exist (guaranteed slots)
            let dungeon_count = sites.iter().filter(|s| s.kind == SiteKind::Dungeon).count();
            assert!(
                dungeon_count >= 2,
                "Seed {}: only {} dungeons (expected at least 2)",
                seed, dungeon_count
            );
        }
    }
}

/// Generate permanent inhabitants for a site based on its kind.
fn generate_inhabitants(
    kind: &SiteKind,
    floors: &[Floor],
    phoneme_set: &name_gen::PhonemeSet,
    rng: &mut StdRng,
) -> Vec<SiteInhabitant> {
    let count = rng.gen_range(2..=8);
    let mut inhabitants = Vec::new();

    for i in 0..count {
        let floor_idx = rng.gen_range(0..floors.len());
        let floor = &floors[floor_idx];

        // Place inhabitant in a room on their floor
        let (x, y) = if !floor.rooms.is_empty() {
            let room = &floor.rooms[rng.gen_range(0..floor.rooms.len())];
            (
                rng.gen_range(room.x..room.x + room.w),
                rng.gen_range(room.y..room.y + room.h),
            )
        } else {
            (FLOOR_WIDTH / 2, FLOOR_HEIGHT / 2)
        };

        let (name, description, glyph) = generate_inhabitant_details(kind, phoneme_set, rng, i);

        inhabitants.push(SiteInhabitant {
            id: i,
            name,
            description,
            glyph,
            floor: floor_idx,
            x,
            y,
        });
    }

    inhabitants
}

/// Generate name, description, and glyph for an inhabitant based on site kind.
fn generate_inhabitant_details(
    kind: &SiteKind,
    phoneme_set: &name_gen::PhonemeSet,
    rng: &mut StdRng,
    _index: usize,
) -> (String, String, char) {
    match kind {
        SiteKind::Dungeon => {
            let names = [
                "The Residual Clerk",
                "Something That Was Once a Clerk",
                "The Unnamed Occupant",
                "A Former Surveyor",
                "The Thing in the Corner",
                "An Entity the Survey Team Declined to Classify",
                "The Remnant Custodian",
                "What the Ledger Calls 'Occupant VII'",
            ];
            let descs = [
                "Something that was once a clerk, or perhaps still is — the distinction has become academic.",
                "An entity the survey team declined to classify, citing insufficient categories.",
                "A figure whose presence predates the current filing system.",
                "Whatever remains of the last person assigned to this post.",
                "An occupant whose employment status has been under review for longer than the reviewing body has existed.",
                "Something that moves with purpose but without any purpose the observer can identify.",
                "A presence that the census consistently fails to enumerate correctly.",
                "An individual whose continued existence contradicts at least one official record.",
            ];
            (
                names[rng.gen_range(0..names.len())].to_string(),
                descs[rng.gen_range(0..descs.len())].to_string(),
                'c', // creature
            )
        }
        SiteKind::Ruin => {
            let name = name_gen::generate_name_part_public(phoneme_set, 1, 2, rng);
            let descs = [
                "A remnant occupant who never received notification of the evacuation.",
                "A squatter whose tenancy now exceeds that of the original builders.",
                "Something that predates the current administrative regime by a comfortable margin.",
                "An individual who claims prior residency under a legal framework that no longer exists.",
                "A figure who has been here longer than the walls, and appears more structurally sound.",
            ];
            (
                name,
                descs[rng.gen_range(0..descs.len())].to_string(),
                'r', // remnant
            )
        }
        SiteKind::Shrine => {
            let name = name_gen::generate_name_part_public(phoneme_set, 1, 2, rng);
            let titles = ["Attendant", "Custodian", "Devoted", "Keeper", "Watcher"];
            let full_name = format!("{} the {}", name, titles[rng.gen_range(0..titles.len())]);
            let descs = [
                "An attendant of unclear affiliation whose duties appear to be self-assigned.",
                "A custodian who maintains the shrine according to a schedule they alone understand.",
                "A devoted person whose devotion is to something the shrine may or may not represent.",
                "A keeper whose keeping consists primarily of being present and occasionally disapproving.",
                "An individual whose relationship to the shrine defies all standard classifications of employment.",
            ];
            (
                full_name,
                descs[rng.gen_range(0..descs.len())].to_string(),
                's', // shrine attendant
            )
        }
        SiteKind::BureaucraticAnnex => {
            let name = name_gen::generate_name_part_public(phoneme_set, 1, 2, rng);
            let titles = ["Filing Clerk", "Sub-Registrar", "Assistant to the Deputy", "Provisional Secretary", "Archivist (Interim)"];
            let full_name = format!("{}, {}", name, titles[rng.gen_range(0..titles.len())]);
            let descs = [
                "A filing clerk whose employment status is itself the subject of an unresolved filing.",
                "Staff of uncertain origin whose payroll records reference a department that does not exist.",
                "An archivist who continues to archive despite the absence of anyone to archive for.",
                "A clerk who processes forms that no one submits, with an efficiency that borders on the devotional.",
                "An employee whose hiring paperwork was lost, creating a status the office terms 'administratively theoretical.'",
            ];
            (
                full_name,
                descs[rng.gen_range(0..descs.len())].to_string(),
                'b', // bureaucrat
            )
        }
        SiteKind::ControversialTombsite => {
            let name = name_gen::generate_name_part_public(phoneme_set, 1, 2, rng);
            let roles = ["Mourner", "Investigator", "Claimant", "Vigil-Keeper", "Petitioner"];
            let role = roles[rng.gen_range(0..roles.len())];
            let full_name = format!("{} the {}", name, role);
            let descs = [
                "A mourner whose grief appears to be professionally maintained.",
                "An investigator examining claims that predate the investigation itself.",
                "A party with a claim whose validity depends on which calendar one consults.",
                "A vigil-keeper who has outlasted the purpose of the vigil but not the habit.",
                "Someone who is here to represent interests that have never been formally articulated.",
            ];
            (
                full_name,
                descs[rng.gen_range(0..descs.len())].to_string(),
                'm', // mourner
            )
        }
        SiteKind::TaxonomicallyAmbiguousRegion => {
            let descs = [
                "A thing that resists classification with what can only be described as intent.",
                "An entity whose taxonomy is the subject of a dispute between three academic bodies, none of which agree on the criteria.",
                "Something the field guide describes only as 'see appendix,' though no appendix exists.",
                "A presence that the survey team documented using a symbol they invented for the purpose and have since forgotten.",
                "An organism — if that is the right word, and it may not be — of indeterminate phylum.",
            ];
            let names = [
                "Specimen Unclassified",
                "The Unnamed Taxonomy",
                "Subject Pending Review",
                "The Categorical Exception",
                "Entity (See Footnote)",
            ];
            (
                names[rng.gen_range(0..names.len())].to_string(),
                descs[rng.gen_range(0..descs.len())].to_string(),
                't', // taxonomic anomaly
            )
        }
        SiteKind::AbandonedInstitution => {
            let name = name_gen::generate_name_part_public(phoneme_set, 1, 2, rng);
            let titles = ["Former Deputy", "Unreleased Employee", "Acting Director (Expired)", "Clerk (Unfired)", "Interim Permanent Secretary"];
            let full_name = format!("{}, {}", name, titles[rng.gen_range(0..titles.len())]);
            let descs = [
                "A former member who did not receive the memo regarding dissolution, or received it and filed an objection.",
                "An employee who never left, owing to a clause in their contract that no one remembers writing.",
                "Someone who continues to report for duty at an institution that has not existed for some time.",
                "A staff member whose termination paperwork was lost in the same event that terminated the institution.",
                "An individual who maintains that the institution still exists, citing bylaws that the bylaws themselves do not reference.",
            ];
            (
                full_name,
                descs[rng.gen_range(0..descs.len())].to_string(),
                'a', // abandoned staff
            )
        }
    }
}

/// Generate a floor plan for a settlement. Produces a single floor with civic
/// buildings connected by streets. Size determines building count and density.
pub fn generate_settlement_floor(size: &SettlementSize, rng: &mut StdRng) -> Floor {
    let mut tiles = vec![vec![Tile::Wall; FLOOR_WIDTH]; FLOOR_HEIGHT];
    let mut rooms = Vec::new();

    // Building count and size ranges vary by settlement size
    let (target_rooms, min_w, max_w, min_h, max_h) = match size {
        SettlementSize::Hamlet => (rng.gen_range(3..=5), 4, 7, 3, 5),
        SettlementSize::Town => (rng.gen_range(6..=9), 4, 9, 3, 6),
        SettlementSize::City => (rng.gen_range(10..=14), 4, 10, 3, 6),
    };

    // Civic purposes cycle — ensures variety
    let civic_purposes = [
        RoomPurpose::Tavern,
        RoomPurpose::Market,
        RoomPurpose::Administrative,
        RoomPurpose::Temple,
        RoomPurpose::Residential,
        RoomPurpose::Warehouse,
        RoomPurpose::Garrison,
        RoomPurpose::Residential,
        RoomPurpose::Residential,
        RoomPurpose::Tavern,
        RoomPurpose::Market,
        RoomPurpose::Residential,
        RoomPurpose::Warehouse,
        RoomPurpose::Residential,
    ];

    let mut attempts = 0;
    while rooms.len() < target_rooms && attempts < 400 {
        attempts += 1;

        let w = rng.gen_range(min_w..=max_w);
        let h = rng.gen_range(min_h..=max_h);
        let x = rng.gen_range(1..FLOOR_WIDTH.saturating_sub(w + 1));
        let y = rng.gen_range(1..FLOOR_HEIGHT.saturating_sub(h + 1));

        // Check overlap with 2-tile padding (leaves room for streets)
        let overlaps = rooms.iter().any(|r: &Room| {
            x < r.x + r.w + 2 && x + w + 2 > r.x && y < r.y + r.h + 2 && y + h + 2 > r.y
        });
        if overlaps {
            continue;
        }

        let purpose = civic_purposes[rooms.len() % civic_purposes.len()].clone();

        // Carve the building interior
        for ry in y..y + h {
            for rx in x..x + w {
                tiles[ry][rx] = Tile::Floor;
            }
        }

        rooms.push(Room { x, y, w, h, purpose });
    }

    // Carve streets connecting buildings — wider paths (2 tiles) where possible
    for i in 1..rooms.len() {
        let (cx1, cy1) = rooms[i - 1].center();
        let (cx2, cy2) = rooms[i].center();
        carve_street(&mut tiles, cx1, cy1, cx2, cy2, rng);
    }
    // Extra cross-connections for cities and towns (settlements feel more connected)
    if rooms.len() > 4 {
        let extra = match size {
            SettlementSize::City => rng.gen_range(2..=4),
            SettlementSize::Town => rng.gen_range(1..=2),
            SettlementSize::Hamlet => 0,
        };
        for _ in 0..extra {
            let a = rng.gen_range(0..rooms.len());
            let b = rng.gen_range(0..rooms.len());
            if a != b {
                let (cx1, cy1) = rooms[a].center();
                let (cx2, cy2) = rooms[b].center();
                carve_street(&mut tiles, cx1, cy1, cx2, cy2, rng);
            }
        }
    }

    // Place doors at building entrances
    place_doors(&mut tiles, &rooms, rng);

    // Add a well or fountain in larger settlements (Water tile as feature)
    if matches!(size, SettlementSize::Town | SettlementSize::City) && rooms.len() >= 2 {
        // Place near the center of the map, on a street tile
        let cx = FLOOR_WIDTH / 2;
        let cy = FLOOR_HEIGHT / 2;
        // Search outward from center for a floor tile
        for r in 0..6 {
            let mut placed = false;
            for dy in (cy.saturating_sub(r))..=(cy + r).min(FLOOR_HEIGHT - 1) {
                for dx in (cx.saturating_sub(r))..=(cx + r).min(FLOOR_WIDTH - 1) {
                    if tiles[dy][dx] == Tile::Floor {
                        tiles[dy][dx] = Tile::Water;
                        placed = true;
                        break;
                    }
                }
                if placed { break; }
            }
            if placed { break; }
        }
    }

    Floor {
        depth: 0,
        tiles,
        rooms,
    }
}

/// Carve a street (wider corridor) between two points.
fn carve_street(tiles: &mut Vec<Vec<Tile>>, x1: usize, y1: usize, x2: usize, y2: usize, rng: &mut StdRng) {
    // Streets carve the main line plus occasionally an adjacent parallel line
    if rng.gen_bool(0.5) {
        carve_h(tiles, x1, x2, y1);
        if y1 > 0 && y1 < FLOOR_HEIGHT - 1 {
            carve_h(tiles, x1, x2, y1 + 1); // widen street
        }
        carve_v(tiles, y1, y2, x2);
    } else {
        carve_v(tiles, y1, y2, x1);
        carve_h(tiles, x1, x2, y2);
        if y2 > 0 && y2 < FLOOR_HEIGHT - 1 {
            carve_h(tiles, x1, x2, y2 + 1);
        }
    }
}

/// Place doors at transitions between corridors and rooms.
fn place_doors(tiles: &mut Vec<Vec<Tile>>, rooms: &[Room], rng: &mut StdRng) {
    for room in rooms {
        // Check room edges for corridor entrances
        let edges = [
            // Top edge
            (room.x..room.x + room.w).map(|x| (x, room.y.wrapping_sub(1))).collect::<Vec<_>>(),
            // Bottom edge
            (room.x..room.x + room.w).map(|x| (x, room.y + room.h)).collect::<Vec<_>>(),
            // Left edge
            (room.y..room.y + room.h).map(|y| (room.x.wrapping_sub(1), y)).collect::<Vec<_>>(),
            // Right edge
            (room.y..room.y + room.h).map(|y| (room.x + room.w, y)).collect::<Vec<_>>(),
        ];

        for edge in &edges {
            for &(ex, ey) in edge {
                if ey >= FLOOR_HEIGHT || ex >= FLOOR_WIDTH {
                    continue;
                }
                if tiles[ey][ex] == Tile::Floor {
                    // This is a corridor tile adjacent to a room wall — potential door spot
                    // Only place a door ~40% of the time to keep things varied
                    if rng.gen_bool(0.4) {
                        tiles[ey][ex] = Tile::Door;
                    }
                }
            }
        }
    }
}
