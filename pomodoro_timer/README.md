
# Pomodoro Timer in Rust

This is a terminal-based **Pomodoro Timer** written in Rust. It helps users manage their time by alternating between **25-minute work sessions** and **5-minute break sessions**. The timer provides a live progress bar in the terminal using a Text User Interface (TUI) with the `tui` and `crossterm` crates.

## Features
- **Work Session**: 25 minutes of focused work.
- **Break Session**: 5 minutes of relaxation.
- **Progress Bar**: Displays the remaining time in both work and break sessions.
- **Keyboard Control**: Press `q` to quit the timer at any time.

## How It Works
- The Pomodoro timer starts with a **25-minute work session** followed by a **5-minute break session**.
- After each session, the progress bar will reset, and a new session will start.
- The user can exit the timer anytime by pressing the `q` key.

## Getting Started

### Prerequisites
To run this project, ensure you have the following:
- [Rust](https://www.rust-lang.org/tools/install) installed on your machine.
- A terminal that supports ANSI escape sequences (for the TUI).

### Installation
1. **Clone the repository**:
   ```bash
   git clone https://github.com/ani3321r/rust-projects.git
   cd pomodoro_timer
   ```

2. **Install dependencies**:  
   Ensure the following dependencies are added to your `Cargo.toml`:
   ```toml
   [dependencies]
   tui = "0.19"
   crossterm = "0.26"
   chrono = "0.4"
   ```

3. **Run the program**:
   After you've cloned the repository and installed the necessary dependencies, you can run the timer by executing:
   ```bash
   cargo run
   ```

### Usage
- **Start Timer**: Run `cargo run` in your terminal.
- **Exit Timer**: Press the `q` key at any time to exit the timer.

### Example Output
```bash
Pomodoro session started at: 2024-10-09 14:32:45
Work Time - 24:59 remaining
Work Time - 24:58 remaining
...
Work Time Finished!
Break Time - 4:59 remaining
...
Break Time Finished!
```

## Dependencies
This project relies on the following Rust crates:
- **tui**: For building the terminal-based user interface.
- **crossterm**: For handling terminal control and keyboard events.
- **chrono**: For handling date and time in Rust.