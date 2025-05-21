#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod font;
mod markdown_table;
use eframe::egui;
use font::setup_custom_fonts;

struct App {
    input_md: String,
    output_tsv: String,
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::viewport::ViewportBuilder::default()
            .with_min_inner_size([800.0, 500.0]),
        ..Default::default()
    };
    eframe::run_native(
        "MD→TSV Converter",
        options,
        Box::new(|cc| {
            // 日本語フォントの設定
            setup_custom_fonts(&cc.egui_ctx);
            Ok(Box::new(App::default()))
        }),
    )
}
