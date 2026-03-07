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
            floors.push(generate_site_floor(&kind, depth, depth == floor_count - 1, rng));
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

/// Dispatch floor generation to the appropriate per-kind generator.
fn generate_site_floor(kind: &SiteKind, depth: usize, is_last: bool, rng: &mut StdRng) -> Floor {
    match kind {
        SiteKind::Dungeon => generate_dungeon_floor(depth, is_last, rng),
        SiteKind::Ruin => generate_ruin_floor(depth, is_last, rng),
        SiteKind::Shrine => generate_shrine_floor(depth, is_last, rng),
        SiteKind::BureaucraticAnnex => generate_annex_floor(depth, is_last, rng),
        SiteKind::ControversialTombsite => generate_tombsite_floor(depth, is_last, rng),
        SiteKind::TaxonomicallyAmbiguousRegion => generate_ambiguous_floor(depth, is_last, rng),
        SiteKind::AbandonedInstitution => generate_abandoned_floor(depth, is_last, rng),
    }
}

/// DUNGEON: Deep grey rooms, corridors, water/pit hazards that stand out.
fn generate_dungeon_floor(depth: usize, is_last: bool, rng: &mut StdRng) -> Floor {
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

        for ry in y..y + h {
            for rx in x..x + w {
                tiles[ry][rx] = Tile::Floor;
            }
        }

        rooms.push(Room { x, y, w, h, purpose });
    }

    for i in 1..rooms.len() {
        let (cx1, cy1) = rooms[i - 1].center();
        let (cx2, cy2) = rooms[i].center();
        carve_corridor(&mut tiles, cx1, cy1, cx2, cy2, rng);
    }

    place_doors(&mut tiles, &rooms, rng);
    place_stairs(&mut tiles, &rooms, depth, is_last);

    // More hazards in dungeons — water and pits stand out against dark floor
    let hazard_count = rng.gen_range(2..=5);
    for _ in 0..hazard_count {
        let rx = rng.gen_range(1..FLOOR_WIDTH - 1);
        let ry = rng.gen_range(1..FLOOR_HEIGHT - 1);
        if tiles[ry][rx] == Tile::Floor {
            tiles[ry][rx] = if rng.gen_bool(0.5) { Tile::Water } else { Tile::Pit };
        }
    }

    Floor { depth, tiles, rooms }
}

/// RUIN: Partially collapsed walls, rubble, open sky sections.
fn generate_ruin_floor(depth: usize, is_last: bool, rng: &mut StdRng) -> Floor {
    // Start with a normal structure, then damage it
    let mut tiles = vec![vec![Tile::Wall; FLOOR_WIDTH]; FLOOR_HEIGHT];
    let mut rooms = Vec::new();

    let room_count = rng.gen_range(4..=7);
    let mut attempts = 0;

    while rooms.len() < room_count && attempts < 200 {
        attempts += 1;
        let w = rng.gen_range(4..=10);
        let h = rng.gen_range(3..=6);
        let x = rng.gen_range(1..FLOOR_WIDTH.saturating_sub(w + 1));
        let y = rng.gen_range(1..FLOOR_HEIGHT.saturating_sub(h + 1));

        let overlaps = rooms.iter().any(|r: &Room| {
            x < r.x + r.w + 1 && x + w + 1 > r.x && y < r.y + r.h + 1 && y + h + 1 > r.y
        });
        if overlaps { continue; }

        let purpose = match rng.gen_range(0..4) {
            0 => RoomPurpose::Storage,
            1 => RoomPurpose::Habitation,
            2 => RoomPurpose::Trophy,
            _ => RoomPurpose::Disputed,
        };

        for ry in y..y + h {
            for rx in x..x + w {
                tiles[ry][rx] = Tile::Floor;
            }
        }
        rooms.push(Room { x, y, w, h, purpose });
    }

    for i in 1..rooms.len() {
        let (cx1, cy1) = rooms[i - 1].center();
        let (cx2, cy2) = rooms[i].center();
        carve_corridor(&mut tiles, cx1, cy1, cx2, cy2, rng);
    }
    place_doors(&mut tiles, &rooms, rng);
    place_stairs(&mut tiles, &rooms, depth, is_last);

    // DAMAGE PASS: convert some walls to rubble, some floor to open sky
    for y in 0..FLOOR_HEIGHT {
        for x in 0..FLOOR_WIDTH {
            match tiles[y][x] {
                Tile::Wall => {
                    // ~25% of walls collapse to rubble
                    if rng.gen_bool(0.25) {
                        tiles[y][x] = Tile::Rubble;
                    }
                }
                Tile::Floor => {
                    // ~15% of floor becomes open sky (collapsed roof)
                    if rng.gen_bool(0.15) {
                        tiles[y][x] = Tile::OpenSky;
                    }
                    // ~5% becomes rubble (fallen debris)
                    else if rng.gen_bool(0.05) {
                        tiles[y][x] = Tile::Rubble;
                    }
                }
                _ => {}
            }
        }
    }

    // Create a few large open-sky patches (collapsed sections)
    let sky_patches = rng.gen_range(1..=3);
    for _ in 0..sky_patches {
        let cx = rng.gen_range(3..FLOOR_WIDTH - 3);
        let cy = rng.gen_range(2..FLOOR_HEIGHT - 2);
        let radius = rng.gen_range(2..=4);
        for dy in cy.saturating_sub(radius)..=(cy + radius).min(FLOOR_HEIGHT - 1) {
            for dx in cx.saturating_sub(radius)..=(cx + radius).min(FLOOR_WIDTH - 1) {
                let dist = ((dx as i32 - cx as i32).pow(2) + (dy as i32 - cy as i32).pow(2)) as f64;
                if dist.sqrt() <= radius as f64 + 0.5 {
                    if tiles[dy][dx] == Tile::Wall || tiles[dy][dx] == Tile::Floor {
                        tiles[dy][dx] = if rng.gen_bool(0.3) { Tile::Rubble } else { Tile::OpenSky };
                    }
                }
            }
        }
    }

    Floor { depth, tiles, rooms }
}

/// SHRINE: Symmetrical layout with central focal point, ceremonial corridors.
fn generate_shrine_floor(depth: usize, is_last: bool, rng: &mut StdRng) -> Floor {
    let mut tiles = vec![vec![Tile::Wall; FLOOR_WIDTH]; FLOOR_HEIGHT];
    let mut rooms = Vec::new();

    let cx = FLOOR_WIDTH / 2;
    let cy = FLOOR_HEIGHT / 2;

    // Central chamber — large and prominent
    let cw = rng.gen_range(6..=10);
    let ch = rng.gen_range(4..=6);
    let crx = cx - cw / 2;
    let cry = cy - ch / 2;
    for ry in cry..cry + ch {
        for rx in crx..crx + cw {
            if ry < FLOOR_HEIGHT && rx < FLOOR_WIDTH {
                tiles[ry][rx] = Tile::Floor;
            }
        }
    }
    // Place focal point at center
    tiles[cy][cx] = Tile::FocalPoint;
    rooms.push(Room { x: crx, y: cry, w: cw, h: ch, purpose: RoomPurpose::Ritual });

    // Symmetrical approach corridors from cardinal directions
    // North corridor
    for y in 1..cry {
        if cx < FLOOR_WIDTH { tiles[y][cx] = Tile::Floor; }
        if cx > 0 { tiles[y][cx - 1] = Tile::Floor; }
    }
    // South corridor
    for y in (cry + ch)..FLOOR_HEIGHT - 1 {
        if cx < FLOOR_WIDTH { tiles[y][cx] = Tile::Floor; }
        if cx > 0 { tiles[y][cx - 1] = Tile::Floor; }
    }
    // East corridor
    for x in (crx + cw)..FLOOR_WIDTH - 1 {
        if cy < FLOOR_HEIGHT { tiles[cy][x] = Tile::Floor; }
        if cy > 0 { tiles[cy - 1][x] = Tile::Floor; }
    }
    // West corridor
    for x in 1..crx {
        if cy < FLOOR_HEIGHT { tiles[cy][x] = Tile::Floor; }
        if cy > 0 { tiles[cy - 1][x] = Tile::Floor; }
    }

    // Small symmetrical side chambers (2-4)
    let side_rooms = rng.gen_range(2..=4);
    let offsets: Vec<(i32, i32)> = vec![(-1, -1), (1, -1), (-1, 1), (1, 1)];
    for i in 0..side_rooms {
        let (dx, dy) = offsets[i % offsets.len()];
        let sx = (cx as i32 + dx * rng.gen_range(6..=10) as i32).clamp(2, FLOOR_WIDTH as i32 - 6) as usize;
        let sy = (cy as i32 + dy * rng.gen_range(4..=6) as i32).clamp(2, FLOOR_HEIGHT as i32 - 5) as usize;
        let sw = rng.gen_range(3..=5);
        let sh = rng.gen_range(2..=4);

        let overlaps = rooms.iter().any(|r: &Room| {
            sx < r.x + r.w + 1 && sx + sw + 1 > r.x && sy < r.y + r.h + 1 && sy + sh + 1 > r.y
        });
        if overlaps { continue; }

        for ry in sy..sy + sh {
            for rx in sx..sx + sw {
                if ry < FLOOR_HEIGHT && rx < FLOOR_WIDTH {
                    tiles[ry][rx] = Tile::Floor;
                }
            }
        }
        // Connect to central chamber
        let (scx, scy) = (sx + sw / 2, sy + sh / 2);
        carve_corridor(&mut tiles, scx, scy, cx, cy, rng);

        let purpose = match rng.gen_range(0..3) {
            0 => RoomPurpose::Storage,
            1 => RoomPurpose::Ritual,
            _ => RoomPurpose::Trophy,
        };
        rooms.push(Room { x: sx, y: sy, w: sw, h: sh, purpose });
    }

    place_doors(&mut tiles, &rooms, rng);
    place_stairs(&mut tiles, &rooms, depth, is_last);

    Floor { depth, tiles, rooms }
}

/// BUREAUCRATIC ANNEX: Grid-like rooms, regular corridors, office labels.
fn generate_annex_floor(depth: usize, is_last: bool, rng: &mut StdRng) -> Floor {
    let mut tiles = vec![vec![Tile::Wall; FLOOR_WIDTH]; FLOOR_HEIGHT];
    let mut rooms = Vec::new();

    let purposes = [
        RoomPurpose::FilingRoom,
        RoomPurpose::WaitingArea,
        RoomPurpose::ProcessingDesk,
        RoomPurpose::ArchiveVault,
        RoomPurpose::Administrative,
        RoomPurpose::FilingRoom,
        RoomPurpose::WaitingArea,
        RoomPurpose::ProcessingDesk,
    ];

    // Grid layout: 2-3 rows, 3-4 columns of uniform rooms
    let cols = rng.gen_range(3..=4);
    let rows = rng.gen_range(2..=3);
    let room_w = (FLOOR_WIDTH - 2) / cols - 1;
    let room_h = (FLOOR_HEIGHT - 2) / rows - 1;
    let room_w = room_w.max(4);
    let room_h = room_h.max(3);

    let mut room_idx = 0;
    for row in 0..rows {
        for col in 0..cols {
            let x = 2 + col * (room_w + 1);
            let y = 2 + row * (room_h + 1);
            if x + room_w >= FLOOR_WIDTH - 1 || y + room_h >= FLOOR_HEIGHT - 1 {
                continue;
            }

            for ry in y..y + room_h {
                for rx in x..x + room_w {
                    tiles[ry][rx] = Tile::Floor;
                }
            }

            let purpose = purposes[room_idx % purposes.len()].clone();
            rooms.push(Room { x, y, w: room_w, h: room_h, purpose });
            room_idx += 1;
        }
    }

    // Central corridor running horizontally through the middle
    let corridor_y = FLOOR_HEIGHT / 2;
    for x in 1..FLOOR_WIDTH - 1 {
        tiles[corridor_y][x] = Tile::Floor;
    }
    // Vertical corridor down the center
    let corridor_x = FLOOR_WIDTH / 2;
    for y in 1..FLOOR_HEIGHT - 1 {
        tiles[y][corridor_x] = Tile::Floor;
    }

    // Connect rooms to corridors
    for room in &rooms {
        let (rcx, rcy) = room.center();
        // Connect to nearest corridor axis
        carve_h(&mut tiles, rcx, corridor_x, rcy);
        carve_v(&mut tiles, rcy, corridor_y, rcx);
    }

    place_doors(&mut tiles, &rooms, rng);
    place_stairs(&mut tiles, &rooms, depth, is_last);

    Floor { depth, tiles, rooms }
}

/// CONTROVERSIAL TOMBSITE: Central tomb, surrounding burial niches, solemn layout.
fn generate_tombsite_floor(depth: usize, is_last: bool, rng: &mut StdRng) -> Floor {
    let mut tiles = vec![vec![Tile::Wall; FLOOR_WIDTH]; FLOOR_HEIGHT];
    let mut rooms = Vec::new();

    let cx = FLOOR_WIDTH / 2;
    let cy = FLOOR_HEIGHT / 2;

    // Central tomb chamber — large
    let tw = rng.gen_range(6..=10);
    let th = rng.gen_range(4..=6);
    let tx = cx - tw / 2;
    let ty = cy - th / 2;
    for ry in ty..ty + th {
        for rx in tx..tx + tw {
            if ry < FLOOR_HEIGHT && rx < FLOOR_WIDTH {
                tiles[ry][rx] = Tile::Floor;
            }
        }
    }
    // Central sarcophagus marker
    tiles[cy][cx] = Tile::FocalPoint;
    rooms.push(Room { x: tx, y: ty, w: tw, h: th, purpose: RoomPurpose::TombChamber });

    // Surrounding burial niches in a ring pattern
    let niche_count = rng.gen_range(4..=8);
    let niche_w = 3;
    let niche_h = 2;
    for i in 0..niche_count {
        let angle = (i as f64 / niche_count as f64) * std::f64::consts::TAU;
        let dist = rng.gen_range(7..=12) as f64;
        let nx = (cx as f64 + angle.cos() * dist).round() as usize;
        let ny = (cy as f64 + angle.sin() * dist * 0.6).round() as usize; // compressed vertically for terminal
        let nx = nx.clamp(2, FLOOR_WIDTH - niche_w - 1);
        let ny = ny.clamp(2, FLOOR_HEIGHT - niche_h - 1);

        let overlaps = rooms.iter().any(|r: &Room| {
            nx < r.x + r.w + 1 && nx + niche_w + 1 > r.x
                && ny < r.y + r.h + 1 && ny + niche_h + 1 > r.y
        });
        if overlaps { continue; }

        for ry in ny..ny + niche_h {
            for rx in nx..nx + niche_w {
                if ry < FLOOR_HEIGHT && rx < FLOOR_WIDTH {
                    tiles[ry][rx] = Tile::Floor;
                }
            }
        }
        // Place a niche marker in each small chamber
        tiles[ny][nx + niche_w / 2] = Tile::Niche;

        rooms.push(Room { x: nx, y: ny, w: niche_w, h: niche_h, purpose: RoomPurpose::BurialNiche });

        // Connect to central tomb
        carve_corridor(&mut tiles, nx + niche_w / 2, ny + niche_h / 2, cx, cy, rng);
    }

    // Mourning hall at entrance (top)
    let mw = rng.gen_range(6..=8);
    let mh = 3;
    let mx = cx - mw / 2;
    let my = 1;
    for ry in my..my + mh {
        for rx in mx..mx + mw {
            if ry < FLOOR_HEIGHT && rx < FLOOR_WIDTH {
                tiles[ry][rx] = Tile::Floor;
            }
        }
    }
    rooms.push(Room { x: mx, y: my, w: mw, h: mh, purpose: RoomPurpose::MourningHall });
    carve_corridor(&mut tiles, mx + mw / 2, my + mh / 2, cx, cy, rng);

    place_doors(&mut tiles, &rooms, rng);
    place_stairs(&mut tiles, &rooms, depth, is_last);

    Floor { depth, tiles, rooms }
}

/// TAXONOMICALLY AMBIGUOUS REGION: Organic, irregular shapes, no straight walls.
fn generate_ambiguous_floor(depth: usize, is_last: bool, rng: &mut StdRng) -> Floor {
    // Start with organic walls everywhere, carve irregular caverns
    let mut tiles = vec![vec![Tile::OrganicWall; FLOOR_WIDTH]; FLOOR_HEIGHT];
    let mut rooms = Vec::new();

    // Use cellular automata to create organic cave shapes
    // Start with random fill
    let mut cave = vec![vec![false; FLOOR_WIDTH]; FLOOR_HEIGHT];
    for y in 1..FLOOR_HEIGHT - 1 {
        for x in 1..FLOOR_WIDTH - 1 {
            cave[y][x] = rng.gen_bool(0.45);
        }
    }

    // Run cellular automata smoothing (4 iterations)
    for _ in 0..4 {
        let mut next = vec![vec![false; FLOOR_WIDTH]; FLOOR_HEIGHT];
        for y in 1..FLOOR_HEIGHT - 1 {
            for x in 1..FLOOR_WIDTH - 1 {
                let mut neighbors = 0;
                for dy in -1i32..=1 {
                    for dx in -1i32..=1 {
                        if dy == 0 && dx == 0 { continue; }
                        let ny = (y as i32 + dy) as usize;
                        let nx = (x as i32 + dx) as usize;
                        if cave[ny][nx] { neighbors += 1; }
                    }
                }
                // B5678/S45678 rule — creates organic caves
                next[y][x] = if cave[y][x] {
                    neighbors >= 4
                } else {
                    neighbors >= 5
                };
            }
        }
        cave = next;
    }

    // Apply cave to tiles
    for y in 0..FLOOR_HEIGHT {
        for x in 0..FLOOR_WIDTH {
            if cave[y][x] {
                // Mix of different floor types for ambiguity
                tiles[y][x] = match rng.gen_range(0..10) {
                    0 => Tile::Water,
                    1 => Tile::Ground,
                    _ => Tile::Floor,
                };
            }
        }
    }

    // Find connected open regions and label them as "rooms"
    let cx = FLOOR_WIDTH / 2;
    let cy = FLOOR_HEIGHT / 2;
    // Place a few marker rooms at random open spots
    let room_count = rng.gen_range(3..=5);
    for i in 0..room_count {
        let mut placed = false;
        for _ in 0..100 {
            let rx = rng.gen_range(3..FLOOR_WIDTH - 3);
            let ry = rng.gen_range(2..FLOOR_HEIGHT - 2);
            if tiles[ry][rx] == Tile::Floor {
                let purpose = match rng.gen_range(0..3) {
                    0 => RoomPurpose::Disputed,
                    1 => RoomPurpose::Trophy,
                    _ => RoomPurpose::Habitation,
                };
                rooms.push(Room { x: rx, y: ry, w: 3, h: 2, purpose });
                placed = true;
                break;
            }
        }
        if !placed && i == 0 {
            // Ensure at least one room exists by carving at center
            for ry in cy.saturating_sub(1)..=(cy + 1).min(FLOOR_HEIGHT - 1) {
                for rx in cx.saturating_sub(2)..=(cx + 2).min(FLOOR_WIDTH - 1) {
                    tiles[ry][rx] = Tile::Floor;
                }
            }
            rooms.push(Room { x: cx - 2, y: cy - 1, w: 5, h: 3, purpose: RoomPurpose::Disputed });
        }
    }

    // Scatter some anomalous tiles — water, pits, focal points
    for _ in 0..rng.gen_range(3..=8) {
        let rx = rng.gen_range(1..FLOOR_WIDTH - 1);
        let ry = rng.gen_range(1..FLOOR_HEIGHT - 1);
        if tiles[ry][rx] == Tile::Floor {
            tiles[ry][rx] = match rng.gen_range(0..4) {
                0 => Tile::Water,
                1 => Tile::Pit,
                2 => Tile::FocalPoint,
                _ => Tile::Niche,
            };
        }
    }

    place_stairs(&mut tiles, &rooms, depth, is_last);

    Floor { depth, tiles, rooms }
}

/// ABANDONED INSTITUTION: Office grid like Annex but deteriorating.
fn generate_abandoned_floor(depth: usize, is_last: bool, rng: &mut StdRng) -> Floor {
    // Generate a clean annex floor first, then damage it
    let mut floor = generate_annex_floor(depth, is_last, rng);

    // Replace room purposes with abandoned variants
    for room in &mut floor.rooms {
        room.purpose = match rng.gen_range(0..5) {
            0 => RoomPurpose::FormerOffice,
            1 => RoomPurpose::CollapsedWing,
            2 => RoomPurpose::ArchiveVault,
            3 => RoomPurpose::FilingRoom,
            _ => RoomPurpose::FormerOffice,
        };
    }

    // Damage pass: break walls, scatter debris
    for y in 0..FLOOR_HEIGHT {
        for x in 0..FLOOR_WIDTH {
            match floor.tiles[y][x] {
                Tile::Wall => {
                    // ~20% of walls break
                    if rng.gen_bool(0.20) {
                        floor.tiles[y][x] = if rng.gen_bool(0.6) { Tile::Rubble } else { Tile::Floor };
                    }
                }
                Tile::Floor => {
                    // ~8% of floor gets debris
                    if rng.gen_bool(0.08) {
                        floor.tiles[y][x] = Tile::Rubble;
                    }
                }
                Tile::Door => {
                    // ~30% of doors are broken
                    if rng.gen_bool(0.30) {
                        floor.tiles[y][x] = Tile::Floor;
                    }
                }
                _ => {}
            }
        }
    }

    // Block off 1-2 rooms entirely (inaccessible — fill with wall/rubble)
    let block_count = rng.gen_range(0..=2).min(floor.rooms.len().saturating_sub(2));
    for _ in 0..block_count {
        let room_idx = rng.gen_range(0..floor.rooms.len());
        let room = &floor.rooms[room_idx];
        for ry in room.y..room.y + room.h {
            for rx in room.x..room.x + room.w {
                if ry < FLOOR_HEIGHT && rx < FLOOR_WIDTH {
                    floor.tiles[ry][rx] = if rng.gen_bool(0.4) { Tile::Rubble } else { Tile::Wall };
                }
            }
        }
    }

    floor
}

/// Place stairs on a floor: up on non-first floors, down on non-last floors.
fn place_stairs(tiles: &mut Vec<Vec<Tile>>, rooms: &[Room], depth: usize, is_last: bool) {
    if depth > 0 {
        if let Some(room) = rooms.first() {
            let (cx, cy) = room.center();
            if cy < FLOOR_HEIGHT && cx < FLOOR_WIDTH {
                tiles[cy][cx] = Tile::StairUp;
            }
        }
    }
    if !is_last {
        if let Some(room) = rooms.last() {
            let (cx, cy) = room.center();
            if cy < FLOOR_HEIGHT && cx < FLOOR_WIDTH {
                tiles[cy][cx] = Tile::StairDown;
            }
        }
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
/// buildings on open ground — bird's-eye village layout, not a dungeon.
pub fn generate_settlement_floor(size: &SettlementSize, rng: &mut StdRng) -> Floor {
    // Start with open ground everywhere
    let mut tiles = vec![vec![Tile::Ground; FLOOR_WIDTH]; FLOOR_HEIGHT];
    let mut rooms = Vec::new();

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

    // Minimum interior width must fit the longest building label without truncation.
    let label_min_w = civic_purposes.iter()
        .map(|p| p.short_label().len())
        .max()
        .unwrap_or(4);

    // Building count and interior size ranges vary by settlement size
    let (target_rooms, min_w, max_w, min_h, max_h) = match size {
        SettlementSize::Hamlet => (rng.gen_range(3..=5), label_min_w, 8, 3, 5),
        SettlementSize::Town => (rng.gen_range(6..=9), label_min_w, 9, 3, 6),
        SettlementSize::City => (rng.gen_range(10..=14), label_min_w, 10, 3, 6),
    };

    let mut attempts = 0;
    while rooms.len() < target_rooms && attempts < 400 {
        attempts += 1;

        let w = rng.gen_range(min_w..=max_w); // interior width
        let h = rng.gen_range(min_h..=max_h); // interior height
        // Leave margin for walls (1 tile each side) plus grid border
        let x = rng.gen_range(2..FLOOR_WIDTH.saturating_sub(w + 2));
        let y = rng.gen_range(2..FLOOR_HEIGHT.saturating_sub(h + 2));

        // Check overlap: each building occupies interior + 1-tile wall border.
        // Require 1 additional tile gap between buildings for open ground.
        let overlaps = rooms.iter().any(|r: &Room| {
            x < r.x + r.w + 3 && x + w + 3 > r.x
                && y < r.y + r.h + 3 && y + h + 3 > r.y
        });
        if overlaps {
            continue;
        }

        let purpose = civic_purposes[rooms.len() % civic_purposes.len()].clone();

        // Draw building walls (border around the interior)
        for rx in x.saturating_sub(1)..=(x + w).min(FLOOR_WIDTH - 1) {
            if y > 0 { tiles[y - 1][rx] = Tile::Wall; }
            if y + h < FLOOR_HEIGHT { tiles[y + h][rx] = Tile::Wall; }
        }
        for ry in y.saturating_sub(1)..=(y + h).min(FLOOR_HEIGHT - 1) {
            if x > 0 { tiles[ry][x - 1] = Tile::Wall; }
            if x + w < FLOOR_WIDTH { tiles[ry][x + w] = Tile::Wall; }
        }

        // Fill interior with floor
        for ry in y..y + h {
            for rx in x..x + w {
                tiles[ry][rx] = Tile::Floor;
            }
        }

        // Place 1-2 doors on building walls
        let door_count = if w + h > 10 { 2 } else { 1 };
        for _ in 0..door_count {
            match rng.gen_range(0u8..4) {
                0 if y > 0 => {
                    let dx = rng.gen_range(x..x + w);
                    tiles[y - 1][dx] = Tile::Door;
                }
                1 if y + h < FLOOR_HEIGHT => {
                    let dx = rng.gen_range(x..x + w);
                    tiles[y + h][dx] = Tile::Door;
                }
                2 if x > 0 => {
                    let dy = rng.gen_range(y..y + h);
                    tiles[dy][x - 1] = Tile::Door;
                }
                _ if x + w < FLOOR_WIDTH => {
                    let dy = rng.gen_range(y..y + h);
                    tiles[dy][x + w] = Tile::Door;
                }
                _ => {}
            }
        }

        rooms.push(Room { x, y, w, h, purpose });
    }

    // Add a well or fountain in larger settlements
    if matches!(size, SettlementSize::Town | SettlementSize::City) && rooms.len() >= 2 {
        let cx = FLOOR_WIDTH / 2;
        let cy = FLOOR_HEIGHT / 2;
        for r in 0..8 {
            let mut placed = false;
            for dy in (cy.saturating_sub(r))..=(cy + r).min(FLOOR_HEIGHT - 1) {
                for dx in (cx.saturating_sub(r))..=(cx + r).min(FLOOR_WIDTH - 1) {
                    if tiles[dy][dx] == Tile::Ground {
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
