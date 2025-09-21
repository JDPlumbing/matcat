use matcat::{MatCatId, props_for};

#[test]
fn procedural_props_are_deterministic() {
    let id = MatCatId::new(1, 42, 7);
    let props1 = props_for(&id);
    let props2 = props_for(&id);
    assert_eq!(format!("{:?}", props1), format!("{:?}", props2), "props must be deterministic");
}

#[test]
fn different_ids_give_different_props() {
    let id1 = MatCatId::new(1, 42, 7);
    let id2 = MatCatId::new(1, 42, 8);
    let props1 = props_for(&id1);
    let props2 = props_for(&id2);
    assert_ne!(format!("{:?}", props1), format!("{:?}", props2), "props should vary across IDs");
}
