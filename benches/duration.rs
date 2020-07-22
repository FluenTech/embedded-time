use criterion::{black_box, criterion_group, criterion_main, Criterion};
use embedded_time::{duration::units::*, traits::*};
use std::mem::size_of;

fn duration_read_write(c: &mut Criterion) {
    use core::time::Duration;

    let mut group = c.benchmark_group("Duration Read/Write");

    println!(
        "embedded_time::Duration size: {} B",
        size_of::<Milliseconds<u64>>()
    );
    group.bench_function("embedded_time::Duration", |b| {
        b.iter(|| {
            let ms = black_box(123);
            let duration = Milliseconds::<u64>(ms);
            let count = duration.count();
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

criterion_group!(benches, duration_read_write);
criterion_main!(benches);
