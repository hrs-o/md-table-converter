use crate::{markdown_table, App};
use eframe::egui;

impl Default for App {
    fn default() -> Self {
        Self {
            input_md: String::new(),
            output_tsv: String::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Markdown Table → Excel/Spreadsheet 用 TSV");
            ui.label("① Markdown テーブルを入力:");
            ui.add(
                egui::TextEdit::multiline(&mut self.input_md)
                    .desired_rows(10)
                    .desired_width(f32::INFINITY),
            );

            if ui.button("変換 ▶").clicked() {
                self.output_tsv = markdown_table::to_tsv(&self.input_md);
            }

            ui.separator();
            ui.label("② 生成された TSV をコピー:");
            ui.add(
                egui::TextEdit::multiline(&mut self.output_tsv)
                    .desired_rows(10)
                    .desired_width(f32::INFINITY),
            );
        });
    }
}
