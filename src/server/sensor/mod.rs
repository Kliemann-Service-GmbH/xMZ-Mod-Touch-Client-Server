pub enum SensorType {
    RA_GAS_CO,
    RA_GAS_NO,
}

pub struct Sensor {
    sensor_type: SensorType,
    adc_value: u32,
}

impl Sensor {
    pub fn new(sensor_type: SensorType) -> Self {
        Sensor { sensor_type: sensor_type, adc_value: 0 }
    }
}
