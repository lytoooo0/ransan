use crate::widget::block::Block;
use eframe::egui;

pub struct Column {
    name: String,
    blocks: Vec<Block>,
}

impl Column {
    pub fn new() -> Self {
        Self {
            name: String::from("Column 1"),
            blocks: vec![Block::new(String::from("1")), Block::new(String::from("2"))],
        }
    }
    pub fn draw(&mut self, ui: &mut egui::Ui) {
        let available_height = ui.available_height();

        // Measure heading height
        let heading_response = ui.heading(&self.name);
        let heading_height = heading_response.rect.height();

        // Reserve space for button (estimate button height + spacing)
        let button_height =
            ui.spacing().button_padding.y * 2.0 + ui.text_style_height(&egui::TextStyle::Button);
        let spacing = ui.spacing().item_spacing.y;

        // Calculate max height for scroll area
        let max_height = available_height - heading_height - button_height - spacing * 2.0;

        egui::ScrollArea::vertical()
            .max_height(max_height.max(50.0)) // TODO: change 50.0 to more reasonable value
            .show(ui, |ui| {
                for block in &mut self.blocks {
                    block.draw(ui);
                }
            });
        if ui.button("Add").clicked() {
            let num = self.blocks.len() + 1;
            self.blocks.push(Block::new(num.to_string()));
        };
    }
}
