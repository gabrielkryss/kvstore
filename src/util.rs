use std::path::Path;
use std::ffi::OsStr;

// recursively visits directories and counts the number of .key files
pub fn count_keys(dir: &Path) -> std::io::Result<usize> {
    let mut count: usize = 0;
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                count += count_keys(&path)?;
            } 
            else if path.is_file() && path.extension() == Some(OsStr::new("key")) {
                count += 1;
            }
        }
    }
    Ok(count)
}