use module::{Module};
use zone::{Zone, ZoneType};

/// Server Datenstruktur
pub struct Server<'a> {
    pub zones: Vec<Zone>,
    module: Vec<Module<'a>>,
}

impl<'a> Server<'a> {
    pub fn new() -> Self {
        Server {
            zones: vec![
                Zone::new(ZoneType::STOERUNG),
                Zone::new(ZoneType::SCHWELLENWERT),
            ],
            module: vec![],
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use zone::{ZoneType};
    use module::{Module, ModuleType};

    #[test]
    fn defaults() {
        let server = Server::new();
        assert_eq!(server.module.len(), 0);
    }

    #[test]
    fn zone_defaults() {
        let server = Server::new();
        assert_eq!(server.zones[0].zone_type, ZoneType::STOERUNG);
        assert_eq!(server.zones[1].zone_type, ZoneType::SCHWELLENWERT);
        assert_eq!(server.zones[0].alarmpunkt(0), Some(false));
        assert_eq!(server.zones[0].alarmpunkt(1), None);
        assert_eq!(server.zones[0].alarmpunkt(2), None);
        assert_eq!(server.zones[0].alarmpunkt(3), None);
        assert_eq!(server.zones[1].alarmpunkt(0), Some(false));
        assert_eq!(server.zones[1].alarmpunkt(1), Some(false));
        assert_eq!(server.zones[1].alarmpunkt(2), Some(false));
        assert_eq!(server.zones[1].alarmpunkt(3), Some(false));
    }

    #[test]
    fn zonen_und_module() {
        let mut server = Server::new();
        let module1 = Module::new(ModuleType::RAGAS_CO_NO);
        assert_eq!(server.module.len(), 0);
        server.module.push(module1);
        assert_eq!(server.module.len(), 1);
    }

    #[test]
    fn server_kann_alarmpunkte_setzen() {
        let server = Server::new();
        assert_eq!(server.zones[0].alarmpunkt(0).unwrap(), false);
        server.zones[0].alarmpunkt_set(0, true);
        assert_eq!(server.zones[0].alarmpunkt(0).unwrap(), true);
    }
}
