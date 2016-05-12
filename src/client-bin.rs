#![feature(unix_socket)]
#[macro_use]
extern crate clap;
extern crate tempdir;

#[macro_use]
mod common;
use clap::{App, Arg, SubCommand};

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
                                .about("LED Kontrolle der 'xMZ-Mod-Touch'")
                                .subcommand(SubCommand::with_name("set")
                                    .about("Schaltet eine LED")
                                    .arg(Arg::with_name("value")
                                        .help("Led Nummer, beginnend mit 1")
                                        .required(true)
                                        .index(1))))
                            .subcommand(SubCommand::with_name("relais")
                                .about("RELAIS Kontrolle der 'xMZ-Mod-Touch'"))
                            .get_matches();

    if let Some(matches) = matches.subcommand_matches("led") {
        if let Some(matches) = matches.subcommand_matches("set") {
            if matches.is_present("value") {
                let val = format!("led set {}", matches.value_of("value").unwrap());
                try_write(stream, &val);
            }
        }
    }
}

fn try_write(mut stream: UnixStream, msg: &str)  {
    match stream.write_all(msg.as_ref()) {
        Err(_) => panic!("Could not send message"),
        Ok(_) => {},
    };
}
