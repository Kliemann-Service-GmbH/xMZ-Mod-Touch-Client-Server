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
                            .subcommand(SubCommand::with_name("leds")
                                .about("LED Kontrolle der 'xMZ-Mod-Touch'")
                                .subcommand(SubCommand::with_name("get")
                                    .about("liefert `true` oder `false` je nach Zustand der LED")
                                    .arg(Arg::with_name("value")
                                        .help("Led Nummer, beginnend mit 1")
                                        .required(true)
                                        .index(1)))
                                .subcommand(SubCommand::with_name("set")
                                    .about("schaltet eine LED ein")
                                    .arg(Arg::with_name("value")
                                        .help("Led Nummer, beginnend mit 1")
                                        .required(true)
                                        .index(1)))
                                .subcommand(SubCommand::with_name("toggle")
                                    .about("wechselt den Zustand einer LED")
                                    .arg(Arg::with_name("value")
                                        .help("LED Nummer, beginnend mit 1")
                                        .required(true)
                                        .index(1)))
                                )
                            .subcommand(SubCommand::with_name("relais")
                                .about("RELAIS Kontrolle der 'xMZ-Mod-Touch'")
                                .subcommand(SubCommand::with_name("get")
                                    .about("liefert `true` oder `false` je nach Zustand der RELAIS")
                                    .arg(Arg::with_name("value")
                                        .help("Led Nummer, beginnend mit 1")
                                        .required(true)
                                        .index(1)))
                                .subcommand(SubCommand::with_name("set")
                                    .about("schaltet eine RELAIS ein")
                                    .arg(Arg::with_name("value")
                                        .help("Led Nummer, beginnend mit 1")
                                        .required(true)
                                        .index(1)))
                                .subcommand(SubCommand::with_name("toggle")
                                    .about("wechselt den Zustand eines RELAIS")
                                    .arg(Arg::with_name("value")
                                        .help("RELAIS Nummer, beginnend mit 1")
                                        .required(true)
                                        .index(1)))

                            )
                            .get_matches();

    if let Some(matches) = matches.subcommand_matches("leds") {
        if let Some(matches) = matches.subcommand_matches("get") {
            if matches.is_present("value") {
                let val = format!("leds get {}", matches.value_of("value").unwrap());
                client.try_write(&val);
            }
        }
        if let Some(matches) = matches.subcommand_matches("set") {
            if matches.is_present("value") {
                let val = format!("leds set {}", matches.value_of("value").unwrap());
                client.try_write(&val);
            }
        }
        if let Some(matches) = matches.subcommand_matches("toggle") {
            if matches.is_present("value") {
                let val = format!("leds toggle {}", matches.value_of("value").unwrap());
                client.try_write(&val);
            }
        }
    }

    if let Some(matches) = matches.subcommand_matches("relais") {
        if let Some(matches) = matches.subcommand_matches("get") {
            if matches.is_present("value") {
                let val = format!("relais get {}", matches.value_of("value").unwrap());
                client.try_write(&val);
            }
        }
        if let Some(matches) = matches.subcommand_matches("set") {
            if matches.is_present("value") {
                let val = format!("relais set {}", matches.value_of("value").unwrap());
                client.try_write(&val);
            }
        }
        if let Some(matches) = matches.subcommand_matches("toggle") {
            if matches.is_present("value") {
                let val = format!("relais toggle {}", matches.value_of("value").unwrap());
                client.try_write(&val);
            }
        }
    }



}
