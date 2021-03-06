use criterion::{black_box, criterion_group, criterion_main, Criterion};
use supercut::masked_cut;

fn masked_cut_4k_benchmark(c: &mut Criterion) {
    let image = image::open("cat.jpg").unwrap();
    let mask = image::open("mask_4k.png").unwrap();
    c.bench_function("masked cut 4k", |b| {
        b.iter(|| {
            masked_cut(black_box(&image), black_box(&mask));
        })
    });
}

fn masked_cut_circle_benchmark(c: &mut Criterion) {
    let image = image::open("cat.jpg").unwrap();
    let mask = image::open("mask_4k.png").unwrap();
    c.bench_function("masked cut circle", |b| {
        b.iter(|| {
            masked_cut(black_box(&image), black_box(&mask));
        })
    });
}

criterion_group!(
    benches,
    masked_cut_4k_benchmark,
    masked_cut_circle_benchmark
);
criterion_main!(benches);
