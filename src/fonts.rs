use eframe::egui;
use std::fs;

pub fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    
    let font_paths = vec![
        "./fonts/MPLUSRounded1c-Regular.ttf",
        "./fonts/NotoSansSymbols2-Regular.ttf",
    ];
    
    for font_path in font_paths {
        if let Ok(buf) = fs::read(font_path) {
            let font_name = "system_font".to_owned();
            fonts.font_data.insert(
                font_name.clone(),
                std::sync::Arc::new(egui::FontData::from_owned(buf)),
            );
            fonts.families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, font_name.clone());
            fonts.families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .insert(0, font_name);
            break;
        }
    }
    
    ctx.set_fonts(fonts);
}
