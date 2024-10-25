extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, Key, PressEvent, ReleaseEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::EventLoop;  
use rand::Rng;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const PADDLE_WIDTH: f64 = 20.0;
const PADDLE_HEIGHT: f64 = 100.0;
const BALL_SIZE: f64 = 15.0;
const PADDLE_SPEED: f64 = 5.0;
const BALL_SPEED: f64 = 4.0;
const AI_REACTION_SPEED: f64 = 4.0;  // Lower = faster reactions

#[derive(PartialEq)]
enum GameState {
    Menu,
    Playing,
}

#[derive(PartialEq)]
enum GameMode {
    TwoPlayer,
    VsComputer,
}

pub struct App {
    gl: GlGraphics,
    game_state: GameState,
    game_mode: GameMode,
    left_score: i32,
    left_pos: f64,
    left_vel: f64,
    right_score: i32,
    right_pos: f64,
    right_vel: f64,
    ball_x: f64,
    ball_y: f64,
    vel_x: f64,
    vel_y: f64,
}

impl App {
    fn new(gl: GlGraphics) -> App {
        App {
            gl,
            game_state: GameState::Menu,
            game_mode: GameMode::TwoPlayer,
            left_score: 0,
            left_pos: (WINDOW_HEIGHT as f64 - PADDLE_HEIGHT) / 2.0,
            left_vel: 0.0,
            right_score: 0,
            right_pos: (WINDOW_HEIGHT as f64 - PADDLE_HEIGHT) / 2.0,
            right_vel: 0.0,
            ball_x: (WINDOW_WIDTH as f64 - BALL_SIZE) / 2.0,
            ball_y: (WINDOW_HEIGHT as f64 - BALL_SIZE) / 2.0,
            vel_x: BALL_SPEED,
            vel_y: BALL_SPEED,
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        
        const BACKGROUND: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const FOREGROUND: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const HIGHLIGHT: [f32; 4] = [0.8, 0.8, 0.8, 1.0];
        
        self.gl.draw(args.viewport(), |c, gl| {
            clear(BACKGROUND, gl);

            if self.game_state == GameState::Menu {
                // Draw menu options as rectangles
                let mode_width = 200.0;
                let mode_height = 50.0;
                let mode_spacing = 100.0;
                let center_x = (WINDOW_WIDTH as f64 - mode_width) / 2.0;
                let center_y = (WINDOW_HEIGHT as f64 - mode_height) / 2.0;

                // Draw mode selection boxes
                let two_player_color = if self.game_mode == GameMode::TwoPlayer { HIGHLIGHT } else { FOREGROUND };
                let vs_computer_color = if self.game_mode == GameMode::VsComputer { HIGHLIGHT } else { FOREGROUND };

                // Two Player box
                rectangle(
                    two_player_color,
                    [center_x, center_y - mode_spacing, mode_width, mode_height],
                    c.transform,
                    gl
                );

                // VS Computer box
                rectangle(
                    vs_computer_color,
                    [center_x, center_y + mode_spacing, mode_width, mode_height],
                    c.transform,
                    gl
                );

                // Draw selection indicator (triangle)
                let triangle_size = 20.0;
                let triangle_x = center_x - triangle_size * 2.0;
                let triangle_y = if self.game_mode == GameMode::TwoPlayer {
                    center_y - mode_spacing + mode_height / 2.0
                } else {
                    center_y + mode_spacing + mode_height / 2.0
                };

                polygon(
                    FOREGROUND,
                    &[
                        [triangle_x, triangle_y - triangle_size / 2.0],
                        [triangle_x + triangle_size, triangle_y],
                        [triangle_x, triangle_y + triangle_size / 2.0],
                    ],
                    c.transform,
                    gl
                );

            } else {
                // Draw center line
                for y in (0..WINDOW_HEIGHT).step_by(30) {
                    rectangle(
                        FOREGROUND,
                        rectangle::square(0.0, 0.0, 5.0),
                        c.transform.trans((WINDOW_WIDTH as f64 - 5.0) / 2.0, y as f64),
                        gl
                    );
                }

                // Draw scores
                let score_size = 20.0;
                for i in 0..self.left_score {
                    rectangle(
                        FOREGROUND,
                        rectangle::square(0.0, 0.0, score_size),
                        c.transform.trans(50.0 + (i as f64 * 30.0), 30.0),
                        gl
                    );
                }
                
                for i in 0..self.right_score {
                    rectangle(
                        FOREGROUND,
                        rectangle::square(0.0, 0.0, score_size),
                        c.transform.trans(WINDOW_WIDTH as f64 - 150.0 + (i as f64 * 30.0), 30.0),
                        gl
                    );
                }

                // Draw paddles
                rectangle(
                    FOREGROUND,
                    [0.0, 0.0, PADDLE_WIDTH, PADDLE_HEIGHT],
                    c.transform.trans(10.0, self.left_pos),
                    gl
                );
                
                rectangle(
                    FOREGROUND,
                    [0.0, 0.0, PADDLE_WIDTH, PADDLE_HEIGHT],
                    c.transform.trans(WINDOW_WIDTH as f64 - PADDLE_WIDTH - 10.0, self.right_pos),
                    gl
                );

                // Draw ball
                rectangle(
                    FOREGROUND,
                    rectangle::square(0.0, 0.0, BALL_SIZE),
                    c.transform.trans(self.ball_x, self.ball_y),
                    gl
                );
            }
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        if self.game_state == GameState::Playing {
            // Update paddle positions
            self.left_pos = (self.left_pos + self.left_vel)
                .max(0.0)
                .min(WINDOW_HEIGHT as f64 - PADDLE_HEIGHT);
                
            // AI control for right paddle if in VS Computer mode
            if self.game_mode == GameMode::VsComputer {
                self.update_ai_paddle();
            } else {
                self.right_pos = (self.right_pos + self.right_vel)
                    .max(0.0)
                    .min(WINDOW_HEIGHT as f64 - PADDLE_HEIGHT);
            }

            // Update ball position
            self.ball_x += self.vel_x;
            self.ball_y += self.vel_y;

            // Ball collision with top and bottom
            if self.ball_y <= 0.0 || self.ball_y >= WINDOW_HEIGHT as f64 - BALL_SIZE {
                self.vel_y = -self.vel_y;
            }

            // Ball collision with paddles
            let ball_center_y = self.ball_y + BALL_SIZE / 2.0;

            // Left paddle collision
            if self.ball_x <= PADDLE_WIDTH + 10.0 && 
               ball_center_y >= self.left_pos && 
               ball_center_y <= self.left_pos + PADDLE_HEIGHT {
                self.vel_x = BALL_SPEED;
                let relative_intersect_y = (self.left_pos + (PADDLE_HEIGHT / 2.0)) - ball_center_y;
                self.vel_y = -(relative_intersect_y / (PADDLE_HEIGHT / 2.0)) * BALL_SPEED;
            }

            // Right paddle collision
            if self.ball_x >= WINDOW_WIDTH as f64 - PADDLE_WIDTH - BALL_SIZE - 10.0 &&
               ball_center_y >= self.right_pos && 
               ball_center_y <= self.right_pos + PADDLE_HEIGHT {
                self.vel_x = -BALL_SPEED;
                let relative_intersect_y = (self.right_pos + (PADDLE_HEIGHT / 2.0)) - ball_center_y;
                self.vel_y = -(relative_intersect_y / (PADDLE_HEIGHT / 2.0)) * BALL_SPEED;
            }

            // Scoring
            if self.ball_x < 0.0 {
                self.right_score += 1;
                self.reset_ball();
            }
            if self.ball_x > WINDOW_WIDTH as f64 {
                self.left_score += 1;
                self.reset_ball();
            }
        }
    }

    fn update_ai_paddle(&mut self) {
        // Predict where the ball will intersect with the AI paddle's x-position
        if self.vel_x > 0.0 {  // Only move if the ball is coming towards the AI
            let time_to_intersect = (WINDOW_WIDTH as f64 - PADDLE_WIDTH - BALL_SIZE - 10.0 - self.ball_x) / self.vel_x;
            let predicted_y = self.ball_y + self.vel_y * time_to_intersect;
            
            // Add some delay/imperfection to make the AI beatable
            let target_y = predicted_y + (rand::random::<f64>() - 0.5) * PADDLE_HEIGHT / 2.0;
            let paddle_center = self.right_pos + PADDLE_HEIGHT / 2.0;
            
            if paddle_center < target_y - AI_REACTION_SPEED {
                self.right_pos += PADDLE_SPEED;
            } else if paddle_center > target_y + AI_REACTION_SPEED {
                self.right_pos -= PADDLE_SPEED;
            }

            // Clamp paddle position
            self.right_pos = self.right_pos.max(0.0).min(WINDOW_HEIGHT as f64 - PADDLE_HEIGHT);
        }
    }

    fn reset_ball(&mut self) {
        self.ball_x = (WINDOW_WIDTH as f64 - BALL_SIZE) / 2.0;
        self.ball_y = (WINDOW_HEIGHT as f64 - BALL_SIZE) / 2.0;
        let mut rng = rand::thread_rng();
        self.vel_x = if rng.gen::<bool>() { BALL_SPEED } else { -BALL_SPEED };
        self.vel_y = if rng.gen::<bool>() { BALL_SPEED } else { -BALL_SPEED };
    }

    fn press(&mut self, args: &Button) {
        if let &Button::Keyboard(key) = args {
            match key {
                Key::Space => {
                    if self.game_state == GameState::Menu {
                        self.game_state = GameState::Playing;
                        self.reset_ball();
                    }
                }
                Key::Up | Key::Down => {
                    if self.game_state == GameState::Menu {
                        self.game_mode = match self.game_mode {
                            GameMode::TwoPlayer => GameMode::VsComputer,
                            GameMode::VsComputer => GameMode::TwoPlayer,
                        };
                    } else if self.game_mode == GameMode::TwoPlayer {
                        self.right_vel = if key == Key::Up { -PADDLE_SPEED } else { PADDLE_SPEED };
                    }
                }
                Key::W => self.left_vel = -PADDLE_SPEED,
                Key::S => self.left_vel = PADDLE_SPEED,
                _ => {}
            }
        }
    }

    fn release(&mut self, args: &Button) {
        if let &Button::Keyboard(key) = args {
            match key {
                Key::Up | Key::Down => {
                    if self.game_mode == GameMode::TwoPlayer {
                        self.right_vel = 0.0;
                    }
                }
                Key::W | Key::S => self.left_vel = 0.0,
                _ => {}
            }
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new("Pong", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App::new(GlGraphics::new(opengl));

    let mut events = Events::new(EventSettings::new().ups(60));

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }
        if let Some(u) = e.update_args() {
            app.update(&u);
        }
        if let Some(b) = e.press_args() {
            app.press(&b);
        }
        if let Some(b) = e.release_args() {
            app.release(&b);
        }
    }
}