use criterion::{criterion_group, criterion_main, Criterion};
use matcat::{MatCatId, props_for};

fn bench_category_props(c: &mut Criterion) {
    let mut group = c.benchmark_group("category_props");

    // try 10k samples across a few categories
    group.bench_function("generate_props_10k", |b| {
        b.iter(|| {
            for cat in 1..=10 {
                for i in 0..1000 {
                    let id = MatCatId::new(cat, (i & 0xFF) as u8, 0, 0, 0);
                    let _props = props_for(id);
                }
            }
        })
    });

    group.finish();
}

criterion_group!(benches, bench_category_props);
criterion_main!(benches);
