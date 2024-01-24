use std::fs;

fn is_directory(entry: &fs::DirEntry) -> bool {
    if let Ok(metadata) = entry.metadata() {
        metadata.is_dir()
    } else {
        false
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Wrong number of argument provided.");
        return;
    }

    let root_folder = &args[1];
    match fs::read_dir(root_folder) {
        Ok(entries) => {
            let mut sorted_folders: Vec<_> = entries
                .filter_map(|entry| match entry {
                    Ok(entry) => {
                        if is_directory(&entry) {
                            Some(entry)
                        } else {
                            None
                        }
                    }
                    Err(_) => None,
                })
                .collect();
            sorted_folders.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

            println!("{:#?}", sorted_folders);
        }

        Err(err) => {
            eprintln!("Failed to read folder: {}", err);
        }
    }
}
