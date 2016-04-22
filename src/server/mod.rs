use std::os::unix::net::{UnixStream};
use std::io::prelude::*;


pub fn handle_client(mut stream: UnixStream) {
    println!("Client handled stream: {:?}", stream);
    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();
    println!("{}", response);
}
