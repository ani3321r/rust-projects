
# Pro Pomodoro Timer

A fully-featured Pomodoro Timer written in Rust using TUI (Text User Interface), Crossterm for terminal management, and Rodio for audio notifications. This application helps manage productivity by guiding work and break sessions using a visual interface and sounds.

## Features

- Customizable work and break durations.
- Text-based user interface with progress indication using a gauge.
- Audio notifications at the end of each work and break session.
- Real-time interaction using keyboard shortcuts (e.g., 'q' to quit).
- Session logging to track the completion of work and break periods.
- Summarizes daily totals for both work and break times.

## How It Works

1. **Timer Mechanism:** 
   - The app manages two primary timers:
     - **Work timer:** Counts down from a user-specified time (in minutes).
     - **Break timer:** Runs after the work timer finishes, for a user-defined break period.
   - Both timers display progress using a visual gauge, and once complete, the app plays an audio notification.
   
2. **Visual Interface:** 
   - The application leverages the `tui` crate to create a terminal-based UI:
     - **Gauges** display the remaining time for work or breaks.
     - The UI refreshes every second, keeping users informed of their session's status.

3. **Keyboard Interaction:** 
   - Users can interact with the app using keyboard inputs:
     - Press **'q'** to quit the application at any time.

4. **Session Logging:** 
   - After every session (both work and break), the app logs the session's details (timestamp, type of session, duration) in a file named `pomodoro_log.txt`.
   - This log is useful for tracking your progress over multiple sessions and days.

5. **Audio Notifications:** 
   - At the end of every work and break session, the application uses the `rodio` crate to play a short sound (a sine wave) as a notification, signaling the end of the current period.

## Dependencies

To run this project, you need the following dependencies:

```toml
[dependencies]
tui = "0.19.0"
crossterm = "0.26.1"
rodio = "0.14.0"
chrono = "0.4"
```

### Key Libraries and Their Usage:
- **tui:** Handles rendering the terminal-based UI, including widgets like gauges for visual feedback.
- **crossterm:** Manages terminal events and input, enabling features like key handling and controlling terminal behavior.
- **rodio:** Plays the audio notification sound when sessions are completed.
- **chrono:** Provides date and time functionalities for logging session completion times.

## How to Run

1. Clone the repository or download the code files.
2. Navigate to the project directory.
3. Ensure you have Rust installed. If not, install Rust using the following command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

4. Run the following command to start the Pomodoro Timer:

```bash
cargo run
```

5. Enter the desired work and break times in minutes when prompted.
6. Use 'q' to quit the application at any time.

## Logging

The application logs each completed work and break session in a file named `pomodoro_log.txt` in the project directory. Each entry includes the session type (work or break), duration, and the time it was completed.

## Example Usage

```bash
Enter work time in minutes: 25
Enter break time in minutes: 5
Pomodoro session started with 25 minutes of work and 5 minutes of break.
```

The application will display a progress gauge, and when the session ends, it will log the session and play a sound. It then switches between work and break periods.

## Contributing

1. Fork the repository.
2. Create your feature branch (`git checkout -b feature/your-feature`).
3. Commit your changes (`git commit -m 'Add your feature'`).
4. Push to the branch (`git push origin feature/your-feature`).
5. Create a new Pull Request.