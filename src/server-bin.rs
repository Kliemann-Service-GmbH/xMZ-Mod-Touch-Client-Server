#![feature(slice_patterns)]
#![feature(stmt_expr_attributes)]
extern crate nanomsg;
extern crate xmz_shift_register;

mod server;

use server::server_core::{Server};

fn main() {
    let mut server = Server::new();
    server.init();
}
