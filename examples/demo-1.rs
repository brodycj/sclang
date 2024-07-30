use std::collections::HashMap;

use sclang::sclang::{execute_command, SCLDataMap};

pub fn execute(m: &mut SCLDataMap, command_line: &str) {
    println!("{}", execute_command(m, command_line));
    println!("=== === ===");
}

fn main() {
    let mut map: SCLDataMap = HashMap::new();
    let m = &mut map;

    execute(m, "( store-data data-1 (\"abc\" \"fds\") )");
    execute(m, "( show-data data-1 )");
    execute(m, "( update-data data-1 (\"ghi\" \"afse\" (data-1 data-1) ) )");
    execute(m, "( show-data data-1 )");
    execute(m, "( store-data data-2 (\"fdsjkl\" \"fdsjkl\" (data-1 data-1) ) )");
    execute(m, "( update-data data-2 (\"ghi\" \"afse\" (data-1 data-1) ) )");
    execute(m, "( show-data data-2 )");
    execute(m, "( drop-symbol data-2 )");
    execute(m, "( show-data data-1 )");
}
