use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn create_directory_if_not_exists(path: &Path) -> io::Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

pub fn move_file(from: &Path, to: &Path) -> io::Result<()> {
    if let Some(parent) = to.parent() {
        create_directory_if_not_exists(parent)?;
    }
    fs::rename(from, to)
}

pub fn get_file_extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_get_file_extension() {
        assert_eq!(
            get_file_extension(Path::new("test.PDF")),
            Some("pdf".to_string())
        );
        assert_eq!(get_file_extension(Path::new("test")), None);
    }
}