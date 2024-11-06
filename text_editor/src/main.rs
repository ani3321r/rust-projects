use std::path::PathBuf;
use std::fs;
use std::io::{self, Write};
use std::collections::{HashMap, VecDeque};
use eframe::egui;
use rfd::FileDialog;
use thiserror::Error;

mod config;
use config::Config;

#[derive(Debug, Clone, Copy)]
struct Cursor {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone)]
struct Selection {
    start: Cursor,
    end: Cursor,
}

#[derive(Debug)]
enum Mode {
    Normal,
    Search,
    Replace,
    Visual,
    Input,
}

#[derive(Debug, Clone)]
struct SearchMatch {
    row: usize,
    range: std::ops::Range<usize>,
}

#[derive(Debug, Clone)]
struct SearchState {
    query: String,
    matches: Vec<SearchMatch>,
    current_match: Option<usize>,
}

#[derive(Clone)]
struct Buffer {
    file_path: Option<PathBuf>,
    content: String,
    modified: bool,
    cursor: Cursor,
    screen_offset: usize,
    selection: Option<Selection>,
    search_state: Option<SearchState>,
    undo_stack: VecDeque<BufferState>,
    redo_stack: VecDeque<BufferState>,
}

#[derive(Clone)]
struct BufferState {
    content: String,
    cursor: Cursor,
}

impl Buffer {
    fn new() -> Self {
        Buffer {
            file_path: None,
            content: String::new(),
            cursor: Cursor { row: 0, col: 0 },
            modified: false,
            screen_offset: 0,
            selection: None,
            search_state: None,
            undo_stack: VecDeque::with_capacity(100),
            redo_stack: VecDeque::new(),
        }
    }

    fn save_state(&mut self) {
        let state = BufferState {
            content: self.content.clone(),
            cursor: self.cursor,
        };
        self.undo_stack.push_back(state);
        if self.undo_stack.len() > 100 {
            self.undo_stack.pop_front();
        }
        self.redo_stack.clear();
    }

    fn undo(&mut self) {
        if let Some(state) = self.undo_stack.pop_back() {
            let current_state = BufferState {
                content: self.content.clone(),
                cursor: self.cursor,
            };
            self.redo_stack.push_back(current_state);
            self.content = state.content;
            self.cursor = state.cursor;
            self.modified = true;
        }
    }

    fn redo(&mut self) {
        if let Some(state) = self.redo_stack.pop_back() {
            let current_state = BufferState {
                content: self.content.clone(),
                cursor: self.cursor,
            };
            self.undo_stack.push_back(current_state);
            self.content = state.content;
            self.cursor = state.cursor;
            self.modified = true;
        }
    }
}

struct TextEditor {
    buffers: HashMap<usize, Buffer>,
    current_buffer: usize,
    next_buffer_id: usize,
    config: Config,
}

#[derive(Error, Debug)]
pub enum EditorError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("No buffer selected")]
    NoBufferSelected,
    #[error("No buffer available")]
    NoBufferAvailable,
    #[error("Cannot close last buffer")]
    CannotCloseLastBuffer,
    #[error("Operation cancelled")]
    OperationCancelled,
}

impl TextEditor {
    fn new() -> Result<Self, EditorError> {
        let config = Config::load().map_err(EditorError::Io)?;
        let mut editor = TextEditor {
            buffers: HashMap::new(),
            current_buffer: 0,
            next_buffer_id: 0,
            config,
        };
        editor.create_buffer();
        Ok(editor)
    }

    fn create_buffer(&mut self) -> usize {
        let buffer_id = self.next_buffer_id;
        self.buffers.insert(buffer_id, Buffer::new());
        self.next_buffer_id += 1;
        buffer_id
    }

    fn current_buffer(&self) -> Result<&Buffer, EditorError> {
        self.buffers.get(&self.current_buffer)
            .ok_or(EditorError::NoBufferSelected)
    }

    fn current_buffer_mut(&mut self) -> Result<&mut Buffer, EditorError> {
        self.buffers.get_mut(&self.current_buffer)
            .ok_or(EditorError::NoBufferSelected)
    }
}

impl eframe::App for TextEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() {
                        let buffer_id = self.create_buffer();
                        self.current_buffer = buffer_id;
                    }
                    
                    if ui.button("Open").clicked() {
                        if let Some(path) = FileDialog::new()
                            .add_filter("Text files", &["txt"])
                            .add_filter("All files", &["*"])
                            .pick_file() 
                        {
                            if let Ok(content) = fs::read_to_string(&path) {
                                if let Ok(buffer) = self.current_buffer_mut() {
                                    buffer.content = content;
                                    buffer.file_path = Some(path.clone());
                                    buffer.modified = false;
                                }
                            }
                        }
                    }
                    
                    if ui.button("Save").clicked() {
                        if let Ok(buffer) = self.current_buffer_mut() {
                            if let Some(path) = &buffer.file_path {
                                if fs::write(path, &buffer.content).is_ok() {
                                    buffer.modified = false;
                                }
                            } else {
                                if let Some(path) = FileDialog::new()
                                    .add_filter("Text files", &["txt"])
                                    .add_filter("All files", &["*"])
                                    .save_file()
                                {
                                    if fs::write(&path, &buffer.content).is_ok() {
                                        buffer.file_path = Some(path);
                                        buffer.modified = false;
                                    }
                                }
                            }
                        }
                    }
                });

                ui.menu_button("Edit", |ui| {
                    if ui.button("Undo").clicked() {
                        if let Ok(buffer) = self.current_buffer_mut() {
                            buffer.undo();
                        }
                    }
                    if ui.button("Redo").clicked() {
                        if let Ok(buffer) = self.current_buffer_mut() {
                            buffer.redo();
                        }
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Ok(buffer) = self.current_buffer_mut() {
                let title = if let Some(path) = &buffer.file_path {
                    format!("{}{}", path.display(), if buffer.modified { "*" } else { "" })
                } else {
                    String::from("Untitled")
                };
                ui.heading(&title);

                let response = ui.add_sized(
                    ui.available_size(),
                    egui::TextEdit::multiline(&mut buffer.content)
                        .desired_width(f32::INFINITY)
                        .desired_rows(0)
                        .font(egui::TextStyle::Monospace)
                );

                if response.changed() {
                    buffer.modified = true;
                }
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Text Editor",
        options,
        Box::new(|_cc| Box::new(TextEditor::new().unwrap()))
    )
}