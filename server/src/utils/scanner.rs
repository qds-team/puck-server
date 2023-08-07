use std::fs;
use crate::utils::env_load::get_universal_path;

pub fn scan_dir() -> std::io::Result<()> {
    let path = get_universal_path();

    // Read the directory
    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name = file_name.to_str().unwrap();

        // TODO: Check if file_name exists in the database
        // If not, add it to the database
    }

    Ok(())
}
