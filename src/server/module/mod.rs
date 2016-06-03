use server::sensor::Sensor;

pub struct Module {
    modbus_address: u32,
    sensors: Vec<Sensor>,
}

impl Module {
    pub fn new(modbus_address: u32, sensors: Vec<Sensor>) -> Self {
        Module { modbus_address: modbus_address, sensors: sensors }
    }
}
