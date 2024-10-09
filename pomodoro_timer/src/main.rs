use std::{thread, time::Duration};
use chrono::Local;
use tui::{backend::CrosstermBackend, widgets::{Block, Borders, Gauge}, Terminal};
use crossterm::event::{self, KeyCode};
use crossterm::{terminal, ExecutableCommand};
use std::io::{stdout, Write};

fn tui_pomodoro_timer(work_minutes: u64, break_minutes: u64) -> Result<(), Box<dyn std::error::Error>> {
    let stdout = stdout();
    terminal::enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut work_done = false;
    let mut progress = 0.0;

    loop {
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
            
            // Ensure the progress ratio is bounded between 0 and 1
            if progress > 1.0 {
                progress = 1.0;
            } else if progress < 0.0 {
                progress = 0.0;
            }

            // Listen for quit input (q)
            if event::poll(Duration::from_millis(100))? {
                if let event::Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Char('q') {
                        terminal.clear()?;
                        terminal::disable_raw_mode()?;
                        return Ok(());
                    }
                }
            }
        }

        // Switch between work and break sessions
        work_done = !work_done;
        progress = 0.0; // Reset progress for the next session
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Pomodoro session started at: {}", Local::now());
    tui_pomodoro_timer(25, 5)?; // 25 min work, 5 min break
    println!("Exiting Pomodoro Timer. Have a productive day!");
    Ok(())
}