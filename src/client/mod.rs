use nanomsg::{Endpoint, Protocol, Socket};
use std::io::Write;

pub struct Client;

impl Client {
    pub fn new() -> Client {
        Client {}
    }

    pub fn try_write(&mut self, msg: &str)  {
        let mut socket = Socket::new(Protocol::Push).unwrap();
        let mut endpoint = socket.connect("ipc:///tmp/pipeline_collector.ipc").unwrap();
        match socket.write_all(msg.as_bytes()) {
            Err(err) => println!("Could not send message: {}", err),
            Ok(_) => {},
        };
        endpoint.shutdown();
    }
}
