use matcat::{MatCatId, props_for};

#[test]
fn test_copper_props() {
    let copper = MatCatId::new(1, 42, 0); // Metal → Copper
    let props = props_for(&copper);

    assert!((props.density - 8.9).abs() < 0.01);
    assert!((props.conductivity - 1.0).abs() < 0.01);
    assert!(props.hardness > 2.5 && props.hardness < 3.5);
}

#[test]
fn test_pvc_props() {
    let pvc = MatCatId::new(2, 10, 0); // Plastic → PVC
    let props = props_for(&pvc);

    assert!((props.density - 1.4).abs() < 0.01);
    assert!(props.flammability < 0.5);
    assert!(props.tensile > 50.0);
}

#[test]
fn test_unknown_material_defaults() {
    let unknown = MatCatId::new(99, 999, 0); // Something undefined
    let props = props_for(&unknown);

    // Should fall back to water-ish defaults
    assert!((props.density - 1.0).abs() < 0.01);
    assert_eq!(props.conductivity, 0.0);
    assert!(props.tensile < 20.0);
}
