use criterion::{black_box, criterion_group, criterion_main, Criterion};

use sandpiles::Field;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("basic 1k", |b| b.iter(|| {
        let mut field = Field::new(black_box(1000), black_box(1000));
        *field.get_mut(field.width / 2, field.height / 2) = black_box(1000);
        while field.job_queue.len() != 0 {
            field.update();
        }
    }));

    let mut group = c.benchmark_group("hard");
    group.sample_size(10).bench_function("hard 10k", |b| b.iter(|| {
        let mut field = Field::new(black_box(1000), black_box(1000));
        *field.get_mut(field.width / 2, field.height / 2) = black_box(10000);
        while field.job_queue.len() != 0 {
            field.update();
        }
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);