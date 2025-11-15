#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(rustdoc::missing_crate_level_docs)]

mod app;
mod config;
mod fonts;
mod models;
mod quiz_loader;

use app::MyApp;
use config::load_config;
use eframe::egui;

fn main() -> eframe::Result {
    env_logger::init();
    let config = load_config();
    
    // Charger l'icône depuis un fichier
    let icon_data = load_icon();
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([config.window_width, config.window_height])
            .with_icon(icon_data),
        ..Default::default()
    };
    let app_title = config.app_title.clone();
    let quiz_path = config.quiz_path.clone();
    eframe::run_native(
        &app_title,
        options,
        Box::new(move |cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            fonts::setup_fonts(&cc.egui_ctx);
            Ok(Box::new(MyApp::new(&quiz_path)))
        }),
    )
}

fn load_icon() -> egui::IconData {
    // Charger votre fichier d'icône (PNG recommandé)
    let icon_path = "./icon.png";
    
    if let Ok(image_data) = std::fs::read(icon_path) {
        if let Ok(image) = image::load_from_memory(&image_data) {
            let rgba = image.to_rgba8();
            let (width, height) = rgba.dimensions();
            return egui::IconData {
                rgba: rgba.into_raw(),
                width,
                height,
            };
        }
    }
    
    // Icône par défaut si le chargement échoue
    egui::IconData {
        rgba: vec![255; 32 * 32 * 4],
        width: 32,
        height: 32,
    }
}
