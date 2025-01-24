use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use glam::Vec2;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 600;
const LIGHT_RADIUS: f32 = 10.0;
const BALL_RADIUS: f32 = 50.0;
const BALL_CENTER: Vec2 = Vec2::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0);
const RAY_STEP: f32 = 1.0;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Raytracing GUI Demo")
        .with_inner_size(PhysicalSize::new(WIDTH, HEIGHT))
        .build(&event_loop)?;

    let mut pixels = Pixels::new(
        WIDTH,
        HEIGHT,
        SurfaceTexture::new(WIDTH, HEIGHT, &window),
    )?;

    let mut light_pos = Vec2::new(WIDTH as f32 / 4.0, HEIGHT as f32 / 2.0);
    let mut dragging = false;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::CursorMoved { position, .. } => {
                    if dragging {
                        light_pos = Vec2::new(
                            position.x.clamp(0.0, WIDTH as f64) as f32,
                            position.y.clamp(0.0, HEIGHT as f64) as f32,
                        );
                    }
                }
                WindowEvent::MouseInput { state, button, .. } => {
                    if button == MouseButton::Left {
                        dragging = match state {
                            ElementState::Pressed => {
                                let mouse_pos = Vec2::new(light_pos.x, light_pos.y);
                                mouse_pos.distance(light_pos) <= LIGHT_RADIUS
                            }
                            ElementState::Released => false,
                        };
                    }
                }
                _ => {}
            },
            Event::RedrawRequested(_) => {
                let frame = pixels.get_frame_mut();
                frame.fill(0);
                draw_scene(frame, light_pos);
                if pixels.render().is_err() {
                    eprintln!("Failed to render frame");
                    *control_flow = ControlFlow::Exit;
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}

fn draw_scene(frame: &mut [u8], light_pos: Vec2) {
    // Draw rays
    for angle in (0..360).step_by(2) {
        let rad = (angle as f32).to_radians();
        let ray_dir = Vec2::new(rad.cos(), rad.sin());
        let mut current_pos = light_pos;

        while current_pos.x >= 0.0
            && current_pos.x < WIDTH as f32
            && current_pos.y >= 0.0
            && current_pos.y < HEIGHT as f32
        {
            let pixel_index = ((current_pos.y as u32 * WIDTH + current_pos.x as u32) * 4) as usize;
            if pixel_index < frame.len() {
                // Check if ray intersects the ball
                if current_pos.distance(BALL_CENTER) <= BALL_RADIUS {
                    frame[pixel_index..pixel_index + 4].copy_from_slice(&[30, 30, 30, 255]); // Shadow color
                    break;
                }
                frame[pixel_index..pixel_index + 4].copy_from_slice(&[255, 255, 200, 255]); // Light ray color
            }
            current_pos += ray_dir * RAY_STEP;
        }
    }

    // Draw the ball
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let pixel_index = ((y * WIDTH + x) * 4) as usize;
            let pixel_pos = Vec2::new(x as f32, y as f32);

            if pixel_pos.distance(BALL_CENTER) <= BALL_RADIUS {
                frame[pixel_index..pixel_index + 4].copy_from_slice(&[100, 0, 0, 255]);
            }
        }
    }

    // Draw the light source
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let pixel_index = ((y * WIDTH + x) * 4) as usize;
            let pixel_pos = Vec2::new(x as f32, y as f32);

            if pixel_pos.distance(light_pos) <= LIGHT_RADIUS {
                frame[pixel_index..pixel_index + 4].copy_from_slice(&[255, 255, 200, 255]);
            }
        }
    }
}