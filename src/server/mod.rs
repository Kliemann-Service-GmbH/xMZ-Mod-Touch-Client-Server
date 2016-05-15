#![feature(slice_patterns)]
use nanomsg::{Socket, Protocol};
use std::thread;
use xmz_shift_register::{ShiftRegister, RegisterType};
use std::io::Read;

pub struct Server {
    pub leds: ShiftRegister,
    pub relais: ShiftRegister,
}

impl Server {
    pub fn new() -> Server {
        Server {
            leds: ShiftRegister::new(RegisterType::LED),
            relais: ShiftRegister::new(RegisterType::RELAIS),
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


    fn start_thread(&self) {
        let mut socket = Socket::new(Protocol::Pull).unwrap();
        let mut command = String::new();
        socket.bind("ipc:///tmp/pipeline_collector.ipc");

        loop {
            match socket.read_to_string(&mut command) {
                Ok(_) => self.handle_client(&command),
                Err(err) => {
                    println!("Server failure: {}", err);
                    break
                }
            }
            command.clear();
        }
    }

    fn handle_client(&self, command: &str) {
        println!("Command: {}", command);
    }
}
