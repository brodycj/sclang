use criterion::{criterion_group, criterion_main, Criterion};

use std::{borrow::BorrowMut, collections::HashMap, sync::{LazyLock, Mutex}, time::Duration};

use sclang::sclang::{execute_command, SCLDataMap};

static UPDATE_ITERATION_COUNT: i32 = 5 * 1000;

static LAZY_MAP: LazyLock<Mutex<SCLDataMap>> = LazyLock::new(|| HashMap::new().into());

pub fn cb(c: &mut Criterion) {
    // ---
    let mut g = c.benchmark_group("ggg");
    // g.sample_size(200);
    g.measurement_time(Duration::from_secs(200));
    g.bench_function("bb-mm", |b| b.iter(mm));
}

criterion_group!(bbbb, cb);

criterion_main!(bbbb);

pub fn exec_quiet(command_line: &str) {
    let mut x = LAZY_MAP.lock().unwrap();
    let m = x.borrow_mut();
    execute_command(m, command_line);
}

// pub fn exec_verbose(command_line: &str) {
//     let mut x = LAZY_MAP.lock().unwrap();
//     let m = x.borrow_mut();
//     println!("{}", execute_command(m, command_line));
//     println!("=== === ===");
// }

fn mm() {
    // SETUP CIRCULAR DATA WITH 3 CELLS
    exec_quiet("(store-data a (\"a-text-1\" \"a-text-2\"))");
    exec_quiet("(store-data b (\"b-text-1\" \"b-text-2\" (a a)))");
    exec_quiet("(update-data a (\"a-text-1\" \"a-text-2\" (b b)))");
    exec_quiet("(store-data c (\"c-text-1\" \"c-text-2\" (a b)))");
    exec_quiet("(update-data a (\"a-text-1\" \"a-text-2\" (b c)))");
    exec_quiet("(update-data b (\"b-text-1\" \"b-text-2\" (a c)))");

    // SHOW STORED DATA
    // exec_verbose("(show-data a)");
    // exec_verbose("(show-data b)");
    // exec_verbose("(show-data c)");

    for _ in 1..UPDATE_ITERATION_COUNT {
        // REVERSE THE LINKS
        exec_quiet("(update-data a (\"a-text-1\" \"a-text-2\" (c b)))");
        exec_quiet("(update-data b (\"b-text-1\" \"b-text-2\" (c a)))");
        exec_quiet("(update-data c (\"c-text-1\" \"c-text-2\" (b a)))");
        // RESTORE THE LINKS
        exec_quiet("(update-data a (\"a-text-1\" \"a-text-2\" (b c)))");
        exec_quiet("(update-data b (\"b-text-1\" \"b-text-2\" (a c)))");
        exec_quiet("(update-data c (\"c-text-1\" \"c-text-2\" (a b)))");
    }
}
