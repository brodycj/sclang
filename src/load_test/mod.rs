use super::sclang::{execute_command, SCLDataMap};

pub fn load_test(update_iteration_count: i32, read_iteration_count: i32) {
    let mut symbol_data_map = SCLDataMap::new();
    let m = &mut symbol_data_map;

    // SETUP CIRCULAR DATA WITH 3 CELLS
    execute_command(m, "(store-data a (\"a-text-1\" \"a-text-2\"))");
    execute_command(m, "(store-data b (\"b-text-1\" \"b-text-2\" (a a)))");
    execute_command(m, "(update-data a (\"a-text-1\" \"a-text-2\" (b b)))");
    execute_command(m, "(store-data c (\"c-text-1\" \"c-text-2\" (a b)))");
    execute_command(m, "(update-data a (\"a-text-1\" \"a-text-2\" (b c)))");
    execute_command(m, "(update-data b (\"b-text-1\" \"b-text-2\" (a c)))");

    for _ in 1..update_iteration_count {
        // REVERSE THE LINKS
        execute_command(m, "(update-data a (\"a-text-1\" \"a-text-2\" (c b)))");
        execute_command(m, "(update-data b (\"b-text-1\" \"b-text-2\" (c a)))");
        execute_command(m, "(update-data c (\"c-text-1\" \"c-text-2\" (b a)))");
        // RESTORE THE LINKS
        execute_command(m, "(update-data a (\"a-text-1\" \"a-text-2\" (b c)))");
        execute_command(m, "(update-data b (\"b-text-1\" \"b-text-2\" (a c)))");
        execute_command(m, "(update-data c (\"c-text-1\" \"c-text-2\" (a b)))");
    }

    for _ in 1..read_iteration_count {
        execute_command(m, "(show-data a)");
        execute_command(m, "(show-data b)");
        execute_command(m, "(show-data c)");
    }
}
