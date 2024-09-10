use sclang::load_test;

use memory_stats::memory_stats;

static UPDATE_ITERATION_COUNT: i32 = 2 * 1000 * 1000;

fn main() {
    load_test::load_test(UPDATE_ITERATION_COUNT);

    let stats = memory_stats().unwrap();
    println!("physical: {}", stats.physical_mem);
    println!("virtual: {}", stats.virtual_mem);
}
