#![feature(unix_socket)]
#[macro_use]
extern crate clap;
extern crate tempdir;

#[macro_use]
mod common;
use clap::{App, SubCommand};

use std::path::Path;
use std::os::unix::net::UnixStream;
use std::io::prelude::*;


fn main() {
    // Setup socket to server
    let path = Path::new("/tmp");
    let socket_path = path.join(common::SOCKET_PATH);
    let mut stream = or_panic!(UnixStream::connect(&socket_path));

    let matches = App::new("xMZ-Mod-Touch Client")
                            .author("Stefan Müller <s.mueller@it.kls-glt.de>")
                            .version(crate_version!())
                            .about("Kontrolliert alle Serverfunktionen der 'xMZ-Mod-Touch' Hardware")
                            .subcommand(SubCommand::with_name("led")
                                .about("LED Kontrolle der 'xMZ-Mod-Touch'"))
                            .subcommand(SubCommand::with_name("relais")
                                .about("RELAIS Kontrolle der 'xMZ-Mod-Touch'"))
                            .get_matches();

    match stream.write_all(b"HELO") {
        Err(_) => panic!("Could not send message"),
        Ok(_) => {}
    }


}
