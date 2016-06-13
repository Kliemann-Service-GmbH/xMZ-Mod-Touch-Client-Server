use sensor::{Sensor, SensorType};

/// Module Arten
///
/// Zur Zeit wird nur eine Art Modul unterstützt
///
/// * RAGAS_CO_NO       - RA-GAS GmbH Kombisensor mit CO und NO Messzelle
#[derive(Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ModuleType {
    RAGAS_CO_NO,
}


/// Sensorplatine mit einem Vector der angeschlossenen Sensoren
pub struct Module<'a> {
    module_type: ModuleType,
    pub sensors: Vec<Sensor<'a>>,
}

impl<'a> Module<'a> {
    /// Erzeugt ein neue Sensorplatine
    ///
    /// # Attribute
    /// * `module_type`         - Art des Moduls
    ///
    /// # Examples
    ///
    /// ```
    /// use playground::module::{Module, ModuleType};
    ///
    /// let module1 = Module::new(ModuleType::RAGAS_CO_NO);
    /// assert_eq!(module1.sensors.len(), 2);
    /// ```
    pub fn new(module_type: ModuleType) -> Self {
        match module_type {
            ModuleType::RAGAS_CO_NO => {
                Module {
                    module_type: ModuleType::RAGAS_CO_NO,
                    sensors: vec![
                        Sensor::new(SensorType::NemotoCO),
                        Sensor::new(SensorType::NemotoNO),
                    ]
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults() {
        let module = Module::new(ModuleType::RAGAS_CO_NO);
        assert_eq!(module.module_type, ModuleType::RAGAS_CO_NO);
    }

}
