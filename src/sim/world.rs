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

    /// Display color for this terrain type (truecolor for expressive maps).
    pub fn color(self) -> Color {
        match self {
            Terrain::DeepWater => Color::Rgb(20, 60, 140),     // deep navy
            Terrain::ShallowWater => Color::Rgb(60, 130, 190), // coastal blue
            Terrain::Plains => Color::Rgb(90, 160, 60),        // living green
            Terrain::Hills => Color::Rgb(170, 150, 80),        // tawny ochre
            Terrain::Forest => Color::Rgb(30, 110, 40),        // dark canopy
            Terrain::Mountains => Color::Rgb(140, 140, 155),   // slate grey
            Terrain::Desert => Color::Rgb(210, 180, 60),       // sandy yellow
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
    /// Index into the phoneme data — determines naming conventions for this culture.
    #[serde(default)]
    pub phoneme_set: usize,
}

/// The narrative register governing prose style for this world.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum NarrativeRegister {
    Clinical,
    Lyrical,
    Bureaucratic,
    Ominous,
    Conspiratorial,
}

impl NarrativeRegister {
    pub fn label(self) -> &'static str {
        match self {
            NarrativeRegister::Clinical => "Clinical",
            NarrativeRegister::Lyrical => "Lyrical",
            NarrativeRegister::Bureaucratic => "Bureaucratic",
            NarrativeRegister::Ominous => "Ominous",
            NarrativeRegister::Conspiratorial => "Conspiratorial",
        }
    }
}

impl Default for NarrativeRegister {
    fn default() -> Self { NarrativeRegister::Bureaucratic }
}

/// World-level simulation parameters — each world generates its own ruleset.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldParams {
    /// Multiplier on how fast events fire per tick (0.1–3.0).
    pub temporal_rate: f32,
    /// Rate of institutional change (0.0–1.0).
    pub political_churn: f32,
    /// Frequency of metaphysical/cosmological events (0.0–1.0).
    pub cosmological_density: f32,
    /// Rate of environmental events (0.0–1.0).
    pub ecological_volatility: f32,
    /// Prose style the world's log tends toward.
    pub narrative_register: NarrativeRegister,
    /// Global absurdity dial (0.0–1.0). Affects naming, events, prose.
    pub weirdness_coefficient: f32,
}

impl Default for WorldParams {
    fn default() -> Self {
        WorldParams {
            temporal_rate: 1.0,
            political_churn: 0.5,
            cosmological_density: 0.3,
            ecological_volatility: 0.5,
            narrative_register: NarrativeRegister::Bureaucratic,
            weirdness_coefficient: 0.5,
        }
    }
}

impl WorldParams {
    /// Human-readable descriptor for a 0.0–1.0 parameter value.
    pub fn describe_level(value: f32) -> &'static str {
        if value < 0.15 { "Negligible" }
        else if value < 0.3 { "Low" }
        else if value < 0.5 { "Moderate" }
        else if value < 0.7 { "Elevated" }
        else if value < 0.85 { "High" }
        else { "Extreme" }
    }

    /// Human-readable descriptor for temporal_rate (0.1–3.0).
    pub fn describe_temporal_rate(&self) -> &'static str {
        if self.temporal_rate < 0.4 { "Geological" }
        else if self.temporal_rate < 0.8 { "Unhurried" }
        else if self.temporal_rate < 1.2 { "Standard" }
        else if self.temporal_rate < 2.0 { "Accelerated" }
        else { "Frenetic" }
    }
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
    #[serde(default)]
    pub params: WorldParams,
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

        // Overlay settlements — symbol scales with size, warm colors
        for s in &self.settlements {
            if s.y < MAP_HEIGHT && s.x < MAP_WIDTH {
                let (glyph, color) = match s.size {
                    SettlementSize::Hamlet => ('·', Color::Rgb(180, 170, 150)),   // dim stone
                    SettlementSize::Town =>   ('o', Color::Rgb(230, 210, 160)),   // warm lantern
                    SettlementSize::City =>   ('O', Color::Rgb(255, 240, 200)),   // bright hearth
                };
                map[s.y][s.x] = (glyph, color);
            }
        }

        map
    }
}
