use items::command::Command;

mod items;
mod utils;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Wrong number of argument provided.");
    }

    let cmd_folders = utils::sorted_subfolders(&args[1])?;
    for folder in cmd_folders {
        let cmd = Command::new(&folder)?;
    }

    Ok(())
}
