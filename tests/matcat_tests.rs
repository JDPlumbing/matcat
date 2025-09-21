use matcat::{MatCatId, props_for};

#[test]
fn materials_are_deterministic() {
    let id = MatCatId::new(1, 42, 0);
    let props1 = props_for(&id);
    let props2 = props_for(&id);
    assert_eq!(props1.density, props2.density);
    assert_eq!(props1.tensile_strength, props2.tensile_strength);
}

#[test]
fn properties_within_reasonable_bounds() {
    let id = MatCatId::new(2, 10, 0);
    let props = props_for(&id);
    assert!(props.density >= 500.0 && props.density <= 20_500.0);
    assert!(props.hardness >= 0.0 && props.hardness <= 10.0);
    assert!(props.melting_point >= 0.0 && props.melting_point <= 4000.0);
}

#[test]
fn different_ids_produce_different_properties() {
    let a = MatCatId::new(1, 42, 0);
    let b = MatCatId::new(2, 10, 0);
    let props_a = props_for(&a);
    let props_b = props_for(&b);
    // Not guaranteed different for every field, but density is a good differentiator
    assert_ne!(props_a.density, props_b.density);
}
