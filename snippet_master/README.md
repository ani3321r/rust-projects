
# Snippet Master

The **Snippet Master** is a simple command-line tool built with Rust that allows you to store, organize, search, and manage code snippets. Each snippet can be categorized by programming language and tagged with keywords for easy retrieval.

## Features

- **Add Snippets**: Store code snippets with a title, programming language, tags, and content.
- **Edit Snippets**: Modify existing snippets’ content or tags.
- **Search Snippets**: Search snippets by tags.
- **File Storage**: Snippets are stored in a local directory as JSON files.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
  - [Add a Snippet](#add-a-snippet)
  - [Edit a Snippet](#edit-a-snippet)
  - [Search Snippets](#search-snippets)
  - [Quit](#quit)
- [Directory Structure](#directory-structure)
- [Future Enhancements](#future-enhancements)

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/ani3321r/rust-projects
   ```

2. Navigate to the project directory:

   ```bash
   cd code-snippet-manager
   ```

3. Install the required dependencies by running:

   ```bash
   cargo build
   ```

4. Run the program:

   ```bash
   cargo run
   ```

Make sure you have **Rust** installed. If you don't have it, you can install Rust by following the instructions [here](https://www.rust-lang.org/tools/install).

## Usage

After running the program, you can perform the following actions through a command-line interface (CLI):

### Add a Snippet

1. Choose the `add` option.
2. Enter the snippet title, programming language, tags (comma-separated), and the code snippet content.
3. The snippet will be saved in the `snippets` directory as a JSON file.

```bash
Choose an action: [add, edit, search, quit]
add
Enter snippet title:
My Snippet
Enter programming language:
Rust
Enter tags (comma-separated):
rust, cli, tools
Enter code snippet content:
fn main() { println!("Hello, world!"); }
Snippet added successfully!
```

### Edit a Snippet

1. Choose the `edit` option.
2. Enter the title of the snippet you want to edit.
3. Modify the snippet’s content and tags.
4. The updated snippet will be saved.

```bash
Choose an action: [edit, add, search, quit]
edit
Enter the title of the snippet you want to edit:
My Snippet
Enter new content:
fn main() { println!("Hello, Rust!"); }
Enter new tags (comma-separated):
rust, cli, updated
Snippet edited successfully!
```

### Search Snippets

1. Choose the `search` option.
2. Enter a tag to search for.
3. The snippets matching the tag will be displayed.

```bash
Choose an action: [search, add, edit, quit]
search
Enter tag to search for:
rust
Title: My Snippet, Language: Rust, Tags: ["rust", "cli", "updated"]
fn main() { println!("Hello, Rust!"); }
```

### Quit

1. Choose the `quit` option to exit the program.

```bash
Choose an action: [quit, add, edit, search]
quit
Goodbye!
```

## Directory Structure

The snippets are stored in the `snippets` directory as individual JSON files. Each file contains metadata and the code snippet.

Example structure:

```bash
code-snippet-manager/
│
├── Cargo.toml
├── src/
│   └── main.rs
├── snippets/
│   └── My Snippet.json
└── README.md
```

A snippet file (`My Snippet.json`) will look like this:

```json
{
  "title": "My Snippet",
  "language": "Rust",
  "tags": ["rust", "cli", "tools"],
  "content": "fn main() { println!("Hello, world!"); }"
}
```

Feel free to contribute by submitting issues or pull requests. Happy coding!
