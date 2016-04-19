extern crate dbus;
extern crate sysfs_gpio;

mod shift_register;
mod server;

use shift_register::ShiftRegister;
use server::Server;


fn main() {
    let server = Server::new();
    let mut leds = ShiftRegister::new_led();
    let mut relais = ShiftRegister::new_relais();

    leds.init();
    relais.init();

    server.start();
}
