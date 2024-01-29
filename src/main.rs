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
    let mut lines: Vec<String> = vec![];
    let mut using_print = false;

    for folder in cmd_folders {
        let mut cmd = Command::new(&folder, &global_scope)?;

        if cli.transpile {
            lines.extend(
                cmd.transpile()?
                    .split("\n")
                    .map(|line| line.to_owned())
                    .into_iter(),
            );

            if lines.iter().find(|line| line.contains("print")).is_some() {
                using_print = true;
            }
        } else {
            cmd.run()?;
        }
    }

    if cli.transpile {
        if using_print {
            println!("use std::io::Write;\n");
        }

        println!("fn main() {{");
        for line in lines {
            println!("\t{line}");
        }
        println!("}}");
    }

    Ok(())
}
