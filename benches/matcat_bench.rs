use criterion::{criterion_group, criterion_main, Criterion};
use matcat::{MatCatId, props_for};

fn bench_props_lookup(c: &mut Criterion) {
    let copper = MatCatId::new(1, 42, 0);
    let pvc = MatCatId::new(2, 10, 0);

    c.bench_function("props_for copper", |b| {
        b.iter(|| {
            let _ = props_for(&copper);
        })
    });

    c.bench_function("props_for pvc", |b| {
        b.iter(|| {
            let _ = props_for(&pvc);
        })
    });
}

criterion_group!(benches, bench_props_lookup);
criterion_main!(benches);
