use nanomsg::{Socket, Protocol};
use server::server_command::ServerCommand;
use std::io::Read;
use std::thread;
use xmz_shift_register::{ShiftRegister, RegisterType};


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
        // TODO: Export to a Config file /dev/mmcblk0p1/xMZ-Mod-Touch-Configuration.toml
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


    fn start_thread(&mut self) {
        let mut socket = Socket::new(Protocol::Pull).unwrap();
        let mut endpoint = socket.bind("ipc:///tmp/pipeline_collector.ipc").unwrap();
        let mut command = String::new();

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
        match endpoint.shutdown() {
            Ok(_) => {}
            Err(err) => { panic!("{}", err); }
        }
    }

    fn handle_client(&mut self, cmd: &str) {
        let split: Vec<_> = cmd.split_whitespace().collect();
        match &split[..] {
            [command, action, value] => {
                match command {
                    "leds" => {
                        match action {
                            "get" => {
                                let value: u64 = value.parse().unwrap();
                                self.leds.get(value);
                                self.leds.shift_out();

                            },
                            "set" => {
                                let value: u64 = value.parse().unwrap();
                                self.leds.set(value);
                                self.leds.shift_out();

                            },
                            "toggle" => {
                                let value: u64 = value.parse().unwrap();
                                self.leds.toggle(value);
                                self.leds.shift_out();
                            },
                            _ => println!("Unknown action: {}", action),
                        }
                    },
                    "relais" => {
                        match action {
                            "get" => {
                                let value: u64 = value.parse().unwrap();
                                self.relais.get(value);
                                self.relais.shift_out();

                            },
                            "set" => {
                                let value: u64 = value.parse().unwrap();
                                self.relais.set(value);
                                self.relais.shift_out();

                            },
                            "toggle" => {
                                let value: u64 = value.parse().unwrap();
                                self.relais.toggle(value);
                                self.relais.shift_out();
                            },
                            _ => println!("Unknown action: {}", action),
                        }
                    },
                    _ => println!("Unknown command: {}", command),
                }
            },
            _ => println!("Unknown command.")
        }
    }
}
