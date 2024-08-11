use criterion::{criterion_group, criterion_main, Criterion};

use std::time::Duration;

use sclang::sclang::{execute_command, SCLDataMap};

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
    let mut symbol_data_map = SCLDataMap::new();
    let m = &mut symbol_data_map;

    // SETUP CIRCULAR DATA WITH 3 CELLS
    execute_command(m, "(store-data a (\"a-text-1\" \"a-text-2\"))");
    execute_command(m, "(store-data b (\"b-text-1\" \"b-text-2\" (a a)))");
    execute_command(m, "(update-data a (\"a-text-1\" \"a-text-2\" (b b)))");
    execute_command(m, "(store-data c (\"c-text-1\" \"c-text-2\" (a b)))");
    execute_command(m, "(update-data a (\"a-text-1\" \"a-text-2\" (b c)))");
    execute_command(m, "(update-data b (\"b-text-1\" \"b-text-2\" (a c)))");

    for _ in 1..UPDATE_ITERATION_COUNT {
        // REVERSE THE LINKS
        execute_command(m, "(update-data a (\"a-text-1\" \"a-text-2\" (c b)))");
        execute_command(m, "(update-data b (\"b-text-1\" \"b-text-2\" (c a)))");
        execute_command(m, "(update-data c (\"c-text-1\" \"c-text-2\" (b a)))");
        // RESTORE THE LINKS
        execute_command(m, "(update-data a (\"a-text-1\" \"a-text-2\" (b c)))");
        execute_command(m, "(update-data b (\"b-text-1\" \"b-text-2\" (a c)))");
        execute_command(m, "(update-data c (\"c-text-1\" \"c-text-2\" (a b)))");
    }
}
