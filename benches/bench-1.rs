use criterion::{criterion_group, criterion_main, Criterion};

use std::time::Duration;

use sclang::load_test;

static UPDATE_ITERATION_COUNT: i32 = 5 * 1000;

// FOR FUTURE CONSIDERATION:
// static BENCH_SAMPLE_SIZE: usize = 200;

static BENCH_MEASUREMENT_TIME: u64 = 200;

pub fn my_bench_startup(c: &mut Criterion) {
    let mut g = c.benchmark_group("group1");

    // FOR FUTURE CONSIDERATION:
    // g.sample_size(BENCH_SAMPLE_SIZE);

    g.measurement_time(Duration::from_secs(BENCH_MEASUREMENT_TIME));

    g.bench_function("bench_1", |b| b.iter(bench_1));
}

criterion_group!(my_bench_main, my_bench_startup);

criterion_main!(my_bench_main);

fn bench_1() {
    load_test::load_test(UPDATE_ITERATION_COUNT);
}
