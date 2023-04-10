use cursive::{
    traits::*,
    views::{Checkbox, Dialog, ListView, ScrollView, NamedView}
};

use serde::Deserialize;

use std::fs;

#[derive(Deserialize, Clone)]
pub struct Device {
    pub name: String,
    pub is_on: bool,
}

#[derive(Deserialize, Clone)]
pub struct Building {
    pub name: String,
    pub password: Option<String>,
    pub devices: Vec<Device>
}

impl Building {
    pub fn from_file(file: String) -> Result<Self, serde_json::Error> {
        let path = String::from("./data/buildings/");
        let data = fs::read_to_string(path + &file)
            .expect("Unable to read file");

        serde_json::from_str::<Self>(data.as_str())
    }

    pub fn manage(&self) {
        let mut siv = cursive::default();

        siv.add_fullscreen_layer(
            Dialog::new()
                .title(format!("{}",  self.name))
                .button("Quit", |s| s.quit())
                .content(self.create_device_checklist())
                .full_width(),
        );

        siv.run();
    }

    fn create_device_checklist(&self) -> ScrollView<ListView> {
        ListView::new()
            .with(move |list| {
                for device in &self.devices {
                    list.add_child(
                        device.name.as_str(),
                        Self::create_checkbox(&device)
                    );
                }
            })
            .scrollable()
    }

    fn create_checkbox(device: &Device) -> NamedView<Checkbox> {
        Checkbox::new()
            .with_checked(device.is_on)
            .with_name(device.name.clone())
    }
}
