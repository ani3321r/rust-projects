
# Task Manager

A simple command-line task management tool with an optional Terminal User Interface (TUI) built using Rust. This task manager allows you to add, list, complete, and delete tasks, all while maintaining the tasks in a JSON file for persistence. You can also interact with your tasks through a TUI interface for a more visual experience.

## Features

- **Add Tasks**: Add tasks with descriptions and optional due dates.
- **List Tasks**: View all tasks with their statuses and due dates.
- **Complete Tasks**: Mark tasks as complete.
- **Delete Tasks**: Remove tasks from the list.
- **TUI Mode**: A simple terminal UI for task visualization.
- **Persistent Storage**: Tasks are saved in a JSON file for later use.

## Prerequisites

Make sure you have the following installed:

- [Rust](https://www.rust-lang.org/) (1.54 or higher)

## Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/ani3321r/rust-projects.git
   ```
2. **Navigate to the project directory:**
   ```bash
   cd task-manager
   ```
3. **Build the project:**
   ```bash
   cargo build
   ```

## Usage

### Command-Line Interface

The task manager supports multiple commands to interact with tasks. Here are the available commands:

#### 1. Add a Task
You can add a new task with an optional due date using the `add` command.

```bash
cargo run -- add "Study Rust" --due 2024-10-12
```

- `description`: A description of the task.
- `--due`: An optional due date in the format `YYYY-MM-DD`.

#### 2. List All Tasks
To list all your tasks, use the `list` command:

```bash
cargo run -- list
```

#### 3. Complete a Task
You can mark a task as completed using its ID with the `complete` command:

```bash
cargo run -- complete 1
```

- `id`: The ID of the task you wish to mark as complete.

#### 4. Delete a Task
To remove a task from your list, use the `delete` command:

```bash
cargo run -- delete 1
```

- `id`: The ID of the task you wish to delete.

### Terminal User Interface (TUI)

For a visual display of your tasks, you can use the TUI by running the following command:

```bash
cargo run -- tui
```

The TUI will display your tasks in a formatted way and update them in real-time.

### Persistent Storage

All tasks are stored in a `tasks.json` file, which is created in the root directory after you add your first task. You can edit, backup, or transfer this file between machines to maintain your task list.

## Example Workflow

1. **Add a task with a due date:**
   ```bash
   cargo run -- add "Complete Rust project" --due 2024-10-15
   ```
   
2. **List your tasks:**
   ```bash
   cargo run -- list
   ```
   
3. **Complete the first task:**
   ```bash
   cargo run -- complete 1
   ```

4. **Run the TUI:**
   ```bash
   cargo run -- tui
   ```

5. **Delete the completed task:**
   ```bash
   cargo run -- delete 1
   ```

## Development

If you'd like to contribute or modify the project, follow these steps:

1. **Fork the repository**
2. **Clone your fork**:
   ```bash
   git clone https://github.com/ani3321r/rust-projects.git
   ```
3. **Create a new branch for your feature**:
   ```bash
   git checkout -b feature-branch
   ```
4. **Make your changes**
5. **Commit and push**:
   ```bash
   git commit -m "Add your feature"
   git push origin feature-branch
   ```
6. **Open a pull request**