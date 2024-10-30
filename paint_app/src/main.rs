use eframe::egui;
use eframe::App;
use std::f32::consts::PI;

#[derive(PartialEq)]
enum DrawingTool {
    Freehand,
    Line,
    Rectangle,
    Circle,
    Ellipse,
}

#[derive(Clone)]
enum Shape {
    Freehand(Vec<egui::Pos2>),
    Line(egui::Pos2, egui::Pos2),
    Rectangle(egui::Pos2, egui::Pos2),
    Circle(egui::Pos2, f32),
    Ellipse(egui::Pos2, egui::Pos2),
}

#[derive(Default)]
struct MyApp {
    shapes: Vec<Shape>,
    current_tool: DrawingTool,
    current_shape: Option<Shape>,
    stroke_width: f32,
    color: egui::Color32,
}

impl Default for DrawingTool {
    fn default() -> Self {
        DrawingTool::Freehand
    }
}

impl MyApp {
    fn new() -> Self {
        Self {
            shapes: Vec::new(),
            current_tool: DrawingTool::default(),
            current_shape: None,
            stroke_width: 2.0,
            color: egui::Color32::BLACK,
        }
    }

    fn draw_shape(shape: &Shape, painter: &egui::Painter, stroke: egui::Stroke) {
        match shape {
            Shape::Freehand(points) => {
                if points.len() >= 2 {
                    for points in points.windows(2) {
                        painter.line_segment([points[0], points[1]], stroke);
                    }
                }
            }
            Shape::Line(start, end) => {
                painter.line_segment([*start, *end], stroke);
            }
            Shape::Rectangle(start, end) => {
                painter.rect(
                    egui::Rect::from_two_pos(*start, *end),
                    0.0,
                    stroke.color,
                    stroke,
                );
            }
            Shape::Circle(center, radius) => {
                painter.circle(*center, *radius, stroke.color, stroke);
            }
            Shape::Ellipse(start, end) => {
                let rect = egui::Rect::from_two_pos(*start, *end);
                let points: Vec<egui::Pos2> = (0..=32)
                    .map(|i| {
                        let angle = (i as f32) * 2.0 * PI / 32.0;
                        let x = rect.center().x + angle.cos() * rect.width() / 2.0;
                        let y = rect.center().y + angle.sin() * rect.height() / 2.0;
                        egui::pos2(x, y)
                    })
                    .collect();
                
                for points in points.windows(2) {
                    painter.line_segment([points[0], points[1]], stroke);
                }
            }
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top toolbar using panel
        egui::containers::panel::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.current_tool, DrawingTool::Freehand, "✏ Freehand");
                ui.selectable_value(&mut self.current_tool, DrawingTool::Line, "📏 Line");
                ui.selectable_value(&mut self.current_tool, DrawingTool::Rectangle, "⬜ Rectangle");
                ui.selectable_value(&mut self.current_tool, DrawingTool::Circle, "⭕ Circle");
                ui.selectable_value(&mut self.current_tool, DrawingTool::Ellipse, "🔵 Ellipse");
                
                ui.separator();
                
                ui.label("Stroke width:");
                ui.add(egui::Slider::new(&mut self.stroke_width, 1.0..=10.0));
                
                ui.separator();
                
                ui.label("Color:");
                ui.color_edit_button_srgba(&mut self.color);
                
                if ui.button("🗑 Clear").clicked() {
                    self.shapes.clear();
                }
            });
        });

        // Main drawing area
        egui::CentralPanel::default().show(ctx, |ui| {
            let (response, painter) = ui.allocate_painter(
                ui.available_size(),
                egui::Sense::drag(),
            );

            let pointer_pos = response.interact_pointer_pos();

            // Handle drawing
            if let Some(pos) = pointer_pos {
                if response.dragged() {
                    match &mut self.current_shape {
                        Some(Shape::Freehand(points)) => {
                            points.push(pos);
                        }
                        Some(Shape::Line(start, end)) => {
                            *end = pos;
                        }
                        Some(Shape::Rectangle(start, end)) => {
                            *end = pos;
                        }
                        Some(Shape::Circle(center, radius)) => {
                            *radius = center.distance(pos);
                        }
                        Some(Shape::Ellipse(start, end)) => {
                            *end = pos;
                        }
                        None => {
                            // Start new shape
                            self.current_shape = Some(match self.current_tool {
                                DrawingTool::Freehand => Shape::Freehand(vec![pos]),
                                DrawingTool::Line => Shape::Line(pos, pos),
                                DrawingTool::Rectangle => Shape::Rectangle(pos, pos),
                                DrawingTool::Circle => Shape::Circle(pos, 0.0),
                                DrawingTool::Ellipse => Shape::Ellipse(pos, pos),
                            });
                        }
                    }
                }
            }

            // Finalize shape when drag ends
            if response.drag_released() {
                if let Some(shape) = self.current_shape.take() {
                    self.shapes.push(shape);
                }
            }

            // Draw all shapes
            let stroke = egui::Stroke::new(self.stroke_width, self.color);
            
            // Draw completed shapes
            for shape in &self.shapes {
                Self::draw_shape(shape, &painter, stroke);
            }

            // Draw current shape
            if let Some(shape) = &self.current_shape {
                Self::draw_shape(shape, &painter, stroke);
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Enhanced Painting App",
        options,
        Box::new(|_cc| Box::new(MyApp::new())),
    )
}