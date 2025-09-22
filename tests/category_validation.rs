use matcat::{get_category_ranges, generate_props_from_category, MatProps};
use rand::rng; // rand 0.9

fn assert_in_range(name: &str, value: f32, range: (f32, f32)) {
    assert!(
        value >= range.0 && value <= range.1,
        "{} = {} out of range [{}, {}]",
        name, value, range.0, range.1
    );
}

#[test]
fn validate_category_props() {
    let mut rng = rng();

    for cat in 0x01..=0x12 {
        if let Some(r) = get_category_ranges(cat) {
            let props: MatProps = generate_props_from_category(cat, &mut rng).unwrap();
            println!("\n=== Category 0x{:02X} ===", cat);

            // Mechanical
            assert_in_range("density", props.density, r.density);
            assert_in_range("elastic_modulus", props.elastic_modulus, r.elastic_modulus);
            assert_in_range("tensile_strength", props.tensile_strength, r.tensile_strength);
            assert_in_range("compressive_strength", props.compressive_strength, r.compressive_strength);
            assert_in_range("hardness", props.hardness, r.hardness);
            assert_in_range("fracture_toughness", props.fracture_toughness, r.fracture_toughness);
            assert_in_range("fatigue_resistance", props.fatigue_resistance, r.fatigue_resistance);

            // Thermal
            assert_in_range("thermal_conductivity", props.thermal_conductivity, r.thermal_conductivity);
            assert_in_range("thermal_expansion", props.thermal_expansion, r.thermal_expansion);
            assert_in_range("melting_point", props.melting_point, r.melting_point);

            // Chemical
            assert_in_range("corrosion_resistance", props.corrosion_resistance, r.corrosion_resistance);
            assert_in_range("solubility", props.solubility, r.solubility);
            assert_in_range("permeability", props.permeability, r.permeability);
            assert_in_range("flammability", props.flammability, r.flammability);

            // Electrical / Magnetic
            assert_in_range("electrical_conductivity", props.electrical_conductivity, r.electrical_conductivity);
            assert_in_range("magnetic_permeability", props.magnetic_permeability, r.magnetic_permeability);
        } else {
            panic!("Category 0x{:02X} missing from ranges", cat);
        }
    }
}
