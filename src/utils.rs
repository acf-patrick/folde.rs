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

    sorted_folders.sort_by(|a, b| human_sort::compare(&a.to_lowercase(), &b.to_lowercase()));

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

/// used in context of literal values
pub fn is_bit_set(folder: &str) -> bool {
    subfolder_count(&folder).unwrap_or(0) != 0
}

/// read byte from folder
pub fn get_byte(folder: &str) -> std::io::Result<u8> {
    let hex_folders = sorted_subfolders(&folder)?;
    if hex_folders.len() != 2 {
        return Err(input_error(format!(
            "{} : invalid byte, found {} subfolders",
            folder,
            hex_folders.len()
        )));
    }

    let left = sorted_subfolders(&hex_folders[0])?;
    let right = sorted_subfolders(&hex_folders[1])?;

    if left.len() != 4 {
        return Err(input_error(format!(
            "{} : invalid hex digit, found {} subfolders",
            hex_folders[0],
            left.len()
        )));
    }

    if right.len() != 4 {
        return Err(input_error(format!(
            "{} : invalid hex digit, found {} subfolders",
            hex_folders[1],
            right.len()
        )));
    }

    let mut i = 7;
    let mut byte: u8 = 0;

    for bit_folder in left {
        byte |= ((is_bit_set(&bit_folder) as i32) << i) as u8;
        i -= 1;
    }

    for bit_folder in right {
        byte |= ((is_bit_set(&bit_folder) as i32) << i) as u8;
        i -= 1;
    }

    Ok(byte)
}
