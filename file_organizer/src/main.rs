use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::collections::HashMap;

fn get_categories() -> HashMap<&'static str, Vec<&'static str>> {
    let mut categories = HashMap::new();
    categories.insert("Code", vec!["ts", "js", "c", "cpp", "rs", "py", "java", "zig"]); // u can add whatever files u like here
    categories.insert("Notes", vec!["txt", "pdf", "doc", "docx", "odt"]);
    categories.insert("Images", vec!["jpeg", "jpg", "png", "gif", "bmp", "tiff"]);
    categories
}

fn sort_files(dir: &Path, categories: &HashMap<&str, Vec<&str>>, log: &mut Vec<(PathBuf, PathBuf)>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
                for (category, extensions) in categories {
                    if extensions.contains(&extension) {
                        let category_path = dir.join(category);
                        fs::create_dir_all(&category_path)?;
                        let new_path = category_path.join(path.file_name().unwrap());

                        fs::rename(&path, &new_path)?;
                        log.push((path.clone(), new_path));
                        break;
                    }
                }
            }
        }
    }
    Ok(())
}

fn undo_last_sort(log: Vec<(PathBuf, PathBuf)>) -> io::Result<()> {
    for (original, moved) in log.into_iter().rev() {
        if moved.exists() {
            fs::rename(moved, original)?;
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let categories = get_categories();

    let target_dir = Path::new("./target_directory");
    let mut operation_log: Vec<(PathBuf, PathBuf)> = Vec::new();

    println!("File Organizer - Rust Version\n");
    println!("1. Sort files\n2. Undo last operation\nChoose an option: ");
    io::stdout().flush()?;

    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;
    let choice = choice.trim();

    match choice {
        "1" => {
            sort_files(target_dir, &categories, &mut operation_log)?;
            println!("Files sorted successfully!");
        }
        "2" => {
            undo_last_sort(operation_log)?;
            println!("Last operation undone!");
        }
        _ => println!("Invalid option!"),
    }

    Ok(())
}