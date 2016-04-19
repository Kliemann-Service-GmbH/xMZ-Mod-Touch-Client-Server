extern crate dbus;
extern crate sysfs_gpio;

mod led;
mod server;

use led::LED;
use server::Server;


fn main() {
    let server = Server::new();
    let mut led = LED::new();
    led.init();

    server.start();
}
