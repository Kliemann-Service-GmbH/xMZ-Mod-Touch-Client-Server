#![feature(unix_socket)]
extern crate tempdir;

#[macro_use]
mod common;

use std::path::Path;
use std::os::unix::net::UnixStream;
use std::io::prelude::*;


fn main() {

    let path = Path::new("/tmp");
    let socket_path = path.join(common::SOCKET_PATH);

    let mut stream = or_panic!(UnixStream::connect(&socket_path));
    stream.write_all(b"zzeroo systems, sa hallo").unwrap();
}
