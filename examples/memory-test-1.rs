use sclang::sclang::{execute_command, SCLDataMap};

use memory_stats::memory_stats;

static UPDATE_ITERATION_COUNT: i32 = 2 * 1000 * 1000;

fn main() {
    let mut symbol_data_map = SCLDataMap::new();
    let m = &mut symbol_data_map;

    // SETUP CIRCULAR DATA WITH 3 CELLS
    execute_command(m, "(store-data a (\"a-text-1\" \"a-text-2\"))");
    execute_command(m, "(store-data b (\"b-text-1\" \"b-text-2\" (a a)))");
    execute_command(m, "(update-data a (\"a-text-1\" \"a-text-2\" (b b)))");
    execute_command(m, "(store-data c (\"c-text-1\" \"c-text-2\" (a b)))");
    execute_command(m, "(update-data a (\"a-text-1\" \"a-text-2\" (b c)))");
    execute_command(m, "(update-data b (\"b-text-1\" \"b-text-2\" (a c)))");

    // SHOW STORED DATA
    println!("{}", execute_command(m, "(show-data a)"));
    println!("{}", execute_command(m, "(show-data b)"));
    println!("{}", execute_command(m, "(show-data c)"));

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

    let stats = memory_stats().unwrap();
    println!("physical: {}", stats.physical_mem);
    println!("virtual: {}", stats.virtual_mem);
}
