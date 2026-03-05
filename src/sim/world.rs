use ratatui::style::Color;
use serde::{Deserialize, Serialize};

/// Width and height of the world map grid.
pub const MAP_WIDTH: usize = 60;
pub const MAP_HEIGHT: usize = 30;

/// Terrain types for each map tile.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Terrain {
    DeepWater,
    ShallowWater,
    Plains,
    Hills,
    Forest,
    Mountains,
    Desert,
}

impl Terrain {
    /// ASCII character for this terrain type.
    pub fn glyph(self) -> char {
        match self {
            Terrain::DeepWater => '~',
            Terrain::ShallowWater => ':',
            Terrain::Plains => '.',
            Terrain::Hills => '^',
            Terrain::Forest => 'T',
            Terrain::Mountains => 'M',
            Terrain::Desert => 's',
        }
    }

    /// Display color for this terrain type.
    pub fn color(self) -> Color {
        match self {
            Terrain::DeepWater => Color::Blue,
            Terrain::ShallowWater => Color::Cyan,
            Terrain::Plains => Color::Green,
            Terrain::Hills => Color::Yellow,
            Terrain::Forest => Color::Rgb(0, 140, 0), // dark green
            Terrain::Mountains => Color::Gray,
            Terrain::Desert => Color::Rgb(210, 180, 60), // sandy yellow
        }
    }

    /// Whether settlements can be placed on this terrain.
    pub fn is_habitable(self) -> bool {
        matches!(self, Terrain::Plains | Terrain::Hills | Terrain::Forest)
    }
}

/// Size class of a settlement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SettlementSize {
    Hamlet,
    Town,
    City,
}

/// A settlement on the world map.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settlement {
    pub name: String,
    pub size: SettlementSize,
    pub x: usize,
    pub y: usize,
}

/// A people/culture inhabiting the world.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct People {
    pub name: String,
    pub preferred_terrain: Vec<Terrain>,
    pub population: u32,
}

/// The complete world state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World {
    pub seed: u64,
    pub name: String,
    pub terrain: Vec<Vec<Terrain>>,
    pub settlements: Vec<Settlement>,
    pub peoples: Vec<People>,
    pub tick: u64,
}

impl World {
    /// Produce the rendered map as (char, Color) pairs for display.
    pub fn render_map(&self) -> Vec<Vec<(char, Color)>> {
        let mut map = Vec::with_capacity(MAP_HEIGHT);
        for y in 0..MAP_HEIGHT {
            let mut row = Vec::with_capacity(MAP_WIDTH);
            for x in 0..MAP_WIDTH {
                let t = self.terrain[y][x];
                row.push((t.glyph(), t.color()));
            }
            map.push(row);
        }

        // Overlay settlements — marked with a bright symbol
        for s in &self.settlements {
            if s.y < MAP_HEIGHT && s.x < MAP_WIDTH {
                let glyph = match s.size {
                    SettlementSize::Hamlet => 'o',
                    SettlementSize::Town => 'O',
                    SettlementSize::City => '#',
                };
                map[s.y][s.x] = (glyph, Color::White);
            }
        }

        map
    }
}
