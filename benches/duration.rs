use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use embedded_time::{duration::*, rate::*, Fraction};
use std::mem::size_of;

fn duration_vs_core_duration(c: &mut Criterion) {
    use core::time::Duration;

    let mut group = c.benchmark_group("Duration Construct/Read");

    println!(
        "embedded_time::Duration size: {} B",
        size_of::<Milliseconds<u64>>()
    );
    group.bench_function("embedded_time::Duration", |b| {
        b.iter(|| {
            let ms = black_box(123);
            let duration = Milliseconds::<u64>(ms);
            let count = duration.integer();
            let _ = black_box(count);
        })
    });

    println!("core::time::Duration size: {} B", size_of::<Duration>());
    group.bench_function("core::time::Duration", |b| {
        b.iter(|| {
            let ms = black_box(123);
            let core_duration = Duration::from_millis(ms);
            let count = core_duration.as_millis();
            let _ = black_box(count);
        })
    });

    group.finish();
}

fn try_from_rate(c: &mut Criterion) {
    let mut group = c.benchmark_group("try_from_rate");

    let rate = 500_u32.Hz();
    group.bench_with_input(
        BenchmarkId::new("Milliseconds<u32> from Hertz<u32>", rate),
        &rate,
        |b, &_size| {
            b.iter(|| Milliseconds::<u32>::try_from_rate(rate));
        },
    );

    group.finish();
}

fn try_convert_from(c: &mut Criterion) {
    let mut group = c.benchmark_group("try_convert_from");

    let duration = 500_u32.seconds();
    group.bench_with_input(
        BenchmarkId::new("Nanoseconds<u64>_from_Seconds<u32>", duration),
        &duration,
        |b, &_size| {
            b.iter(|| Nanoseconds::<u64>::try_convert_from(duration));
        },
    );

    let duration = 500_u32.minutes();
    group.bench_with_input(
        BenchmarkId::new("Nanoseconds<u64>_from_Minutes<u32>", duration),
        &duration,
        |b, &_size| {
            b.iter(|| Nanoseconds::<u64>::try_convert_from(duration));
        },
    );

    let duration = 500_u32.nanoseconds();
    group.bench_with_input(
        BenchmarkId::new("Seconds<u64>_from_Nanoseconds<u32>", duration),
        &duration,
        |b, &_size| {
            b.iter(|| Seconds::<u64>::try_convert_from(duration));
        },
    );

    let duration = 500_u32.nanoseconds();
    group.bench_with_input(
        BenchmarkId::new("Minutes<u64>_from_Nanoseconds<u32>", duration),
        &duration,
        |b, &_size| {
            b.iter(|| Minutes::<u64>::try_convert_from(duration));
        },
    );

    let duration = 500_u64.nanoseconds();
    group.bench_with_input(
        BenchmarkId::new("Seconds<u32>_from_Nanoseconds<u64>", duration),
        &duration,
        |b, &_size| {
            b.iter(|| Seconds::<u32>::try_convert_from(duration));
        },
    );

    group.finish();
}

criterion_group!(
    benches,
    duration_vs_core_duration,
    try_convert_from,
    try_from_rate
);
criterion_main!(benches);
