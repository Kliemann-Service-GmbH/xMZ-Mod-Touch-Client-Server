#![feature(stmt_expr_attributes)]
#![feature(unix_socket)]
extern crate tempdir;
extern crate sysfs_gpio;
extern crate unix_socket;

#[macro_use]
mod common;
mod shift_register;
mod server;

use shift_register::ShiftRegister;
use std::fs;
use std::os::unix::net::{UnixListener};
use std::thread;
use std::path::Path;
use server::handle_client;

fn main() {
    let leds = ShiftRegister::new_led();
    let relais = ShiftRegister::new_relais();

    #[cfg(target_arch = "arm")]
    leds.init();
    #[cfg(target_arch = "arm")]
    relais.init();

    let path = Path::new("/tmp");
    let socket_path = path.join(common::SOCKET_PATH);

    // remove socket_path if present
    match fs::remove_file(&socket_path) {
        Ok(()) => {},
        Err(err) => println!("Could not remove old socket: {}", err),
    }

    let listener = or_panic!(UnixListener::bind(&socket_path));
    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // connection succeeded
                thread::spawn(|| handle_client(stream));
            }
            Err(err) => {
                // connection failed
                println!("Connection failed! Error: {}", err);
                break;
            }
        }
    }
    // close listener connection
    drop(listener);

}
