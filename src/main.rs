use matcat::{MatCatId, props_for};


fn main() {
    let copper = MatCatId::new(1, 42, 0);
    let pvc = MatCatId::new(2, 10, 0);

    let c_props = props_for(&copper);
    let p_props = props_for(&pvc);

    println!("Copper → density {} g/cm³, conductivity {}", c_props.density, c_props.conductivity);
    println!("PVC → density {} g/cm³, flammability {}", p_props.density, p_props.flammability);
}
