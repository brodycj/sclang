use sclang::load_test;

static UPDATE_ITERATION_COUNT: i32 = 2;

static READ_ITERATION_COUNT_SHORT: i32 = 500;

static READ_ITERATION_COUNT_LONG: i32 = 5 * 1000;

fn iai_bench_short() {
    load_test::load_test(UPDATE_ITERATION_COUNT, READ_ITERATION_COUNT_SHORT);
}

fn iai_bench_long() {
    load_test::load_test(UPDATE_ITERATION_COUNT, READ_ITERATION_COUNT_LONG);
}

iai::main!(iai_bench_short, iai_bench_long);
