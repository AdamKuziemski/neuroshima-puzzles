use cursive::{
    Cursive,
    traits::*,
    views::{Checkbox, Dialog, ListView, ScrollView, NamedView, LinearLayout, EditView, ResizedView, PaddedView, TextView}, view::Margins
};

use serde::{Deserialize, Serialize};

// use chrono::

use std::{fs, io::Write};

use crate::device::{ Device, Door, SensorType, Sensor, DoorControl };

#[derive(Serialize, Deserialize, Clone)]
pub struct Building {
    pub name: String,
    pub password: Option<String>,
    pub devices: Vec<Device>,
    pub doors: Vec<Door>
}

impl Building {
    pub fn from_file(file: String) -> Result<Self, serde_json::Error> {
        let path = String::from("./data/buildings/");
        let data = fs::read_to_string(path + &file + ".json")
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
                list.add_child(
                    "Device\n",
                    LinearLayout::horizontal()
                        .child(Self::create_header("On"))
                        .child(Self::create_header("Motion"))
                        .child(Self::create_header("Optics"))
                        .child(Self::create_header("Thermals"))
                );

                for device in &self.devices {
                    list.add_child(
                        device.name.as_str(),
                        Self::create_sensor_controls(&device)
                    );
                }

                if self.doors.len() == 0 {
                    return;
                }

                list.add_child(
                    "\nDoor",
                    LinearLayout::horizontal()
                        .child(PaddedView::new(Margins::lrtb(0, 6, 1, 1), TextView::new("On")))
                        .child(PaddedView::new(Margins::lrtb(0, 0, 1, 1), TextView::new("Open")))
                );

                for door in &self.doors {
                    list.add_child(
                        door.name.as_str(),
                        Self::create_door_controls(&door)
                    )
                }
            })
            .scrollable()
    }

    fn create_sensor_controls(device: &Device) -> LinearLayout {
        LinearLayout::horizontal()
            .child(Self::create_sensor_checkbox(device))
            .child(Self::create_input(SensorType::Motion, device.name.clone(), &device.motion))
            .child(Self::create_input(SensorType::Optics, device.name.clone(), &device.optics))
            .child(Self::create_input(SensorType::Thermals, device.name.clone(), &device.thermals))
    }

    fn create_sensor_checkbox(device: &Device) -> PaddedView<NamedView<Checkbox>> {
        let modify_for_name = device.name.clone();

        let checkbox = Checkbox::new()
            .with_checked(device.is_on)
            .on_change(move |siv, value| {
                siv.with_user_data(|building: &mut Building| {
                    let i = Self::get_device_index(building, modify_for_name.clone());
                    building.devices.get_mut(i).unwrap().is_on = value;
                })
                .unwrap();
            })
            .with_name(device.name.clone() + " -- is_on");
        
        PaddedView::new(Margins::lr(0, 5), checkbox)
    }

    fn create_input(sensor_type: SensorType, name: String, sensor: &Option<Sensor>) -> PaddedView<NamedView<ResizedView<EditView>>> {
        let modify_for_name = name.clone();
        let sensor_name = match sensor_type {
            SensorType::Motion => "motion",
            SensorType::Optics => "optics",
            SensorType::Thermals => "thermals",
        };

        let input = EditView::new();
        let input = match sensor {
            Some(s) => input.content(s.value.to_string())
                .on_edit_mut(move |siv: &mut Cursive, text: &str, _cursor: usize| {
                    siv.with_user_data(|building: &mut Building| {
                        let i = Self::get_device_index(building, modify_for_name.clone());
                        let device = building.devices.get_mut(i).unwrap();

                        match sensor_type {
                            SensorType::Motion => device.motion.as_mut().unwrap().value = text.parse().unwrap_or(0),
                            SensorType::Optics => device.optics.as_mut().unwrap().value = text.parse().unwrap_or(0),
                            SensorType::Thermals => device.thermals.as_mut().unwrap().value = text.parse().unwrap_or(0),
                        }
                    })
                    .unwrap();
            }),
            None => input.disabled()
        };

        let input = input
            .fixed_width(4)
            .with_name(name.clone() + " -- " + sensor_name);

        PaddedView::new(Margins::lr(0, 4), input)

    }

    fn create_door_controls(door: &Door) -> LinearLayout {
        LinearLayout::horizontal()
            .child(Self::create_door_checkbox(door, DoorControl::On))
            .child(Self::create_door_checkbox(door, DoorControl::Open))
    }

    fn create_door_checkbox(door: &Door, control_type: DoorControl) -> PaddedView<NamedView<Checkbox>> {
        let modify_for_name = door.name.clone();
        let initial_value = match control_type {
            DoorControl::On => door.is_on,
            DoorControl::Open => door.is_open
        };
        let control_name = match control_type {
            DoorControl::On => "is_on",
            DoorControl::Open => "is_open"
        };

        let checkbox = Checkbox::new()
            .with_checked(initial_value)
            .on_change(move |siv, value| {
                siv.with_user_data(|building: &mut Building| {
                    let i = Self::get_door_index(building, modify_for_name.clone());
                    let door = building.doors.get_mut(i).unwrap();

                    match control_type {
                        DoorControl::On => door.is_on = value,
                        DoorControl::Open => door.is_open = value
                    }
                })
                .unwrap();
            })
            .with_name(door.name.clone() + " -- " + control_name);
        
        PaddedView::new(Margins::lr(0, 5), checkbox)
    }

    fn get_device_index(building: &Building, name: String) -> usize {
        building.devices
            .iter()
            .position(|d| d.name == name)
            .unwrap()
    }

    fn get_door_index(building: &Building, name: String) -> usize {
        building.doors
            .iter()
            .position(|d| d.name == name)
            .unwrap()
    }

    fn create_header(name: &str) -> PaddedView<TextView> {
        let width = 8 - name.len();
        PaddedView::new(Margins::lrtb(0, width, 0, 1), TextView::new(name))        
    }
}
