use chrono::{NaiveDate, Utc};
use clap::{Parser, Subcommand, Args};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Write, stdout};
use std::path::Path;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};

// Struct for Task
#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    completed: bool,
    due_date: Option<NaiveDate>, // Use NaiveDate for date without time
}

impl Task {
    fn new(description: String, due_date: Option<NaiveDate>) -> Task {
        Task {
            description,
            completed: false,
            due_date,
        }
    }
}

// Function to parse due date from string
fn parse_due_date(due: &str) -> Result<NaiveDate, chrono::format::ParseError> {
    NaiveDate::parse_from_str(due, "%Y-%m-%d") // expects "YYYY-MM-DD" format
}

// Load tasks from a JSON file
fn load_tasks() -> Vec<Task> {
    if Path::new("tasks.json").exists() {
        let file_content = fs::read_to_string("tasks.json").unwrap();
        serde_json::from_str(&file_content).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    }
}

// Save tasks to a JSON file
fn save_tasks(tasks: &Vec<Task>) {
    let serialized = serde_json::to_string(tasks).unwrap();
    let mut file = File::create("tasks.json").unwrap();
    file.write_all(serialized.as_bytes()).unwrap();
}

// Command-line argument for adding tasks
#[derive(Args)]
struct AddArgs {
    description: String,
    #[arg(short, long)]
    due: Option<String>,
}

// Command-line parser struct
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// Command enum for different actions
#[derive(Subcommand)]
enum Commands {
    Add(AddArgs),
    List,
    Complete { id: usize },
    Delete { id: usize },
    Tui,
}

// Function to list all tasks
fn list_tasks(tasks: &Vec<Task>) {
    for (i, task) in tasks.iter().enumerate() {
        let status = if task.completed { "✓" } else { " " };
        let due_date = task
            .due_date
            .map(|d| d.to_string())
            .unwrap_or_else(|| "No due date".to_string());
        println!("{}. [{}] {} (Due: {})", i + 1, status, task.description, due_date);
    }
}

// Function to complete a task
fn complete_task(tasks: &mut Vec<Task>, id: usize) {
    if id < tasks.len() {
        tasks[id].completed = true;
        save_tasks(tasks);
        println!("Task {} marked as complete", id + 1);
    } else {
        println!("Invalid task ID");
    }
}

// Function to delete a task
fn delete_task(tasks: &mut Vec<Task>, id: usize) {
    if id < tasks.len() {
        tasks.remove(id);
        save_tasks(tasks);
        println!("Task {} deleted", id + 1);
    } else {
        println!("Invalid task ID");
    }
}

// Function to run TUI interface
fn run_tui<B: Backend>(terminal: &mut Terminal<B>, tasks: &Vec<Task>) -> Result<(), io::Error> {
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(60),
                    Constraint::Percentage(20),
                ]
                .as_ref(),
            )
            .split(f.size());

        let block = Block::default().title("Task Manager").borders(Borders::ALL);
        f.render_widget(block, chunks[0]);

        let block = Block::default().title("Tasks").borders(Borders::ALL);
        f.render_widget(block, chunks[1]);

        let mut task_str = String::new();
        for (i, task) in tasks.iter().enumerate() {
            let status = if task.completed { "✓" } else { " " };
            let due_date = task
                .due_date
                .map(|d| d.to_string())
                .unwrap_or_else(|| "No due date".to_string());
            task_str.push_str(&format!("{}. [{}] {} (Due: {})\n", i + 1, status, task.description, due_date));
        }

        let task_paragraph = Paragraph::new(task_str).style(Style::default().fg(Color::White));
        f.render_widget(task_paragraph, chunks[1]);

        let block = Block::default().title("Footer").borders(Borders::ALL);
        f.render_widget(block, chunks[2]);
    })?;

    Ok(())
}

fn main() -> Result<(), io::Error> {
    let args = Cli::parse();
    let mut tasks = load_tasks();

    // Initialize TUI backend for TUI command
    if let Commands::Tui = args.command {
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnableMouseCapture)?;

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        run_tui(&mut terminal, &tasks)?;

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), DisableMouseCapture)?;
        terminal.show_cursor()?;
    }

    // Command execution based on user input
    match args.command {
        Commands::Add(add_args) => {
            let due_date = match &add_args.due {
                Some(due) => match parse_due_date(due) {
                    Ok(date) => Some(date),
                    Err(e) => {
                        println!("Invalid date format: {}", e);
                        None
                    }
                },
                None => None,
            };

            let new_task = Task::new(add_args.description, due_date);
            tasks.push(new_task);
            save_tasks(&tasks);
            println!("Task added!");
        }
        Commands::List => {
            list_tasks(&tasks);
        }
        Commands::Complete { id } => {
            complete_task(&mut tasks, id - 1); // Task ID starts at 1 for users
        }
        Commands::Delete { id } => {
            delete_task(&mut tasks, id - 1); // Task ID starts at 1 for users
        }
        Commands::Tui => {
            // Handled above to run TUI, no extra logic needed here
        }
    }

    Ok(())
}