use criterion::{black_box, criterion_group, criterion_main, Criterion};
use embedded_time::{duration::*, rate::*};

fn rem_duration_rate(c: &mut Criterion) {
    use core::time::Duration;

    let mut group = c.benchmark_group("Rem Duration vs Rate");

    group.bench_function("duration", |b| {
        b.iter(|| {
            let dur1 = black_box(402);
            let dur2 = black_box(10);
            let rem = Seconds::<u64>(dur1) % Seconds::<u64>(dur2);
            let count = rem.integer();
            let _ = black_box(count);
        })
    });

    group.bench_function("rate", |b| {
        b.iter(|| {
            let rate1 = black_box(402);
            let rate2 = black_box(10);
            let rem = Hertz::<u64>(rate1) % Hertz::<u64>(rate2);
            let count = rem.integer();
            let _ = black_box(count);
        })
    });

    group.finish();
}

criterion_group!(benches, rem_duration_rate);
criterion_main!(benches);
