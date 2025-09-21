use matcat::{MatCatId, props_for};

fn main() {
    // Example: Metal → Copper
    let copper = MatCatId::new(1, 42, 0);
    let c_props = props_for(&copper);

    println!("Copper Properties:");
    println!("  Density: {} kg/m³", c_props.density);
    println!("  Elastic Modulus: {} GPa", c_props.elastic_modulus);
    println!("  Tensile Strength: {} MPa", c_props.tensile_strength);
    println!("  Compressive Strength: {} MPa", c_props.compressive_strength);
    println!("  Hardness: {}", c_props.hardness);
    println!("  Fracture Toughness: {} MPa·m^0.5", c_props.fracture_toughness);
    println!("  Fatigue Resistance: {}", c_props.fatigue_resistance);

    println!("  Thermal Conductivity: {} W/m·K", c_props.thermal_conductivity);
    println!("  Thermal Expansion: {} 1/K", c_props.thermal_expansion);
    println!("  Melting Point: {} °C", c_props.melting_point);

    println!("  Corrosion Resistance: {}", c_props.corrosion_resistance);
    println!("  Solubility: {}", c_props.solubility);
    println!("  Permeability: {}", c_props.permeability);
    println!("  Flammability: {}", c_props.flammability);

    println!("  Electrical Conductivity: {}", c_props.electrical_conductivity);
    println!("  Magnetic Permeability: {}", c_props.magnetic_permeability);
}
