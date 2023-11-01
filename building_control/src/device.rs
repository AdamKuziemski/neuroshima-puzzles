use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ValueCap {
    pub min: i32,
    pub max: i32
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Sensor {
    pub value: i32,
    pub cap: ValueCap
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Device {
    pub name: String,
    pub is_on: bool,
    pub optics: Option<Sensor>,
    pub thermals: Option<Sensor>,
    pub motion: Option<Sensor>,
}

pub enum SensorType {
    Optics,
    Thermals,
    Motion
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Door {
    pub name: String,
    pub is_on: bool,
    pub is_open: bool
}

pub enum DoorControl {
    On,
    Open
}
