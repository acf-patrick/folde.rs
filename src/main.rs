use crate::utils::sorted_subfolders;
use items::command::Command;
use scope::Scope;
use std::{cell::RefCell, rc::Rc};

mod items;
mod scope;
mod utils;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        // panic!("Wrong number of argument provided.");
    }

    let global_scope = Rc::new(RefCell::new(Scope::new(None)));
    let cmd_folders = sorted_subfolders("./samples/HelloWorld")?;

    for folder in cmd_folders {
        let cmd = Command::new(&folder, &global_scope)?;
        cmd.run()?;
    }

    Ok(())
}
