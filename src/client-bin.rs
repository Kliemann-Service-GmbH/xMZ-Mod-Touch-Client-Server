#![feature(slice_patterns)]
#![feature(stmt_expr_attributes)]
#![feature(unix_socket)]
#[macro_use]
extern crate clap;
extern crate nanomsg;
extern crate tempdir;
extern crate unix_socket;
extern crate xmz_shift_register;
use client::Client;
use clap::{App, Arg, SubCommand};

#[macro_use]
mod common;
mod client;

fn main() {
    let mut client = Client::new();

    let matches = App::new("xMZ-Mod-Touch Client")
                            .author("Stefan Müller <s.mueller@it.kls-glt.de>")
                            .version(crate_version!())
                            .about("Kontrolliert alle Serverfunktionen der 'xMZ-Mod-Touch' Hardware")
                            .subcommand(SubCommand::with_name("led")
                                .about("LED Kontrolle der 'xMZ-Mod-Touch'")
                                .subcommand(SubCommand::with_name("get")
                                    .about("True oder False je nach Zustand der LED")
                                    .arg(Arg::with_name("value")
                                        .help("Led Nummer, beginnend mit 1")
                                        .required(true)
                                        .index(1)))
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
        if let Some(matches) = matches.subcommand_matches("get") {
            if matches.is_present("value") {
                let val = format!("led get {}", matches.value_of("value").unwrap());
                client.try_write(&val);
            }
        }
        if let Some(matches) = matches.subcommand_matches("set") {
            if matches.is_present("value") {
                let val = format!("led set {}", matches.value_of("value").unwrap());
                client.try_write(&val);
            }
        }
    }
}
