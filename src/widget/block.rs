use chrono::{DateTime, Datelike, Local, TimeZone};
use eframe::egui::{self, ComboBox};

struct DateSublock {
    _title: String,
    time: DateTime<Local>,
}
impl DateSublock {
    pub fn new(name: String) -> Self {
        Self {
            _title: name,
            time: chrono::Local::now(),
        }
    }

    pub fn draw(&mut self, ui: &mut egui::Ui) {
        let mut cur_year = self.time.year().to_string();
        let mut cur_month = self.time.month().to_string();
        let mut cur_day = self.time.day().to_string();

        // TODO: make the following global values
        let years: Vec<String> = (2025..=2035).map(|year| format!("{:4}", year)).collect();
        let months: Vec<String> = (1..=12).map(|month| format!("{:02}", month)).collect();
        let days: Vec<String> = (1..=31).map(|day| format!("{:02}", day)).collect();
        ComboBox::from_label("Year")
            .selected_text(&cur_year) // TODO: now it doesn't change value cuz it is not stored in the struct but locally
            .show_ui(ui, |ui| {
                for y in years {
                    ui.selectable_value(&mut cur_year, y.clone(), y);
                }
            });
        ComboBox::from_label("Month")
            .selected_text(&cur_month) // TODO: now it doesn't change value cuz it is not stored in the struct but locally
            .show_ui(ui, |ui| {
                for y in months {
                    ui.selectable_value(&mut cur_month, y.clone(), y);
                }
            });
        ComboBox::from_label("Day")
            .selected_text(&cur_day) // TODO: now it doesn't change value cuz it is not stored in the struct but locally
            .show_ui(ui, |ui| {
                for y in days {
                    ui.selectable_value(&mut cur_day, y.clone(), y);
                }
            });
        // Parse the selected date components and update the DateTime
        if let (Ok(year), Ok(month), Ok(day)) = (
            cur_year.parse::<i32>(),
            cur_month.parse::<u32>(),
            cur_day.parse::<u32>(),
        ) {
            if let Some(naive_date) = chrono::NaiveDate::from_ymd_opt(year, month, day) {
                let naive_time = chrono::NaiveTime::from_hms_opt(12, 0, 0).unwrap(); // Default to noon
                let naive_datetime = naive_date.and_time(naive_time);
                if let Some(local_dt) = chrono::Local.from_local_datetime(&naive_datetime).single()
                {
                    self.time = local_dt;
                }
            }
        }
    }
}

// TODO: should I add a uuid for each block
pub struct Block {
    title: String,
    note: String,
    created_time: DateSublock,
    _planned_time: DateSublock,
    users: Vec<u64>, // Vectors of User IDs
    checked: bool,
}

impl Block {
    pub fn new(name: String) -> Self {
        Self {
            title: String::from("Item") + &name,
            note: String::from("Blank note..."),
            created_time: DateSublock::new(String::from("Created")),
            _planned_time: DateSublock::new(String::from("Due")),
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
                    // self.planned_time.draw(ui);
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
