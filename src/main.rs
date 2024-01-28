use crate::utils::sorted_subfolders;
use clap::Parser;
use items::command::Command;
use scope::Scope;
use std::{cell::RefCell, rc::Rc};
use transpile::Transpile;

mod items;
mod scope;
mod transpile;
mod utils;

#[derive(Parser)]
#[command(
    author = "acf-patrick",
    version = "1.0.0",
    about = "Interpreter and transpiler for esolang Folders 📂"
)]
struct Cli {
    /// Folder path to operate on
    folder: String,

    /// Transpile folder to actual source code
    #[arg(short, long)]
    transpile: bool,
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    let global_scope = Rc::new(RefCell::new(Scope::new(None)));
    let cmd_folders = sorted_subfolders(&cli.folder)?;

    // used for translation
    let mut token = String::new();

    for folder in cmd_folders {
        let mut cmd = Command::new(&folder, &global_scope)?;

        if cli.transpile {
            token += &cmd.transpile()?;
            token += "\n";
        } else {
            cmd.run()?;
        }
    }

    if cli.transpile {
        println!("fn main() {{");
        for line in token.split("\n") {
            println!("\t{line}");
        }
        println!("}}");
    }

    Ok(())
}
