use criterion::{criterion_group, criterion_main, Criterion};
use matcat::{MatCatId, props_for, find_closest_material, MatProps};

fn bench_props_for(c: &mut Criterion) {
    let id = MatCatId::new(1, 42, 0);
    c.bench_function("props_for (copper-like)", |b| {
        b.iter(|| props_for(&id))
    });
}

fn bench_find_closest(c: &mut Criterion) {
    // Build a search space of 1000 fake IDs
    let search_space: Vec<MatCatId> = (0..1000)
        .map(|i| MatCatId::new(1, i as u16, 0))
        .collect();

    // Target: arbitrary props
    let target = MatProps {
        density: 8000.0,
        elastic_modulus: 200.0,
        tensile_strength: 500.0,
        compressive_strength: 1000.0,
        hardness: 5.0,
        fracture_toughness: 20.0,
        fatigue_resistance: 0.5,

        thermal_conductivity: 100.0,
        thermal_expansion: 1e-5,
        melting_point: 1500.0,

        corrosion_resistance: 0.5,
        solubility: 0.2,
        permeability: 0.2,
        flammability: 0.1,

        electrical_conductivity: 0.8,
        magnetic_permeability: 10.0,
    };

    c.bench_function("find_closest_material (1000 candidates)", |b| {
        b.iter(|| {
            let _ = find_closest_material(&target, &search_space);
        })
    });
}

criterion_group!(benches, bench_props_for, bench_find_closest);
criterion_main!(benches);
