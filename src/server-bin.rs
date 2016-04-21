extern crate sysfs_gpio;

mod shift_register;
mod server;

use shift_register::ShiftRegister;


fn main() {
    let mut leds = ShiftRegister::new_led();
    let mut relais = ShiftRegister::new_relais();

    leds.init();
    relais.init();
}
