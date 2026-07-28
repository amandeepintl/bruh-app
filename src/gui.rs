use eframe::{egui, App};
use std::path::{Path, PathBuf};

pub struct ImageViewer {
    image_path: PathBuf,
    texture: Option<egui::TextureHandle>,
}

impl ImageViewer {
    pub fn new(path: &Path) -> Result<Self, String> {
        let image = image::open(path).map_err(|e| format!("failed to load image: {e}"))?;
        let rgba = image.to_rgba8();
        let size = [rgba.width() as usize, rgba.height() as usize];
        let pixels = rgba.into_raw();
        Ok(Self {
            image_path: path.to_path_buf(),
            texture: None,
        })
    }
}

impl App for ImageViewer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(self.image_path.display().to_string());
            ui.label("Preview is available in the GUI build.");
        });
    }
}

pub fn show(path: &Path) -> Result<(), String> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(640.0, 480.0)),
        ..Default::default()
    };
    eframe::run_native(
        "bruh preview",
        options,
        Box::new(|_| Box::new(ImageViewer::new(path).unwrap())),
    )
    .map_err(|e| format!("gui error: {e}"))
}
