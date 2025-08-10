use eframe::egui;

pub mod widget;

use widget::block::Block;

struct MyApp {
    ui: Block,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { ui: Block::new() }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Set global font size
        ctx.style_mut(|style| {
            style.text_styles.iter_mut().for_each(|(_, font_id)| {
                font_id.size = 24.0; // Increase font size globally
            });
        });

        if ctx.input(|i| i.key_pressed(egui::Key::Q)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui.draw(ui);
        });
    }
}

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Ransan",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}
