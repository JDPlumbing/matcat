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
}

/// Abstracted material properties
#[derive(Debug, Clone, Copy)]
pub struct MatProps {
    pub density: f32,       // g/cm³
    pub conductivity: f32,  // electrical 0.0–1.0
    pub hardness: f32,      // Mohs-like 0.0–10.0
    pub flammability: f32,  // 0.0–1.0
    pub corrosion: f32,     // 0.0–1.0
    pub tensile: f32,       // MPa (approx)
}

/// Category-level defaults
fn default_props_for_category(cat: u8) -> MatProps {
    match cat {
        1 => MatProps { // Metal
            density: 7.5,
            conductivity: 0.7,
            hardness: 5.0,
            flammability: 0.0,
            corrosion: 0.5,
            tensile: 200.0,
        },
        2 => MatProps { // Plastic
            density: 1.2,
            conductivity: 0.0,
            hardness: 2.0,
            flammability: 0.8,
            corrosion: 0.0,
            tensile: 50.0,
        },
        3 => MatProps { // Wood
            density: 0.7,
            conductivity: 0.0,
            hardness: 3.0,
            flammability: 0.9,
            corrosion: 0.1,
            tensile: 40.0,
        },
        _ => MatProps { // Unknown fallback (water-ish baseline)
            density: 1.0,
            conductivity: 0.0,
            hardness: 1.0,
            flammability: 0.5,
            corrosion: 0.5,
            tensile: 10.0,
        },
    }
}

/// Variant-level overrides (examples)
fn override_variant_props(cat: u8, var: u16, mut props: MatProps) -> MatProps {
    match (cat, var) {
        (1, 42) => { // Metal → Copper
            props.density = 8.9;
            props.conductivity = 1.0;
            props.hardness = 3.0;
            props.corrosion = 0.6;
            props.tensile = 210.0;
        }
        (1, 43) => { // Metal → Iron
            props.density = 7.9;
            props.conductivity = 0.2;
            props.hardness = 4.0;
            props.corrosion = 0.8;
            props.tensile = 250.0;
        }
        (2, 10) => { // Plastic → PVC
            props.density = 1.4;
            props.hardness = 2.5;
            props.flammability = 0.4;
            props.tensile = 55.0;
        }
        _ => {}
    }
    props
}

/// Public API: get material properties
pub fn props_for(id: &MatCatId) -> MatProps {
    let defaults = default_props_for_category(id.category);
    override_variant_props(id.category, id.variant, defaults)
}
