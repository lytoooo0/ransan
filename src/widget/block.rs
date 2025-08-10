use chrono::{DateTime, Local};
use eframe::egui;

struct DateSublock {
    title: String,
    time: DateTime<Local>,
}
impl DateSublock {
    pub fn new(name: String) -> Self {
        Self {
            title: name,
            time: chrono::Local::now(),
        }
    }

    pub fn draw(&mut self, ui: &mut egui::Ui) {
        let title =
            self.title.clone() + &String::from(": ") + &self.time.format("%Y-%m-%d").to_string();
        ui.label(title);
    }
}

// TODO: should I add a uuid for each block
pub struct Block {
    title: String,
    note: String,
    created_time: DateSublock,
    planned_time: DateSublock,
    users: Vec<u64>, // Vectors of User IDs
    checked: bool,
}

impl Block {
    pub fn new(name: String) -> Self {
        Self {
            title: String::from("Item") + &name,
            note: String::from("Blank note..."),
            created_time: DateSublock::new(String::from("Created")),
            planned_time: DateSublock::new(String::from("Due")),
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
                    self.created_time.draw(ui);
                    self.planned_time.draw(ui);
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
