use crate::window::GameWindow;
use crate::music::MusicPlayer;
use eframe::{App, NativeOptions};
use egui::ViewportBuilder;
use egui_extras::install_image_loaders;
use walkdir::WalkDir;
use eframe::egui::{FontDefinitions, FontFamily};

mod window;
mod music;



fn main() -> Result<(), eframe::Error> {
    MusicPlayer::play_music_on_startup(r"Assets/Music/C4HTrack.mp3");

    let options = NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([1366.0, 768.0]),
        ..Default::default()
    };

    let mut image_paths = WalkDir::new(r"Assets/Pictures")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .filter_map(|entry| entry.path().to_str().map(str::to_string))
        .collect::<Vec<_>>();

    if let Some(first_path) = image_paths.first().cloned() {
        if let Some(pos) = image_paths.iter().position(|path| *path == first_path) {
            image_paths.remove(pos);
            image_paths.insert(0, first_path);
        }
    }

    eframe::run_native("C4H", options, Box::new(|cc| {
        install_image_loaders(&cc.egui_ctx);


        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "my_font".to_owned(), 
            egui::FontData::from_static(include_bytes!(r"../Assets/Font/pixel.ttf"))
        );
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "my_font".to_owned()); 
        cc.egui_ctx.set_fonts(fonts);

        let game_window = GameWindow::new(&cc.egui_ctx, image_paths);
        Ok(Box::new(game_window) as Box<dyn App>)
    }))
}
   
