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
        }
    }

    pub fn color(self) -> ratatui::style::Color {
        use ratatui::style::Color;
        match self {
            Tile::Floor => Color::Gray,
            Tile::Wall => Color::White,
            Tile::Door => Color::Yellow,
            Tile::StairDown | Tile::StairUp => Color::LightCyan,
            Tile::Water => Color::Blue,
            Tile::Pit => Color::DarkGray,
        }
    }

    pub fn walkable(self) -> bool {
        matches!(self, Tile::Floor | Tile::Door | Tile::StairDown | Tile::StairUp)
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
}
