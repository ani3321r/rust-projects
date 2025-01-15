use crate::category::Category;
use crate::utils::file_operations::{create_directory_if_not_exists, get_file_extension, move_file};
use eframe::egui;
use rfd::FileDialog;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Default)]
pub struct FileOrganizerApp {
    categories: Vec<Category>,
    files_to_organize: Vec<PathBuf>,
    new_category_name: String,
    new_extension: String,
    status_message: String,
    operation_log: Vec<(PathBuf, PathBuf)>,
    show_add_category: bool,
    current_color: [f32; 3],
    is_drop_hover: bool,
}

impl FileOrganizerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let categories = if let Ok(data) = fs::read_to_string("categories.json") {
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            vec![
                Category {
                    name: "Documents".to_string(),
                    extensions: vec!["pdf", "doc", "docx", "txt"].iter().map(|s| s.to_string()).collect(),
                    color: [0.2, 0.6, 1.0],
                },
                Category {
                    name: "Images".to_string(),
                    extensions: vec!["jpg", "png", "gif"].iter().map(|s| s.to_string()).collect(),
                    color: [0.8, 0.2, 0.2],
                },
            ]
        };

        Self {
            categories,
            current_color: [0.5, 0.5, 0.5],
            is_drop_hover: false,
            ..Default::default()
        }
    }

    fn save_categories(&self) {
        if let Ok(json) = serde_json::to_string_pretty(&self.categories) {
            let _ = fs::write("categories.json", json);
        }
    }

    fn organize_files(&mut self) {
        let mut successful_moves = 0;
        let total_files = self.files_to_organize.len();

        for file_path in &self.files_to_organize {
            if let Some(extension) = get_file_extension(file_path) {
                for category in &self.categories {
                    if category.extensions.iter().any(|e| e.to_lowercase() == extension.to_lowercase()) {
                        let target_dir = file_path.parent().unwrap_or(Path::new(".")).join(&category.name);
                        if let Err(_) = create_directory_if_not_exists(&target_dir) {
                            self.status_message = format!("Failed to create directory: {}", target_dir.display());
                            continue;
                        }
                        
                        let new_path = target_dir.join(file_path.file_name().unwrap());
                        match move_file(file_path, &new_path) {
                            Ok(_) => {
                                self.operation_log.push((file_path.clone(), new_path));
                                successful_moves += 1;
                            }
                            Err(e) => {
                                self.status_message = format!("Failed to move file: {}", e);
                            }
                        }
                        break;
                    }
                }
            }
        }

        if successful_moves > 0 {
            self.status_message = format!("Successfully organized {}/{} files!", successful_moves, total_files);
            self.files_to_organize.clear();
        } else if total_files > 0 {
            self.status_message = "No files were organized. Check if the file extensions match any categories.".to_string();
        }
    }

    fn undo_last_operation(&mut self) {
        if let Some((original, moved)) = self.operation_log.pop() {
            if moved.exists() {
                if let Ok(_) = move_file(&moved, &original) {
                    self.status_message = "Last operation undone!".to_string();
                    return;
                }
            }
        }
        self.status_message = "Nothing to undo".to_string();
    }
}

impl eframe::App for FileOrganizerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("categories_panel")
            .resizable(true)
            .min_width(200.0)
            .show(ctx, |ui| {
                ui.heading("Categories");
                
                if ui.button("Add New Category").clicked() {
                    self.show_add_category = true;
                }

                ui.separator();

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for category in &self.categories {
                        ui.horizontal(|ui| {
                            let color = egui::Color32::from_rgb(
                                (category.color[0] * 255.0) as u8,
                                (category.color[1] * 255.0) as u8,
                                (category.color[2] * 255.0) as u8,
                            );
                            ui.colored_label(color, &category.name);
                            ui.label(format!("({})", category.extensions.join(", ")));
                        });
                    }
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("File Organizer");
            
            // Drag and drop area with visual feedback
            ui.add_space(10.0);
            let drop_zone = egui::Frame::none()
                .stroke(egui::Stroke::new(
                    if self.is_drop_hover { 2.0 } else { 1.0 },
                    if self.is_drop_hover {
                        egui::Color32::LIGHT_BLUE
                    } else {
                        egui::Color32::GRAY
                    },
                ))
                .fill(if self.is_drop_hover {
                    egui::Color32::from_rgba_premultiplied(100, 100, 255, 25)
                } else {
                    egui::Color32::from_gray(32)
                })
                .inner_margin(egui::style::Margin::same(10.0))
                .show(ui, |ui| {
                    ui.allocate_space(egui::vec2(ui.available_width(), 80.0));
                    ui.centered_and_justified(|ui| {
                        ui.label(
                            if self.is_drop_hover {
                                "Drop files here!"
                            } else {
                                "Drag and drop files here or click to select"
                            }
                        );
                    });
                });

            // Handle drag & drop
            ctx.input(|i| {
                // Update hover state
                self.is_drop_hover = !i.raw.hovered_files.is_empty();
                
                // Handle dropped files
                if !i.raw.dropped_files.is_empty() {
                    let new_files: Vec<PathBuf> = i.raw.dropped_files
                        .iter()
                        .filter_map(|f| f.path.clone())
                        .collect();
                    
                    if !new_files.is_empty() {
                        self.files_to_organize.extend(new_files);
                        self.status_message = format!("Added {} files for organizing", i.raw.dropped_files.len());
                        ctx.request_repaint(); // Ensure UI updates immediately
                    }
                }
            });

            ui.add_space(10.0);
            if ui.button("Select Files").clicked() {
                if let Some(paths) = FileDialog::new().pick_files() {
                    self.files_to_organize.extend(paths.clone());
                    self.status_message = format!("Added {} files for organizing", paths.len());
                }
            }

            ui.add_space(10.0);
            egui::ScrollArea::vertical()
                .max_height(200.0)
                .show(ui, |ui| {
                    ui.heading("Selected Files:");
                    if self.files_to_organize.is_empty() {
                        ui.label("No files selected");
                    } else {
                        for path in &self.files_to_organize {
                            ui.label(path.file_name().unwrap_or_default().to_string_lossy().to_string());
                        }
                    }
                });

            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if ui.add_enabled(
                    !self.files_to_organize.is_empty(),
                    egui::Button::new("Organize Files")
                ).clicked() {
                    self.organize_files();
                }
                if ui.add_enabled(
                    !self.operation_log.is_empty(),
                    egui::Button::new("Undo Last Operation")
                ).clicked() {
                    self.undo_last_operation();
                }
            });

            ui.add_space(10.0);
            if !self.status_message.is_empty() {
                ui.colored_label(
                    if self.status_message.contains("Failed") {
                        egui::Color32::RED
                    } else {
                        egui::Color32::GREEN
                    },
                    &self.status_message
                );
            }
        });

        if self.show_add_category {
            egui::Window::new("Add Category")
                .resizable(false)
                .collapsible(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Name:");
                        ui.text_edit_singleline(&mut self.new_category_name);
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("Extension:");
                        ui.text_edit_singleline(&mut self.new_extension);
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("Color:");
                        ui.color_edit_button_rgb(&mut self.current_color);
                    });

                    ui.horizontal(|ui| {
                        if ui.button("Add").clicked() && !self.new_extension.is_empty() {
                            let category = Category {
                                name: self.new_category_name.clone(),
                                extensions: vec![self.new_extension.clone()],
                                color: self.current_color,
                            };
                            self.categories.push(category);
                            self.save_categories();
                            self.new_category_name.clear();
                            self.new_extension.clear();
                            self.show_add_category = false;
                        }
                        if ui.button("Cancel").clicked() {
                            self.show_add_category = false;
                        }
                    });
                });
        }
    }
}