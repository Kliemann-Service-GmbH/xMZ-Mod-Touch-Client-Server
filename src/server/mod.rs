#![feature(slice_patterns)]
use common;
use std::fs;
use std::io::prelude::*;
use std::os::unix::net::{UnixStream, UnixListener};
use std::path::{Path, PathBuf};
use std::thread;
use xmz_shift_register::{ShiftRegister, RegisterType};

pub struct Server {
    pub leds: ShiftRegister,
    pub relais: ShiftRegister,
    pub socket_path: PathBuf,
}

impl Server {
    pub fn new() -> Server {
        Server {
            leds: ShiftRegister::new(RegisterType::LED),
            relais: ShiftRegister::new(RegisterType::RELAIS),
            socket_path: Path::new("/tmp").join(common::SOCKET_PATH),
        }
    }

    pub fn init(&mut self) {
        #[cfg(target_arch = "arm")]
        {
            self.leds.init();
            self.relais.init();
        }
        // default configuration
        self.relais.set(1);
        self.leds.set(1);
        self.leds.set(3);
        #[cfg(target_arch = "arm")]
        {
            self.leds.shift_out();
            self.relais.shift_out();
        }
        self.start_thread();
    }

    fn prepare_socket_path(&self) {
        // remove socket_path if present
        //
        // first check with fs::metadata if file is present ....
        match fs::metadata(&self.socket_path) {
            Ok(_) => {
                // ... if present, try to remove the old socket.
                match fs::remove_file(&self.socket_path) {
                    Ok(()) => {},
                    Err(err) => println!("Could not remove old socket: {}", err),
                }
            }
            Err(_) => {}
        }
    }

    fn start_thread(&self) {
        self.prepare_socket_path();
        let listener = or_panic!(UnixListener::bind(&self.socket_path));
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(|| handle_client(stream));
                }
                Err(err) => {
                    println!("Connection failed! Error: {}", err);
                    break;
                }
            }
        }
        drop(listener);
    }
}


fn handle_client(mut stream: UnixStream) {
    println!("Server handle stream: {:?}", stream);
}
