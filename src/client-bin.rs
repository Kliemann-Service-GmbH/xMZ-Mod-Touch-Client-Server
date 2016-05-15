#![feature(slice_patterns)]
#![feature(stmt_expr_attributes)]
#![feature(unix_socket)]
#[macro_use]
extern crate clap;
extern crate tempdir;
extern crate unix_socket;
extern crate xmz_shift_register;

#[macro_use]
mod common;
mod client;
use client::Client;

fn main() {
    let mut client = Client::new();
}
