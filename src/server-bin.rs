#![feature(slice_patterns)]
#![feature(stmt_expr_attributes)]
#![feature(unix_socket)]
extern crate tempdir;
extern crate unix_socket;
extern crate xmz_shift_register;

#[macro_use]
mod common;
mod server;
use server::Server;

fn main() {
    let mut server = Server::new();
    server.init();
}
