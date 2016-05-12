#![feature(slice_patterns)]
#![feature(stmt_expr_attributes)]
#![feature(unix_socket)]
extern crate tempdir;
extern crate unix_socket;
extern crate xmz_shift_register;

#[macro_use]
mod common;
mod server;

use server::handle_client;
use std::fs;
use std::os::unix::net::{UnixListener};
use std::path::Path;
use std::thread;
use xmz_shift_register::ShiftRegister;


fn main() {
    #[cfg(target_arch = "arm")]
    {
        let mut leds = ShiftRegister::new_led();
        let mut relais = ShiftRegister::new_relais();

        leds.init();
        relais.init();

        // default configuration
        leds.set(1);
        leds.set(3);
        leds.shift_out();
    }

    let path = Path::new("/tmp");
    let socket_path = path.join(common::SOCKET_PATH);

    // remove socket_path if present
    //
    // first check with fs::metadata if file is present ....
    match fs::metadata(&socket_path) {
        Ok(_) => {
            // ... if present, try to remove the old socket.
            match fs::remove_file(&socket_path) {
                Ok(()) => {},
                Err(err) => println!("Could not remove old socket: {}", err),
            }
        }
        Err(_) => {}
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
