use cursive::{
    traits::*,
    views::{Checkbox, Dialog, ListView}
};

use serde::Deserialize;

use std::fs;

#[derive(Deserialize)]
pub struct Device {
    pub name: String,
    pub is_on: bool,
}

#[derive(Deserialize)]
pub struct Building {
    pub name: String,
    pub password: Option<String>,
    pub devices: Vec<Device>
}

impl Building {
    pub fn new(name: String) -> Self {
        Self {
            name,
            password: None,
            devices: vec![]
        }
    }

    pub fn from_file(file: String) -> Result<Self, serde_json::Error> {
        let path = String::from("./data/buildings/");
        let data = fs::read_to_string(path + &file)
            .expect("Unable to read file");

        serde_json::from_str::<Self>(data.as_str())
    }

    pub fn manage(&mut self) {
        let mut siv = cursive::default();

        siv.add_fullscreen_layer(
            Dialog::new()
                .title(format!("{}",  self.name))
                .button("Quit", |s| s.quit())
                .content(
                    ListView::new()
                        .with(move |list| {
                            for device in &self.devices {
                                let mut checkbox = Checkbox::new();
                                let _ = checkbox.set_checked(device.is_on);

                                // checkbox.on_change(|s, checked| {
                                //     device.is_on = checked;
                                // });

                                list.add_child(
                                    device.name.as_str(),
                                    checkbox
                                );
                            }
                        })
                        .scrollable(),
                ).full_width(),
        );
    

        siv.run();
    }
}
