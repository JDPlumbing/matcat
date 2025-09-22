/// 5-byte compact material identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MatCatId {
    pub category: u8,   // e.g., Metal = 1
    pub variant: u16,   // e.g., Copper = 42
    pub grade: u16,     // e.g., Type L = 999
}

impl MatCatId {
    pub fn new(category: u8, variant: u16, grade: u16) -> Self {
        Self { category, variant, grade }
    }

    /// Collapse into a single 64-bit seed for procedural generation
    pub fn seed(&self) -> u64 {
        ((self.category as u64) << 32) | ((self.variant as u64) << 16) | (self.grade as u64)
    }
}

/// Canonical material property set.
/// Every material is defined by these values, regardless of name.
#[derive(Debug, Clone, Copy)]
pub struct MatProps {
    // --- Mechanical ---
    pub density: f32,              // kg/m³
    pub elastic_modulus: f32,      // GPa
    pub tensile_strength: f32,     // MPa
    pub compressive_strength: f32, // MPa
    pub hardness: f32,             // Mohs-like 0–10
    pub fracture_toughness: f32,   // MPa·m^0.5
    pub fatigue_resistance: f32,   // 0.0–1.0

    // --- Thermal ---
    pub thermal_conductivity: f32, // W/m·K
    pub thermal_expansion: f32,    // 1/K
    pub melting_point: f32,        // °C

    // --- Chemical ---
    pub corrosion_resistance: f32, // 0.0–1.0
    pub solubility: f32,           // 0.0–1.0
    pub permeability: f32,         // 0.0–1.0
    pub flammability: f32,         // 0.0–1.0

    // --- Electrical / Magnetic ---
    pub electrical_conductivity: f32, // 0.0–1.0
    pub magnetic_permeability: f32,   // relative μ
}

/// Small deterministic PRNG for procedural props
fn lcg(mut seed: u64) -> impl FnMut() -> f32 {
    move || {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let bits = (seed >> 32) as u32;
        (bits as f32) / (u32::MAX as f32) // → [0.0, 1.0]
    }
}

/// Procedurally derive material properties from MatCatId
use crate::category_ranges::generate_props_from_category;

pub fn props_for(id: &MatCatId) -> MatProps {
    if let Some(props) = generate_props_from_category(id.category, &mut rand::rng()) {
        props
    } else {
        // fallback to old RNG if category is undefined
        let mut rng = lcg(id.seed());
        MatProps {
            density: 500.0 + rng() * 20000.0,
            elastic_modulus: rng() * 4e11,
            tensile_strength: rng() * 2000.0,
            compressive_strength: rng() * 4000.0,
            hardness: rng() * 10.0,
            fracture_toughness: rng() * 50.0,
            fatigue_resistance: rng(),

            thermal_conductivity: rng() * 400.0,
            thermal_expansion: rng() * 1e-4,
            melting_point: rng() * 4000.0,

            corrosion_resistance: rng(),
            solubility: rng(),
            permeability: rng(),
            flammability: rng(),

            electrical_conductivity: rng(),
            magnetic_permeability: rng() * 1000.0,
        }
    }
}


use std::f32;

/// Distance metric between two materials (Euclidean in property space).
fn distance(a: &MatProps, b: &MatProps) -> f32 {
    let diffs = [
        (a.density - b.density).powi(2),
        (a.elastic_modulus - b.elastic_modulus).powi(2),
        (a.tensile_strength - b.tensile_strength).powi(2),
        (a.compressive_strength - b.compressive_strength).powi(2),
        (a.hardness - b.hardness).powi(2),
        (a.fracture_toughness - b.fracture_toughness).powi(2),
        (a.fatigue_resistance - b.fatigue_resistance).powi(2),

        (a.thermal_conductivity - b.thermal_conductivity).powi(2),
        (a.thermal_expansion - b.thermal_expansion).powi(2),
        (a.melting_point - b.melting_point).powi(2),

        (a.corrosion_resistance - b.corrosion_resistance).powi(2),
        (a.solubility - b.solubility).powi(2),
        (a.permeability - b.permeability).powi(2),
        (a.flammability - b.flammability).powi(2),

        (a.electrical_conductivity - b.electrical_conductivity).powi(2),
        (a.magnetic_permeability - b.magnetic_permeability).powi(2),
    ];

    diffs.iter().sum::<f32>().sqrt()
}

/// Find the closest material ID to some target properties.
pub fn find_closest_material(target: &MatProps, search_space: &[MatCatId]) -> Option<(MatCatId, MatProps)> {
    let mut best_id = None;
    let mut best_props = None;
    let mut best_dist = f32::MAX;

    for id in search_space {
        let props = props_for(id);
        let d = distance(&props, target);
        if d < best_dist {
            best_dist = d;
            best_id = Some(*id);
            best_props = Some(props);
        }
    }

    best_id.zip(best_props)
}

