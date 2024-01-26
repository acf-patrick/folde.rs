use std::fs;

pub fn input_error(msg: String) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidInput, msg)
}

pub fn is_directory(entry: &fs::DirEntry) -> bool {
    if let Ok(metadata) = entry.metadata() {
        metadata.is_dir()
    } else {
        false
    }
}

pub fn sorted_subfolders(folder: &str) -> std::io::Result<Vec<String>> {
    let entries = fs::read_dir(folder)?;

    let mut sorted_folders: Vec<_> = entries
        .filter_map(|entry| match entry {
            Ok(entry) => {
                if is_directory(&entry) {
                    Some(entry.path().to_str().unwrap().to_owned())
                } else {
                    None
                }
            }
            Err(_) => None,
        })
        .collect();

    sorted_folders.sort_by(|a, b| a.cmp(&b));
    Ok(sorted_folders)
}

pub fn subfolder_count(folder: &str) -> std::io::Result<usize> {
    let entries = fs::read_dir(folder)?;

    let mut count: usize = 0;
    for entry in entries {
        if let Ok(entry) = entry {
            if is_directory(&entry) {
                count += 1;
            }
        }
    }

    Ok(count)
}
