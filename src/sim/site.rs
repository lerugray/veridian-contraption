use serde::{Deserialize, Serialize};

/// Width and height of a site floor grid.
pub const FLOOR_WIDTH: usize = 40;
pub const FLOOR_HEIGHT: usize = 20;

/// Types of sites that can exist in the world.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SiteKind {
    Dungeon,
    Ruin,
    Shrine,
    BureaucraticAnnex,
    ControversialTombsite,
    TaxonomicallyAmbiguousRegion,
    AbandonedInstitution,
}

impl SiteKind {
    pub fn label(&self) -> &'static str {
        match self {
            SiteKind::Dungeon => "Dungeon",
            SiteKind::Ruin => "Ruin",
            SiteKind::Shrine => "Shrine",
            SiteKind::BureaucraticAnnex => "Bureaucratic Annex",
            SiteKind::ControversialTombsite => "Controversial Tombsite",
            SiteKind::TaxonomicallyAmbiguousRegion => "Taxonomically Ambiguous Region",
            SiteKind::AbandonedInstitution => "Abandoned Institution",
        }
    }

    /// Color used to render this site kind on the world map.
    pub fn map_color(&self) -> ratatui::style::Color {
        use ratatui::style::Color;
        match self {
            SiteKind::Dungeon => Color::Red,
            SiteKind::Ruin => Color::Rgb(180, 140, 80),
            SiteKind::Shrine => Color::LightMagenta,
            SiteKind::BureaucraticAnnex => Color::LightCyan,
            SiteKind::ControversialTombsite => Color::Rgb(160, 100, 160),
            SiteKind::TaxonomicallyAmbiguousRegion => Color::LightGreen,
            SiteKind::AbandonedInstitution => Color::DarkGray,
        }
    }
}

/// A tile in a site floor.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Tile {
    Floor,
    Wall,
    Door,
    StairDown,
    StairUp,
    Water,
    Pit,
    /// Outdoor open space (settlements).
    Ground,
    /// Collapsed wall / debris (ruins, abandoned institutions).
    Rubble,
    /// Open sky where roof has collapsed (ruins).
    OpenSky,
    /// Shrine focal point / altar.
    FocalPoint,
    /// Burial niche (tombsites).
    Niche,
    /// Organic / irregular boundary (taxonomically ambiguous regions).
    OrganicWall,
}

impl Tile {
    pub fn glyph(self) -> char {
        match self {
            Tile::Floor => '.',
            Tile::Wall => '#',
            Tile::Door => '+',
            Tile::StairDown => '>',
            Tile::StairUp => '<',
            Tile::Water => '~',
            Tile::Pit => ' ',
            Tile::Ground => '\u{00B7}', // middle dot ·
            Tile::Rubble => '%',
            Tile::OpenSky => '\u{00B7}', // middle dot, colored differently
            Tile::FocalPoint => '*',
            Tile::Niche => '\u{00B0}', // degree sign °
            Tile::OrganicWall => '\u{2593}', // dark shade ▓
        }
    }

    pub fn color(self) -> ratatui::style::Color {
        use ratatui::style::Color;
        match self {
            Tile::Floor => Color::Rgb(70, 65, 60),
            Tile::Wall => Color::Rgb(130, 120, 110),
            Tile::Door => Color::Rgb(200, 170, 80),
            Tile::StairDown | Tile::StairUp => Color::Rgb(100, 200, 220),
            Tile::Water => Color::Rgb(40, 90, 160),
            Tile::Pit => Color::Rgb(30, 25, 20),
            Tile::Ground => Color::Rgb(55, 80, 45),
            Tile::Rubble => Color::Rgb(140, 110, 70),
            Tile::OpenSky => Color::Rgb(80, 100, 130),
            Tile::FocalPoint => Color::Rgb(240, 200, 80),
            Tile::Niche => Color::Rgb(120, 115, 110),
            Tile::OrganicWall => Color::Rgb(90, 120, 80),
        }
    }

    pub fn walkable(self) -> bool {
        matches!(self, Tile::Floor | Tile::Door | Tile::StairDown | Tile::StairUp
            | Tile::Ground | Tile::OpenSky | Tile::Rubble)
    }
}

/// The purpose of a room within a site floor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoomPurpose {
    Storage,
    Ritual,
    Administrative,
    Habitation,
    Trophy,
    Disputed,
    // Settlement-specific civic purposes
    Tavern,
    Market,
    Temple,
    Residential,
    Warehouse,
    Garrison,
    // Bureaucratic Annex purposes
    FilingRoom,
    WaitingArea,
    ProcessingDesk,
    ArchiveVault,
    // Tombsite purposes
    TombChamber,
    BurialNiche,
    MourningHall,
    // Abandoned Institution purposes
    FormerOffice,
    CollapsedWing,
}

impl RoomPurpose {
    pub fn label(&self) -> &'static str {
        match self {
            RoomPurpose::Storage => "Storage",
            RoomPurpose::Ritual => "Ritual",
            RoomPurpose::Administrative => "Administrative",
            RoomPurpose::Habitation => "Habitation",
            RoomPurpose::Trophy => "Trophy",
            RoomPurpose::Disputed => "Disputed",
            RoomPurpose::Tavern => "Tavern",
            RoomPurpose::Market => "Market",
            RoomPurpose::Temple => "Temple",
            RoomPurpose::Residential => "Residential",
            RoomPurpose::Warehouse => "Warehouse",
            RoomPurpose::Garrison => "Garrison",
            RoomPurpose::FilingRoom => "Filing Room",
            RoomPurpose::WaitingArea => "Waiting Area",
            RoomPurpose::ProcessingDesk => "Processing Desk",
            RoomPurpose::ArchiveVault => "Archive Vault",
            RoomPurpose::TombChamber => "Tomb Chamber",
            RoomPurpose::BurialNiche => "Burial Niche",
            RoomPurpose::MourningHall => "Mourning Hall",
            RoomPurpose::FormerOffice => "Former Office",
            RoomPurpose::CollapsedWing => "Collapsed Wing",
        }
    }

    /// Short label for rendering inside floor plan buildings.
    pub fn short_label(&self) -> &'static str {
        match self {
            RoomPurpose::Storage => "Store",
            RoomPurpose::Ritual => "Ritual",
            RoomPurpose::Administrative => "Admin",
            RoomPurpose::Habitation => "Home",
            RoomPurpose::Trophy => "Trophy",
            RoomPurpose::Disputed => "???",
            RoomPurpose::Tavern => "Tavern",
            RoomPurpose::Market => "Market",
            RoomPurpose::Temple => "Temple",
            RoomPurpose::Residential => "Home",
            RoomPurpose::Warehouse => "Wares",
            RoomPurpose::Garrison => "Guard",
            RoomPurpose::FilingRoom => "Filing",
            RoomPurpose::WaitingArea => "Wait",
            RoomPurpose::ProcessingDesk => "Desk",
            RoomPurpose::ArchiveVault => "Archive",
            RoomPurpose::TombChamber => "Tomb",
            RoomPurpose::BurialNiche => "Niche",
            RoomPurpose::MourningHall => "Mourn",
            RoomPurpose::FormerOffice => "Office",
            RoomPurpose::CollapsedWing => "Ruins",
        }
    }
}

/// A room on a floor, defined by its bounding rectangle and purpose.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
    pub purpose: RoomPurpose,
}

impl Room {
    pub fn center(&self) -> (usize, usize) {
        (self.x + self.w / 2, self.y + self.h / 2)
    }
}

/// A single floor within a site.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Floor {
    pub depth: usize,
    pub tiles: Vec<Vec<Tile>>,
    pub rooms: Vec<Room>,
}

/// A permanent inhabitant of a site — distinct from visiting adventurer agents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteInhabitant {
    /// Unique ID within this site (0-based).
    pub id: usize,
    /// Name of the inhabitant.
    pub name: String,
    /// One-line description in the game's register.
    pub description: String,
    /// Display glyph (lowercase letter by type).
    pub glyph: char,
    /// Which floor this inhabitant resides on.
    pub floor: usize,
    /// Position within the floor (x, y).
    pub x: usize,
    pub y: usize,
}

/// A site of interest on the world map.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Site {
    pub id: u64,
    pub name: String,
    pub kind: SiteKind,
    /// Generated explanation of why this site exists.
    pub origin: String,
    pub grid_x: u32,
    pub grid_y: u32,
    pub floors: Vec<Floor>,
    /// Agent IDs currently in this site.
    pub population: Vec<u64>,
    /// Artifact IDs present at this site.
    pub artifacts: Vec<u64>,
    /// Chronicle of notable events at this site.
    pub history: Vec<String>,
    /// Institution ID that controls this site, if any.
    pub controlling_faction: Option<u64>,
    /// Permanent inhabitants of this site (do not leave).
    #[serde(default)]
    pub inhabitants: Vec<SiteInhabitant>,
}
