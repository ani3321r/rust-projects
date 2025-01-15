mod app;
mod category;
mod utils {
    pub mod file_operations;
}

use app::FileOrganizerApp;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_min_inner_size([800.0, 600.0])
            .with_resizable(true),
        ..Default::default()
    };
    
    eframe::run_native(
        "File Organizer",
        options,
        Box::new(|cc| Box::new(FileOrganizerApp::new(cc)))
    )
}