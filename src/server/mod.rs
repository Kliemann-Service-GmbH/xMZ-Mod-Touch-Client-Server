use std::os::unix::net::{UnixListener, UnixStream};
use common::SOCKET_PATH;
use shift_register::ShiftRegister;
use std::fs;
use std::thread;
use std::io::prelude::*;
use std::path::Path;
use tempdir::TempDir;


pub fn handle_client(mut stream: UnixStream) {
    println!("Client handled stream: {:?}", stream);
    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();
    println!("{}", response);
}
