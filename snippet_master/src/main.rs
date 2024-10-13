use serde::{Serialize, Deserialize};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;


#[derive(Serialize, Deserialize, Debug)]
struct Snippet {
    title: String,
    language: String,
    tags: Vec<String>,
    content: String,
}


impl Snippet {
    fn save_to_file(&self, directory: &str) -> std::io::Result<()> {
        // Create the directory if it doesn't exist
        fs::create_dir_all(directory)?;

        // File path: directory/title.json
        let file_path = Path::new(directory).join(format!("{}.json", self.title));

        // Serialize the snippet to JSON
        let json_data = serde_json::to_string_pretty(self)?;

        // Write the JSON to the file
        let mut file = File::create(file_path)?;
        file.write_all(json_data.as_bytes())?;

        Ok(())
    }

    fn load_from_file(file_path: &str) -> std::io::Result<Snippet> {
        let file_content = fs::read_to_string(file_path)?;
        let snippet: Snippet = serde_json::from_str(&file_content)?;
        Ok(snippet)
    }
}


fn add_snippet(title: &str, language: &str, tags: Vec<String>, content: &str, directory: &str) -> std::io::Result<()> {
    let snippet = Snippet {
        title: title.to_string(),
        language: language.to_string(),
        tags,
        content: content.to_string(),
    };
    snippet.save_to_file(directory)
}


fn search_snippets_by_tag(tag: &str, directory: &str) -> std::io::Result<Vec<Snippet>> {
    let mut results = Vec::new();

    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension() == Some(std::ffi::OsStr::new("json")) {
            let snippet = Snippet::load_from_file(path.to_str().unwrap())?;

            if snippet.tags.contains(&tag.to_string()) {
                results.push(snippet);
            }
        }
    }

    Ok(results)
}


fn edit_snippet(file_path: &str, new_content: &str, new_tags: Vec<String>) -> std::io::Result<()> {
    let mut snippet = Snippet::load_from_file(file_path)?;

    snippet.content = new_content.to_string();
    snippet.tags = new_tags;

    snippet.save_to_file(file_path)?;
    Ok(())
}


fn main() {
    println!("Welcome to the Code Snippet Manager!");

    loop {
        println!("Choose an action: [add, edit, search, quit]");
        let mut action = String::new();
        io::stdin().read_line(&mut action).expect("Failed to read line");

        match action.trim() {
            "add" => {
                println!("Enter snippet title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).expect("Failed to read line");

                println!("Enter programming language:");
                let mut language = String::new();
                io::stdin().read_line(&mut language).expect("Failed to read line");

                println!("Enter tags (comma-separated):");
                let mut tags = String::new();
                io::stdin().read_line(&mut tags).expect("Failed to read line");
                let tag_list: Vec<String> = tags.trim().split(',').map(|s| s.trim().to_string()).collect();

                println!("Enter code snippet content:");
                let mut content = String::new();
                io::stdin().read_line(&mut content).expect("Failed to read line");

                let result = add_snippet(&title.trim(), &language.trim(), tag_list, &content.trim(), "./snippets");
                match result {
                    Ok(_) => println!("Snippet added successfully!"),
                    Err(e) => eprintln!("Failed to add snippet: {}", e),
                }
            }
            "search" => {
                println!("Enter tag to search for:");
                let mut tag = String::new();
                io::stdin().read_line(&mut tag).expect("Failed to read line");

                let results = search_snippets_by_tag(&tag.trim(), "./snippets");
                match results {
                    Ok(snippets) => {
                        if snippets.is_empty() {
                            println!("No snippets found with tag '{}'", tag.trim());
                        } else {
                            for snippet in snippets {
                                println!("Title: {}, Language: {}, Tags: {:?}\n{}", 
                                    snippet.title, snippet.language, snippet.tags, snippet.content);
                            }
                        }
                    }
                    Err(e) => eprintln!("Error searching snippets: {}", e),
                }
            }
            "edit" => {
                println!("Enter the title of the snippet you want to edit:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).expect("Failed to read line");

                let file_path = format!("./snippets/{}.json", title.trim());
                
                if Path::new(&file_path).exists() {
                    println!("Enter new content:");
                    let mut content = String::new();
                    io::stdin().read_line(&mut content).expect("Failed to read line");

                    println!("Enter new tags (comma-separated):");
                    let mut tags = String::new();
                    io::stdin().read_line(&mut tags).expect("Failed to read line");
                    let tag_list: Vec<String> = tags.trim().split(',').map(|s| s.trim().to_string()).collect();

                    let result = edit_snippet(&file_path, &content.trim(), tag_list);
                    match result {
                        Ok(_) => println!("Snippet edited successfully!"),
                        Err(e) => eprintln!("Failed to edit snippet: {}", e),
                    }
                } else {
                    println!("Snippet with title '{}' not found.", title.trim());
                }
            }
            "quit" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid action!"),
        }
    }
}