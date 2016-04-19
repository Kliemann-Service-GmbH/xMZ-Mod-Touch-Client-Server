extern crate dbus;


mod server;

use server::Server;

fn main() {
    let server = Server::new();

    server.start();
}
