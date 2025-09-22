use criterion::{criterion_group, criterion_main, Criterion};
use matcat::{MatCatId, props_for};

fn bench_category_props(c: &mut Criterion) {
    let mut group = c.benchmark_group("category_props");

    // Generate 10k samples across 10 categories
    group.bench_function("generate_props_10k", |b| {
        b.iter(|| {
            for cat in 1..=10 {
                for i in 0..1000 {
                    // (category: u8, variant: u16, grade: u16)
                    let id = MatCatId::new(cat, i as u16, 0);
                    // props_for takes a reference
                    let _props = props_for(&id);
                }
            }
        })
    });

    group.finish();
}

criterion_group!(benches, bench_category_props);
criterion_main!(benches);
