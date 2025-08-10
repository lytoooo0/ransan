use chrono::{DateTime, Duration, Local};
use eframe::egui;

// TODO: should I add a uuid for each block
pub struct Block {
    title: String,
    note: String,
    created_time: DateTime<Local>,
    planned_time: DateTime<Local>,
    users: Vec<u64>, // Vectors of User IDs
    checked: bool,
}

impl Block {
    pub fn new(name: String) -> Self {
        let current_time = chrono::Local::now();
        Self {
            title: String::from("Item") + &name,
            note: String::from("Blank note..."),
            created_time: current_time,
            planned_time: current_time + Duration::days(7),
            users: vec![123, 456],
            checked: false,
        }
    }

    pub fn draw(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.checkbox(&mut self.checked, "").changed() {
                if self.checked {
                    println!("checked");
                } else {
                    println!("unchecked");
                }
            }
            egui::CollapsingHeader::new(&self.title)
                .default_open(true)
                .show(ui, |ui| {
                    ui.label(self.note.clone());
                    ui.label(
                        String::from("Created: ")
                            + &self.created_time.format("%Y-%m-%d").to_string(),
                    );
                    ui.label(
                        String::from("Due: ")
                            + &self.planned_time.format("%Y-%m-%d %H:%M").to_string(),
                    );
                    // TODO: unify icon size
                    let mut user_str = String::from("User: ");

                    for user in &self.users {
                        user_str = user_str + " " + &user.to_string();
                    }
                    ui.label(user_str);
                });
        });
    }
}
