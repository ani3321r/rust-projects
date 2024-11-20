use ggez::{
    event::{self, EventHandler},
    graphics::{self, DrawMode, MeshBuilder},
    input::mouse::MouseButton,
    Context, GameResult,
};
use std::time::Instant;
use rand::Rng;

struct Vector2D {
    x: f32,
    y: f32,
}

impl Vector2D {
    fn new(x: f32, y: f32) -> Self {
        Vector2D { x, y }
    }

    fn to_point(&self) -> mint::Point2<f32> {
        mint::Point2 {
            x: self.x,
            y: self.y,
        }
    }

    fn distance(&self, other: &Vector2D) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    fn normalize(&self) -> Vector2D {
        let magnitude = (self.x * self.x + self.y * self.y).sqrt();
        if magnitude == 0.0 {
            return Vector2D::new(0.0, 0.0);
        }
        Vector2D::new(self.x / magnitude, self.y / magnitude)
    }
}

struct Ball {
    position: Vector2D,
    velocity: Vector2D,
    radius: f32,
    mass: f32,
    color: graphics::Color,
    energy_multiplier: f32,
}

impl Ball {
    fn new(position: Vector2D, velocity: Vector2D, radius: f32, mass: f32) -> Self {
        let mut rng = rand::thread_rng();
        let color = graphics::Color::new(
            rng.gen_range(0.5..1.0),
            rng.gen_range(0.5..1.0),
            rng.gen_range(0.5..1.0),
            1.0,
        );

        Ball {
            position,
            velocity,
            radius,
            mass,
            color,
            energy_multiplier: rng.gen_range(1.001..1.002),
        }
    }

    fn update(&mut self, dt: f32) {
        // Reduced gravity for slower vertical movement
        const GRAVITY: f32 = 9.81 * 15.0;
        // Very minimal damping
        const DAMPING: f32 = 0.9999;
        // High bounce retention
        const BOUNCE_DAMPENING: f32 = 0.99;
        // Minimal friction
        const FRICTION: f32 = 0.9995;
        // Higher max velocity allowed
        const MAX_VELOCITY: f32 = 800.0;
        
        // Apply reduced gravity
        self.velocity.y += GRAVITY * dt;
        
        // Apply minimal air resistance
        self.velocity.x *= DAMPING;
        self.velocity.y *= DAMPING;
        
        // Add slight energy to maintain motion
        self.velocity.x *= self.energy_multiplier;
        self.velocity.y *= self.energy_multiplier;
        
        // Update position
        self.position.x += self.velocity.x * dt;
        self.position.y += self.velocity.y * dt;
        
        // Handle boundary collisions with high energy retention
        self.handle_boundary_collisions(BOUNCE_DAMPENING, FRICTION);
        
        // Enforce maximum velocity
        self.limit_velocity(MAX_VELOCITY);
        
        // Add slight random impulses occasionally to maintain chaos
        self.add_random_impulse(dt);
    }

    fn add_random_impulse(&mut self, dt: f32) {
        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() < 0.01 * dt {  // 1% chance per dt
            let impulse_strength = 0.5;
            let angle = rng.gen::<f32>() * 2.0 * std::f32::consts::PI;
            self.velocity.x += angle.cos() * impulse_strength;
            self.velocity.y += angle.sin() * impulse_strength;
        }
    }

    fn handle_boundary_collisions(&mut self, bounce_dampening: f32, friction: f32) {
        const WINDOW_WIDTH: f32 = 800.0;
        const WINDOW_HEIGHT: f32 = 600.0;
        const EDGE_BOOST: f32 = 1.001; 
        
        // Floor collision with high energy retention
        if self.position.y + self.radius > WINDOW_HEIGHT {
            self.position.y = WINDOW_HEIGHT - self.radius;
            self.velocity.y = -self.velocity.y * bounce_dampening * EDGE_BOOST;
            self.velocity.x *= friction;
            
            // Add slight horizontal impulse on floor hits to maintain motion
            if self.velocity.x.abs() < 50.0 {
                self.velocity.x += if self.velocity.x > 0.0 { 10.0 } else { -10.0 };
            }
        }
        
        // Ceiling collision
        if self.position.y - self.radius < 0.0 {
            self.position.y = self.radius;
            self.velocity.y = -self.velocity.y * bounce_dampening * EDGE_BOOST;
        }
        
        // Wall collisions
        if self.position.x + self.radius > WINDOW_WIDTH {
            self.position.x = WINDOW_WIDTH - self.radius;
            self.velocity.x = -self.velocity.x * bounce_dampening * EDGE_BOOST;
        }
        if self.position.x - self.radius < 0.0 {
            self.position.x = self.radius;
            self.velocity.x = -self.velocity.x * bounce_dampening * EDGE_BOOST;
        }
    }

    fn limit_velocity(&mut self, max_velocity: f32) {
        let velocity_squared = self.velocity.x * self.velocity.x + 
                             self.velocity.y * self.velocity.y;
        if velocity_squared > max_velocity * max_velocity {
            let scale = max_velocity / velocity_squared.sqrt();
            self.velocity.x *= scale;
            self.velocity.y *= scale;
        }
    }

    fn collide_with(&mut self, other: &mut Ball) {
        let distance = self.position.distance(&other.position);
        let min_distance = self.radius + other.radius;

        if distance < min_distance {
            if distance == 0.0 {
                self.position.x += 0.1;
                return;
            }

            // Calculate collision normal
            let nx = (other.position.x - self.position.x) / distance;
            let ny = (other.position.y - self.position.y) / distance;

            // Calculate relative velocity
            let rx = other.velocity.x - self.velocity.x;
            let ry = other.velocity.y - self.velocity.y;
            let velocity_along_normal = rx * nx + ry * ny;

            if velocity_along_normal > 0.0 {
                return;
            }

            // High restitution for bouncy collisions
            let restitution = 0.98;
            
            // Calculate impulse with energy preservation
            let impulse_scalar = -(1.0 + restitution) * velocity_along_normal /
                               (1.0/self.mass + 1.0/other.mass);

            // Apply impulse with minimal damping
            let damping = 0.999;
            let impulse_x = impulse_scalar * nx * damping;
            let impulse_y = impulse_scalar * ny * damping;

            // Add slight repulsion to prevent clustering
            let repulsion = 0.1;
            let repulsion_x = nx * repulsion;
            let repulsion_y = ny * repulsion;

            // Apply combined impulse and repulsion
            self.velocity.x += (-impulse_x / self.mass - repulsion_x) * self.energy_multiplier;
            self.velocity.y += (-impulse_y / self.mass - repulsion_y) * self.energy_multiplier;
            other.velocity.x += (impulse_x / other.mass + repulsion_x) * other.energy_multiplier;
            other.velocity.y += (impulse_y / other.mass + repulsion_y) * other.energy_multiplier;

            // Separate balls
            self.separate_from(other, distance, min_distance);
        }
    }

    fn apply_impulse(&mut self, impulse_x: f32, impulse_y: f32) {
        self.velocity.x += impulse_x / self.mass;
        self.velocity.y += impulse_y / self.mass;
    }

    fn separate_from(&mut self, other: &mut Ball, distance: f32, min_distance: f32) {
        let overlap = min_distance - distance;
        let separation_x = overlap * (other.position.x - self.position.x) / distance * 0.5;
        let separation_y = overlap * (other.position.y - self.position.y) / distance * 0.5;

        self.position.x -= separation_x;
        self.position.y -= separation_y;
        other.position.x += separation_x;
        other.position.y += separation_y;
    }

    fn get_kinetic_energy(&self) -> f32 {
        0.5 * self.mass * (self.velocity.x * self.velocity.x + 
                          self.velocity.y * self.velocity.y)
    }

    fn get_momentum(&self) -> Vector2D {
        Vector2D::new(
            self.mass * self.velocity.x,
            self.mass * self.velocity.y
        )
    }

    fn is_moving(&self, threshold: f32) -> bool {
        self.velocity.x.abs() > threshold || self.velocity.y.abs() > threshold
    }

    fn add_force(&mut self, force_x: f32, force_y: f32, dt: f32) {
        // F = ma -> a = F/m
        let ax = force_x / self.mass;
        let ay = force_y / self.mass;
        
        // v = v0 + at
        self.velocity.x += ax * dt;
        self.velocity.y += ay * dt;
    }

    fn draw(&self, ctx: &mut Context) -> GameResult {
        let circle = MeshBuilder::new()
            .circle(
                DrawMode::fill(),
                self.position.to_point(),
                self.radius,
                0.1,
                self.color,
            )
            .build(ctx)?;
        
        graphics::draw(ctx, &circle, graphics::DrawParam::default())
    }

    fn set_color(&mut self, color: graphics::Color) {
        self.color = color;
    }
}

struct PhysicsEngine {
    balls: Vec<Ball>,
    last_update: Instant,
    mouse_start: Option<Vector2D>,
    mouse_end: Option<Vector2D>,
    gravity_enabled: bool,
}

impl PhysicsEngine {
    fn new(_ctx: &mut Context) -> GameResult<PhysicsEngine> {
        let mut rng = rand::thread_rng();
        let mut balls = Vec::with_capacity(100);

        // Create custom number of balls with random properties
        for _ in 0..50 { // u can change this number to create more or less balls
            let radius = rng.gen_range(10.0..25.0);
            let x = rng.gen_range(radius..800.0 - radius);
            let y = rng.gen_range(radius..600.0 - radius);
            let vx = rng.gen_range(-100.0..100.0);
            let vy = rng.gen_range(-100.0..100.0);
            let mass = radius * 0.2;

            let ball = Ball::new(
                Vector2D::new(x, y),
                Vector2D::new(vx, vy),
                radius,
                mass,
            );
            balls.push(ball);
        }

        Ok(PhysicsEngine {
            balls,
            last_update: Instant::now(),
            mouse_start: None,
            mouse_end: None,
            gravity_enabled: true, // Initialize gravity as enabled
        })
    }

    fn handle_collisions(&mut self) {
        // Process multiple collision iterations to improve stability
        for _ in 0..3 {
            for i in 0..self.balls.len() {
                for j in (i + 1)..self.balls.len() {
                    let (ball1, ball2) = get_two_balls_mut(&mut self.balls, i, j);
                    ball1.collide_with(ball2);
                }
            }
        }
    }

    fn reset_simulation(&mut self) {
        let mut rng = rand::thread_rng();
        self.balls.clear();
        for _ in 0..100 {
            let radius = rng.gen_range(10.0..25.0);
            let x = rng.gen_range(radius..800.0 - radius);
            let y = rng.gen_range(radius..600.0 - radius);
            let vx = rng.gen_range(-100.0..100.0);
            let vy = rng.gen_range(-100.0..100.0);
            let mass = radius * 0.2;
            let ball = Ball::new(Vector2D::new(x, y), Vector2D::new(vx, vy), radius, mass);
            self.balls.push(ball);
        }
    }
}

// Helper function to get mutable references to two balls
fn get_two_balls_mut(balls: &mut [Ball], i: usize, j: usize) -> (&mut Ball, &mut Ball) {
    if i > j {
        let (right, left) = balls.split_at_mut(i);
        (&mut left[j], &mut right[0])
    } else {
        let (right, left) = balls.split_at_mut(j);
        (&mut right[i], &mut left[0])
    }
}

impl EventHandler for PhysicsEngine {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let now = Instant::now();
        let dt = (now - self.last_update).as_secs_f32();
        self.last_update = now;

        // Update ball positions and colors
        for ball in &mut self.balls {
            ball.update(dt);
            ball.update(dt); // Update color cycling
        }

        // Apply gravity if enabled
        if self.gravity_enabled {
            const GRAVITY: f32 = 9.81 * 15.0; // Adjust gravity as needed
            for ball in &mut self.balls {
                ball.velocity.y += GRAVITY * dt; // Apply gravity to each ball
            }
        }

        // Handle collisions between balls
        self.handle_collisions();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        // Draw all balls
        for ball in &self.balls {
            let circle = MeshBuilder::new()
                .circle(
                    DrawMode::fill(),
                    ball.position.to_point(),
                    ball.radius,
                    0.1,
                    ball.color,
                )
                .build(ctx)?;
            graphics::draw(ctx, &circle, graphics::DrawParam::default())?;
        }

        // Draw launch trajectory if dragging
        if let (Some(start), Some(end)) = (self.mouse_start.as_ref(), self.mouse_end.as_ref()) {
            let line = MeshBuilder::new()
                .line(
                    &[start.to_point(), end.to_point()],
                    2.0,
                    graphics::WHITE,
                )?
                .build(ctx)?;
            graphics::draw(ctx, &line, graphics::DrawParam::default())?;
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) {
        if button == MouseButton::Left {
            self.mouse_start = Some(Vector2D::new(x, y));
            self.mouse_end = Some(Vector2D::new(x, y));

            // Check if a ball is clicked
            for ball in &mut self.balls {
                if ball.position.distance(&Vector2D::new(x, y)) < ball.radius {
                    ball.set_color(graphics::Color::new(1.0, 0.0, 0.0, 1.0)); // Change color to red
                }
            }
        }
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) {
        if button == MouseButton::Left {
            if let Some(start) = self.mouse_start.take() {
                // Calculate launch velocity based on drag distance and direction
                let dx = start.x - x;
                let dy = start.y - y;
                let velocity_scale = 2.0;

                // Apply force to all balls within a certain radius
                for ball in &mut self.balls {
                    if ball.position.distance(&start) < 100.0 { // Apply force to nearby balls
                        ball.add_force(dx * velocity_scale, dy * velocity_scale, 1.0);
                    }
                }
            }
            self.mouse_start = None;
            self.mouse_end = None;
        }
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        if self.mouse_start.is_some() {
            self.mouse_end = Some(Vector2D::new(x, y));
        }
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: ggez::event::KeyCode, _keymod: ggez::event::KeyMods, _repeat: bool) {
        match keycode {
            ggez::event::KeyCode::G => {
                // Toggle gravity
                self.gravity_enabled = !self.gravity_enabled;
            }
            ggez::event::KeyCode::Up => {
                // Increase ball size
                for ball in &mut self.balls {
                    ball.radius += 1.0;
                }
            }
            ggez::event::KeyCode::Down => {
                // Decrease ball size
                for ball in &mut self.balls {
                    if ball.radius > 1.0 {
                        ball.radius -= 1.0;
                    }
                }
            }
            ggez::event::KeyCode::R => {
                // Reset the simulation
                self.reset_simulation();
            }
            _ => {}
        }
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("physics_engine", "fatbrad")
        .window_setup(ggez::conf::WindowSetup::default().title("Colliding Balls"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0));

    let (mut ctx, mut event_loop) = cb.build()?;
    let mut state = PhysicsEngine::new(&mut ctx)?;
    event::run(&mut ctx, &mut event_loop, &mut state)
}