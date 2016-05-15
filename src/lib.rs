f#![feature(stmt_expr_attributes)]
#![feature(unix_socket)]
extern crate tempdir;
extern crate unix_socket;
extern crate xmz_shift_register;

#[macro_use]
mod common;
mod server;
mod server_command;
