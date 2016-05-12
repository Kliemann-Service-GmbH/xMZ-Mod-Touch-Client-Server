#![feature(slice_patterns)]
use std::os::unix::net::{UnixStream};
use std::io::prelude::*;


pub fn handle_client(mut stream: UnixStream) {
    println!("Client handled stream: {:?}", stream);
    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();
    let split: Vec<_> = response.split_whitespace().collect();
    match &split[..] {
        [command, action, value] => Ok(println!("c:{} a:{} v:{}", command.to_string(), action.to_string(), value.to_string())),
        _ => Err(format!("Invalid command: {}", response)),
    };
}
