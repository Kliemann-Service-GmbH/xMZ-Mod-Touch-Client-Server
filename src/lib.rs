#![feature(stmt_expr_attributes)]
#![feature(unix_socket)]
extern crate tempdir;
extern crate sysfs_gpio;
extern crate unix_socket;

#[macro_use]
mod common;
mod shift_register;
mod server;

use common::SOCKET_PATH;
use shift_register::ShiftRegister;
use std::fs;
use std::os::unix::net::{UnixListener, UnixStream};
use std::thread;
use std::io::prelude::*;
use std::path::Path;
use tempdir::TempDir;
