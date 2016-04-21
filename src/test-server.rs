extern crate dbus;

use dbus::{tree, Connection, BusType, NameFlag};
use std::cell::Cell;

/// Server struct, controlls the server behavior
///
/// stopp: Cell here because we can mutate on member level. Server instance can imutable
/// https://play.rust-lang.org/?gist=dc81a5cc2e6a041a4ef6cfb43845dae5&version=stable&backtrace=0
///
/// # Examples
///
/// ```
/// let server = Server::new();
/// server.stopp.set(true);
/// assert!(!server.sopp.get());
/// ```
struct Server {
    stopp: Cell<bool>,
}

impl Server {
    fn new() -> Server {
        Server { stopp: Cell::new(false) }
    }

    fn start(&self) {
        let connection = Connection::get_private(BusType::Session).unwrap();
        connection.register_name("com.gaswarnanlagen.test_server.rs", NameFlag::ReplaceExisting as u32).unwrap();
        let factory = tree::Factory::new_sync();
        let tree = factory.tree()
            .add(factory.object_path("/led").introspectable()
                .add(factory.interface("com.gaswarnanlagen.test_server.rs")
                .add_m(factory.method("set", |_, _, _| unimplemented!()).in_arg(("num", "u")))
                .add_m(factory.method("get", |_, _, _| unimplemented!()).in_arg(("num", "u")).out_arg(("leds", "au")))
                )
            )
            .add(factory.object_path("/info").introspectable()
            .add(factory.interface("com.gaswarnanlagen.test_server.rs")
                .add_m(factory.method("Info", |m, _, _| {
                    Ok(vec![m.method_return().append("xMZ-Mod-Touch Test Server")])
                }).out_arg("s"))
                .add_m(factory.method("Usage", |m, _, _| {
                    Ok(vec![m.method_return().append("controll via dbus")])
                }).out_arg("s"))
            ));

        tree.set_registered(&connection, true).unwrap();

        // run forever
        for _ in tree.run(&connection, connection.iter(1000)) {
            if self.stopp.get() {
                break;
            }
        }
    }
}


fn main() {
    let server = Server::new();

    server.start();
}
