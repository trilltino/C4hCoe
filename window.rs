use eframe::egui::{CentralPanel, Context, Frame, Vec2, Layout, Align, RichText, TopBottomPanel, ScrollArea};
use crate::App;
use egui::{ColorImage, TextureOptions};
use std::collections::HashMap;
use walkdir::WalkDir;
use regex::Regex; 

#[derive(Clone)]
pub struct GameWindow {
    pub output_text: String,
    pub textures: HashMap<String, eframe::egui::TextureHandle>,
    pub image_paths: Vec<String>,
    pub current_image_index: usize,
    pub image_requirements: HashMap<String, String>,
}

impl Default for GameWindow {
    fn default() -> Self {
        Self {
            output_text: String::new(),
            textures: HashMap::new(),
            image_paths: Vec::new(),
            current_image_index: 0,
            image_requirements: HashMap::new(),
        }
    }
}

impl GameWindow {
    pub fn new(ctx: &Context, image_paths: Vec<String>) -> GameWindow {
        let mut game_window = GameWindow {
            textures: HashMap::new(),
            image_paths,
            current_image_index: 0,
            output_text: String::new(),
            image_requirements: HashMap::new(),
        };

        game_window.load_images_from_directory(ctx, "Assets/Pictures")
            .unwrap_or_else(|err| eprintln!("Error preloading images from Int directory: {}", err));

        if let Some(initial_image_path) = game_window.image_paths.get(0).cloned() {
            game_window.load_image(ctx, &initial_image_path)
                .unwrap_or_else(|err| eprintln!("Error loading initial image: {}", err));
        }

        game_window
    }

    fn load_images_from_directory(&mut self, ctx: &Context, directory: &str) -> Result<(), String> {
        for entry in WalkDir::new(directory)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file()) {
            if let Some(path_str) = entry.path().to_str() {
                self.load_image(ctx, path_str)?;
                self.image_paths.push(path_str.to_string());
            }
        }

        self.image_paths.sort_by_key(|path| {
            let re = Regex::new(r"\d+").unwrap();
            re.find_iter(path)
                .last()
                .map(|m| m.as_str().parse::<u32>().unwrap_or(0))
                .unwrap_or(0)
        });

        Ok(())
    }

    fn load_image(&mut self, ctx: &Context, image_path: &str) -> Result<(), String> {
        let image = image::open(image_path)
            .map_err(|err| format!("Failed to open image {}: {}", image_path, err))?
            .to_rgba8();

        let size = [image.width() as usize, image.height() as usize];
        let pixels = image.into_raw();
        let color_image = ColorImage::from_rgba_unmultiplied(size, &pixels);
        let texture = ctx.load_texture(image_path, color_image, TextureOptions::default());

        self.textures.insert(image_path.to_string(), texture);
        Ok(())
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.label("Double tap A and D to cycle through the story, ");
            ui.horizontal(|_ui| {
            });
        });

        CentralPanel::default()
        .frame(Frame::default())
        .show(ctx, |ui| {
            let panel_width = ui.available_width();
            let panel_height = ui.available_height();
            ui.set_min_size(Vec2::new(panel_width, panel_height));
    
            let current_image_path = self.image_paths.get(self.current_image_index).cloned();
    
            if let Some(image_path) = &current_image_path {
                let textures = self.textures.clone();
    
                match textures.get(image_path) {
                    Some(texture) => {
                        ScrollArea::vertical().show(ui, |ui| {
                            ui.centered_and_justified(|ui| {
                                ui.image(texture,); 
                            });
                        });
                    },
                    None => {
                        ui.label("Loading image or error occurred...");
                    }
                }
            } else {
                ui.label("No image available.");
            }
    
            ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
                ui.label(RichText::new(&self.output_text).monospace());
            });
        });
    
    if ctx.input(|i| i.key_pressed(egui::Key::D)) {
        if let Some(current_image_path) = self.image_paths.get(self.current_image_index) {
            if !self.image_requirements.contains_key(current_image_path) {
                self.current_image_index = (self.current_image_index + 1) % self.image_paths.len();
                ctx.request_repaint();
            }
        }
    }
    
    if ctx.input(|i| i.key_pressed(egui::Key::A)) {
        self.current_image_index = (self.current_image_index + self.image_paths.len() - 1) % self.image_paths.len();
        ctx.request_repaint();
    }
    
    impl App for GameWindow {
        fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
            self.update(ctx, frame);
        }
    }
    }
}
 
    



