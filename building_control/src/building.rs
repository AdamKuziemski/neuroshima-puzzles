use cursive::{
    traits::*,
    views::{Checkbox, Dialog, ListView, ScrollView, NamedView}
};

use serde::{Deserialize, Serialize};

// use chrono::

use std::{fs, io::Write};

use crate::device::Device;

#[derive(Serialize, Deserialize, Clone)]
pub struct Building {
    pub name: String,
    pub password: Option<String>,
    pub devices: Vec<Device>,
    pub manage_mode: ManageMode,
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
        siv.set_user_data(self.clone());

        siv.add_fullscreen_layer(
            Dialog::new()
                .title(format!("{}",  self.name))
                .button("Save", |s| s.with_user_data(|d: &mut Building| {
                    d.save_to_file()
                }).unwrap())
                .button("Quit", |s| s.quit())
                .content(self.create_device_checklist())
                .full_width(),
        );

        siv.run();
    }

    pub fn save_to_file(&mut self) {
        // todo: add {date}-{time} to the file name
        match fs::File::create("./data/saves/save_file.json") {
            Ok(mut file) => {
                let save = serde_json::to_string(self).unwrap();
                file.write_all(save.as_bytes()).unwrap();
            },
            Err(_) => ()
        };
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
        let modify_for_name = device.name.clone();

        // todo: create a table column for each flag

        Checkbox::new()
            .with_checked(device.is_on)
            .on_change(move |siv, value| {
                siv.with_user_data(|building: &mut Building| {
                    let i = building.devices
                        .iter()
                        .position(|d| d.name == modify_for_name)
                        .unwrap();
                    building.devices.get_mut(i).unwrap().is_on = value;
                })
                .unwrap();
            })
            .with_name(device.name.clone())
    }
}
