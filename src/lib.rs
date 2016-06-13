#![doc(html_logo_url = "https://raw.githubusercontent.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server/master/share/xmz-logo.png",
       html_favicon_url = "https://raw.githubusercontent.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server/master/share/favicon.ico",
       html_root_url = "https://gaswarnanlagen.com/")]

/// Shift register, Zugriff und Steuerung der an die Shift Register angeschlossenen Hardware
pub mod shift_register;
/// Zonen, Alarmzonen wie Störung und Schwellenwert Auswertungen
pub mod zone;
/// Sensoren, Messzellen auf den Sensorplatinen (Module)
pub mod sensor;
/// Module, Sensorplatinen mit Modbus Transceiver und einem oder mehreren Sensoren
pub mod module;
/// Server, Kernkomponente der 'xMZ-Mod-Touch'-Plattform
///
/// Der Server verbindet alle Komponenten untereinander und sorgt für deren Zusammenspiel.
pub mod server;
