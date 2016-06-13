
use zone::{Zone};

/// Sensortyp, Art der Messzelle
#[derive(Debug, Eq, PartialEq)]
pub enum SensorType {
    NemotoCO,
    NemotoNO,
}

/// Representiert eine Sensor Messzelle
///
/// Jeder Sensor kann keiner, einer oder mehreren Zonen (Alarmzonen) zugeordnet werden.
pub struct Sensor<'a> {
    sensor_type: SensorType,
    adc_value: u32,
    zones: Vec<&'a Zone>,
}

impl<'a> Sensor<'a> {
    pub fn new(sensor_type: SensorType) -> Self {
        match sensor_type {
            SensorType::NemotoCO => Sensor {
                sensor_type: SensorType::NemotoCO,
                adc_value: 0,
                zones: vec!(),
            },
            SensorType::NemotoNO => Sensor {
                sensor_type: SensorType::NemotoNO,
                adc_value: 0,
                zones: vec!(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use server::Server;

    #[test]
    fn defaults() {
        let sensor = Sensor::new(SensorType::NemotoCO);
        assert_eq!(sensor.adc_value, 0);
    }

    #[test]
    fn sensor_nemoto_co() {
        let sensor = Sensor::new(SensorType::NemotoCO);
        assert_eq!(sensor.sensor_type, SensorType::NemotoCO);
    }

    #[test]
    fn sensor_mit_keiner_zone() {
        let sensor = Sensor::new(SensorType::NemotoCO);
        assert_eq!(sensor.zones.len(), 0);
    }

    #[test]
    fn sensor_mit_einer_zone() {
        let server = Server::new();
        let mut sensor = Sensor::new(SensorType::NemotoCO);
        sensor.zones.push(&server.zones[0]);
        assert_eq!(sensor.zones.len(), 1);
    }

    #[test]
    fn sensor_mit_mehr_als_einer_zone() {
        let server = Server::new();
        let mut sensor = Sensor::new(SensorType::NemotoCO);
        sensor.zones.push(&server.zones[0]);
        sensor.zones.push(&server.zones[1]);
        assert_eq!(sensor.zones.len(), 2);
    }

    #[test]
    fn sensor_mit_einer_zone_kann_alarmpunkt_setzen() {
        let mut server = Server::new();
        let mut sensor = Sensor::new(SensorType::NemotoCO);
        sensor.zones.push(&mut server.zones[0]);
        assert_eq!(sensor.zones[0].alarmpunkt(0).unwrap(), false);
        // Setze den ersten Alarmpunkt der ersten Zone
        sensor.zones[0].alarmpunkt_set(0, true);
        assert_eq!(sensor.zones[0].alarmpunkt(0).unwrap(), true);
    }
    #[test]
    fn sensor_mit_mehr_als_einer_zone_kann_alarmpunkt_setzen() {
        let server = Server::new();
        let mut sensor = Sensor::new(SensorType::NemotoCO);
        sensor.zones.push(&server.zones[0]);
        sensor.zones.push(&server.zones[1]);
        assert_eq!(sensor.zones[0].alarmpunkt(0).unwrap(), false);
        assert_eq!(sensor.zones[1].alarmpunkt(0).unwrap(), false);
        // Setze den ersten Alarmpunkt der ersten Zone
        sensor.zones[0].alarmpunkt_set(0, true);
        // Setze den ersten Alarmpunkt der zweiten Zone
        sensor.zones[1].alarmpunkt_set(0, true);
        assert_eq!(sensor.zones[0].alarmpunkt(0).unwrap(), true);
        assert_eq!(sensor.zones[1].alarmpunkt(0).unwrap(), true);
    }

}
