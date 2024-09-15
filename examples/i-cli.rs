use std::collections::HashMap;

use rustyline::{config::Configurer, DefaultEditor, Result};

use sclang::sclang::{execute_command, SCDataRecordMap};

fn main() -> Result<()> {
    let mut map: SCDataRecordMap = HashMap::new();
    let m = &mut map;

    let mut r = DefaultEditor::new()?;
    r.set_auto_add_history(true);
    loop {
        let l = r.readline("--> ");
        match l {
            Ok(lt) => {
                let x = execute_command(m, lt.as_str());
                println!("{}", x);
            }
            Err(e) => {
                println!("ERROR: {:?}", e);
                break Err(e);
            }
        }
    }
}
