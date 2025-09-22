use matcat::{generate_props_from_category, get_category_ranges};
use rand::thread_rng;

#[test]
fn generate_samples_for_all_categories() {
    let mut rng = thread_rng();

    for cat in 0x01..=0x14 { // loop over the 20 categories we defined
        if let Some(range) = get_category_ranges(cat) {
            println!("\n=== Category 0x{:02X} ===", cat);
            let props = generate_props_from_category(cat, &mut rng).unwrap();

            println!(
                "Density: {:.1} kg/m³ | E: {:.2e} Pa | Hardness: {:.1} | Fracture Toughness: {:.1} MPa·m^0.5",
                props.density, props.elastic_modulus, props.hardness, props.fracture_toughness
            );
            println!(
                "Thermal: k = {:.2} W/mK, α = {:.2e} 1/K, Tm = {:.1} °C",
                props.thermal_conductivity, props.thermal_expansion, props.melting_point
            );
        } else {
            println!("Category 0x{:02X} not implemented", cat);
        }
    }
}
