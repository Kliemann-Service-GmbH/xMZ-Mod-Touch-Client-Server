#![feature(slice_patterns)]
use std::os::unix::net::{UnixStream};
use std::io::prelude::*;

use xmz_shift_register::ShiftRegister;

pub fn handle_client(mut stream: UnixStream) {
    let mut relais = ShiftRegister::new_relais();

    println!("Client handled stream: {:?}", stream);
    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();
    let split: Vec<_> = response.split_whitespace().collect();
    match &split[..] {
        [command, action, value] => {
            //Ok(println!("c:{} a:{} v:{}", command.to_string(), action.to_string(), value.to_string()))
            let val: u64 = value.parse().unwrap();
            relais.set(val);
            relais.shift_out();
        },
        _ => println!("Invalid command: {}", response),
    };
}
