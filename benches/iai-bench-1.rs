use sclang::load_test;

static UPDATE_ITERATION_COUNT_SHORT: i32 = 500;

static UPDATE_ITERATION_COUNT_LONG: i32 = 5 * 1000;

static READ_ITERATION_COUNT: i32 = 1;

fn iai_bench_short() {
    load_test::load_test(UPDATE_ITERATION_COUNT_SHORT, READ_ITERATION_COUNT);
}

fn iai_bench_long() {
    load_test::load_test(UPDATE_ITERATION_COUNT_LONG, READ_ITERATION_COUNT);
}

iai::main!(iai_bench_short, iai_bench_long);
