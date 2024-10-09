use std::{fs::OpenOptions, io::{self, Write}, thread, time::Duration};
use chrono::Local;
use tui::{backend::CrosstermBackend, widgets::{Block, Borders, Gauge}, Terminal};
use crossterm::event::{self, KeyCode};
use crossterm::{terminal, execute};
use rodio::{OutputStream, source::SineWave, Source};

fn tui_pomodoro_timer(work_minutes: u64, break_minutes: u64) -> Result<bool, Box<dyn std::error::Error>> {
    let stdout = std::io::stdout(); 
    terminal::enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut work_done = false;
    let mut progress = 0.0;

    let mut total_work_time = 0;
    let mut total_break_time = 0;

    loop {
        // Handle the timer logic (in seconds)
        let total_seconds = if !work_done { work_minutes * 60 } else { break_minutes * 60 };
        let mut timer_seconds = total_seconds;

        while timer_seconds > 0 {
            terminal.draw(|f| {
                let size = f.size();
                let gauge = Gauge::default()
                    .block(Block::default().title(if !work_done { "Work Time" } else { "Break Time" }).borders(Borders::ALL))
                    .gauge_style(
                        tui::style::Style::default()
                            .fg(tui::style::Color::Yellow)
                            .bg(tui::style::Color::Black),
                    )
                    .ratio(progress);
                f.render_widget(gauge, size);
            })?;

            // Sleep for 1 second, update timer and progress
            thread::sleep(Duration::from_secs(1));
            timer_seconds -= 1;

            // Update progress correctly as a ratio between 0 and 1
            progress = 1.0 - (timer_seconds as f64 / total_seconds as f64);
            progress = progress.clamp(0.0, 1.0); // Ensure progress stays between 0 and 1

            // Listen for quit input (q)
            if event::poll(Duration::from_millis(100))? {
                if let event::Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Char('q') {
                        terminal.clear()?;
                        terminal::disable_raw_mode()?;
                        return Ok(false); // Indicate that we want to quit
                    }
                }
            }
        }

        // Log the session and play notification sound
        if !work_done {
            total_work_time += work_minutes;
            log_session(true, work_minutes);
            play_notification_sound(); // Play sound after work session
        } else {
            total_break_time += break_minutes;
            log_session(false, break_minutes);
            play_notification_sound(); // Play sound after break session
        }

        // Switch between work and break sessions
        work_done = !work_done;
        progress = 0.0; // Reset progress for the next session
    }
}

fn get_user_input(prompt: &str) -> u64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Please enter a valid number")
}

fn log_session(is_work: bool, duration: u64) {
    let session_type = if is_work { "Work" } else { "Break" };
    let log_entry = format!(
        "{} - Completed a {} session for {} minutes\n",
        Local::now(), session_type, duration
    );

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("pomodoro_log.txt")
        .expect("Cannot open file");

    file.write_all(log_entry.as_bytes()).expect("Cannot write to file");
}

fn play_notification_sound() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let source = SineWave::new(440).take_duration(Duration::from_secs(1));
    stream_handle.play_raw(source.convert_samples()).unwrap();
    std::thread::sleep(Duration::from_secs(1)); // Give time for the sound to play
}

fn display_completed_sessions(count: u32) {
    println!("Completed work sessions: {}", count);
}

fn display_summary(total_work: u64, total_break: u64) {
    println!("Summary of the Day:");
    println!("Total work time: {} minutes", total_work);
    println!("Total break time: {} minutes", total_break);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let work_time = get_user_input("Enter work time in minutes: ");
    let break_time = get_user_input("Enter break time in minutes: ");

    println!("Pomodoro session started with {} minutes of work and {} minutes of break.", work_time, break_time);

    let mut session_count = 0;
    let mut total_work_time = 0;
    let mut total_break_time = 0;

    loop {
        // Start the Pomodoro Timer
        if tui_pomodoro_timer(work_time, break_time).expect("Pomodoro timer failed") == false {
            break; // Exit the loop if you want to quit
        }

        // Increment and display the completed sessions
        session_count += 1;
        display_completed_sessions(session_count);

        // Add to totals and display summary
        total_work_time += work_time;
        total_break_time += break_time;
        display_summary(total_work_time, total_break_time);
    }

    terminal::disable_raw_mode()?; // Ensure terminal is reset before exiting
    println!("Goodbye!");
    Ok(())
}