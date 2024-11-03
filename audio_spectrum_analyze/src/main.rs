use iced::widget::{canvas::{self, Cache, Canvas, Frame, Geometry, Text}, Column};
use iced::{
    executor, Application, Color, Command, Element, Length, Point, Rectangle, Settings, Size,
    Subscription, Theme, time,
};
use iced::mouse;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rustfft::{num_complex::Complex, FftPlanner};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;

const FFT_SIZE: usize = 960;
const SAMPLE_RATE: f32 = 48000.0;
const MIN_DB: f32 = -60.0;
const MAX_DB: f32 = 0.0;
const PEAK_DECAY_RATE: f32 = 0.05;
const SMOOTHING_FACTOR: f32 = 0.2;

struct SpectrumAnalyzer {
    audio_data: Vec<f32>,
    spectrum_data: Vec<f32>,
    peak_levels: Vec<f32>,
    prev_spectrum: Vec<f32>,
    rx: Receiver<Vec<f32>>,
    _tx: Sender<Vec<f32>>,
    cache: Cache,
    frame_count: usize,
}

#[derive(Debug, Clone)]
enum Message {
    Tick,
}

impl Application for SpectrumAnalyzer {
    type Message = Message;
    type Theme = Theme;
    type Flags = ();
    type Executor = executor::Default;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let (tx, rx) = channel();
        let _tx = tx.clone();

        let host = cpal::default_host();
        
        // Print available input devices
        println!("Available input devices:");
        for device in host.input_devices().unwrap() {
            println!("  - {}", device.name().unwrap());
        }
        
        let device = host.default_input_device().expect("no input device");
        let config = device.default_input_config().unwrap();
        
        println!("\nSelected device: {:?}", device.name());
        println!("Config: {:?}", config);
        println!("Sample rate: {:?}", config.sample_rate());
        println!("Channels: {:?}", config.channels());

        let tx_clone = tx.clone();
        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_input_stream(
                &config.into(),
                move |data: &[f32], _: &_| {
                    let max_val = data.iter().fold(0.0f32, |max, &x| max.max(x.abs()));
                    // println!("Audio callback - length: {}, max value: {}", data.len(), max_val);
                    if data.len() >= FFT_SIZE {
                        tx_clone.send(data[..FFT_SIZE].to_vec()).unwrap();
                    }
                },
                |err| eprintln!("Error in audio stream: {}", err),
                None,
            ),
            _ => panic!("Unsupported sample format"),
        }
        .unwrap();

        // Make sure we explicitly start the stream
        println!("Starting audio stream...");
        stream.play().unwrap();
        
        // Store the stream to prevent it from being dropped
        std::mem::forget(stream);

        let spectrum_size = FFT_SIZE / 2;
        (
            SpectrumAnalyzer {
                audio_data: Vec::new(),
                spectrum_data: vec![0.0; spectrum_size],
                peak_levels: vec![0.0; spectrum_size],
                prev_spectrum: vec![0.0; spectrum_size],
                rx,
                _tx: tx,
                cache: Cache::new(),
                frame_count: 0,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Enhanced Spectrum Analyzer")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick => {
                self.frame_count += 1;

                while let Ok(data) = self.rx.try_recv() {
                    let max_amplitude = data.iter().fold(0.0f32, |max, &x| max.max(x.abs()));
                    // println!("Processing frame {}: Max amplitude: {:.2}", self.frame_count, max_amplitude);
                    
                    self.audio_data = data;
                    let mut planner = FftPlanner::new();
                    let fft = planner.plan_fft_forward(FFT_SIZE);

                    // Apply a window function and prepare data for FFT
                    let mut windowed_data: Vec<Complex<f32>> = self.audio_data
                        .iter()
                        .enumerate()
                        .map(|(i, &x)| {
                            // Apply Hann window instead of Hamming for better frequency resolution
                            let window = 0.5 * (1.0 - (2.0 * std::f32::consts::PI * i as f32 / (FFT_SIZE - 1) as f32).cos());
                            Complex::new(x * window, 0.0)
                        })
                        .collect();

                    fft.process(&mut windowed_data);

                    // Compute magnitude and convert to dB with improved scaling
                    let new_spectrum: Vec<f32> = windowed_data[..FFT_SIZE / 2]
                        .iter()
                        .map(|c| {
                            let magnitude = c.norm() / (FFT_SIZE as f32).sqrt();
                            let db = 20.0 * (magnitude + 1e-10).log10();
                            ((db - MIN_DB) / (MAX_DB - MIN_DB)).max(0.0).min(1.0)
                        })
                        .collect();

                    // Update spectrum with smoothing
                    self.spectrum_data
                        .iter_mut()
                        .zip(new_spectrum.iter())
                        .for_each(|(current, &new)| {
                            *current = SMOOTHING_FACTOR * new + (1.0 - SMOOTHING_FACTOR) * *current;
                        });

                    // Update peak levels
                    self.peak_levels
                        .iter_mut()
                        .zip(self.spectrum_data.iter())
                        .for_each(|(peak, &current)| {
                            *peak = if current > *peak {
                                current
                            } else {
                                (*peak - PEAK_DECAY_RATE).max(current)
                            };
                        });

                    self.cache.clear();
                }
                Command::none()
            }
        }
    }
    

    fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_millis(16))
            .map(|_| Message::Tick)
    }

    fn view(&self) -> Element<Message> {
        Column::new()
            .push(
                Canvas::new(Visualizer {
                    spectrum_data: self.spectrum_data.clone(),
                    peak_levels: self.peak_levels.clone(),
                    frame_count: self.frame_count,
                    cache: &self.cache,
                })
                .width(Length::Fill)
                .height(Length::Fill)
            )
            .into()
    }
}

struct Visualizer<'a> {
    spectrum_data: Vec<f32>,
    peak_levels: Vec<f32>,
    frame_count: usize,
    cache: &'a Cache,
}

impl<'a> canvas::Program<Message> for Visualizer<'a> {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            draw_background(frame, bounds);

            if !self.spectrum_data.is_empty() {
                let width = bounds.width;
                let height = bounds.height;

                draw_frequency_markers(frame, bounds);

                let bar_width = width / self.spectrum_data.len() as f32;
                
                for (i, (&magnitude, &peak)) in self.spectrum_data.iter()
                    .zip(self.peak_levels.iter()).enumerate() {
                    let x = i as f32 * bar_width;
                    
                    let bar_height = magnitude * height * 0.95;
                    let color = frequency_to_color(i as f32 / self.spectrum_data.len() as f32, 
                        magnitude);
                    
                    frame.fill_rectangle(
                        Point::new(x, height),
                        Size::new(bar_width * 0.9, -bar_height),
                        color,
                    );

                    let peak_y = height - (peak * height * 0.95);
                    frame.fill_rectangle(
                        Point::new(x, peak_y),
                        Size::new(bar_width * 0.9, 2.0),
                        Color::from_rgb(1.0, 1.0, 1.0),
                    );
                }
            }
        });

        vec![geometry]
    }
}

fn draw_background(frame: &mut Frame, bounds: Rectangle) {
    for i in 0..20 {
        let progress = i as f32 / 20.0;
        let y = progress * bounds.height;
        let height = bounds.height / 20.0;
        let color = Color::from_rgb(
            0.1 - progress * 0.05,
            0.1 - progress * 0.05,
            0.15 - progress * 0.05,
        );
        
        frame.fill_rectangle(
            Point::new(0.0, y),
            Size::new(bounds.width, height),
            color,
        );
    }
}

fn draw_frequency_markers(frame: &mut Frame, bounds: Rectangle) {
    let marker_frequencies = [50, 100, 200, 500, 1000, 2000, 5000, 10000];
    let height = bounds.height;
    
    for &freq in &marker_frequencies {
        let x = freq_to_x(freq as f32, bounds.width);
        
        frame.fill_rectangle(
            Point::new(x, 0.0),
            Size::new(1.0, height),
            Color::from_rgba(1.0, 1.0, 1.0, 0.2),
        );
        
        let label = if freq >= 1000 {
            format!("{}k", freq / 1000)
        } else {
            format!("{}", freq)
        };
        
        frame.fill_text(Text {
            content: label,
            position: Point::new(x + 2.0, height - 15.0),
            color: Color::from_rgba(1.0, 1.0, 1.0, 0.6),
            size: 12.0,
            ..Text::default()
        });
    }
}

fn freq_to_x(freq: f32, width: f32) -> f32 {
    let min_freq = 20.0;
    let max_freq = SAMPLE_RATE / 2.0;
    let log_pos = (freq / min_freq).log10() / (max_freq / min_freq).log10();
    log_pos * width
}

fn frequency_to_color(position: f32, magnitude: f32) -> Color {
    let intensity = magnitude.max(0.2);
    let hue = position * 360.0;

    let h = hue / 60.0;
    let s = 0.8;
    let v = intensity;

    let c = v * s;
    let x = c * (1.0 - ((h % 2.0) - 1.0).abs());
    let m = v - c;

    let (r, g, b) = match h as i32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        5 => (c, 0.0, x),
        _ => (c, x, 0.0),
    };

    Color::from_rgb(
        (r + m).min(1.0),
        (g + m).min(1.0),
        (b + m).min(1.0),
    )
}

fn main() -> iced::Result {
    println!("Starting Enhanced Spectrum Analyzer...");
    let settings = Settings {
        antialiasing: true,
        ..Settings::default()
    };
    SpectrumAnalyzer::run(settings)
}