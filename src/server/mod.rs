use dbus::{BusType,Connection,ConnectionItem,Message,NameFlag};
use dbus::obj::{Interface,Method,ObjectPath};
use std::cell::Cell;


static DBUS_ERROR_FAILED: &'static str = "org.freedesktop.DBus.Error.Failed";
// Version of the crate equals the server version
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");


pub struct Server {
    /// a flag that stops the server
    pub stop: Cell<bool>,
}

impl Server {
    pub fn new() -> Server {
        Server { stop:Cell::new(false)}
    }

    /// Start in listening state and wait for incoming requests
    pub fn start(&self) {
        let connection = Connection::get_private(BusType::Session).unwrap();
        connection.register_name("com.gaswarnanlagen.server", NameFlag::ReplaceExisting as u32).expect("Was not able to register name.");
        let mut object_path = ObjectPath::new(&connection, "/com/gaswarnanlagen/server", false);
        connection.register_object_path( "/com/gaswarnanlagen/server").expect("could not register object path");

        let server_interface = Interface::new(
            vec![
                Method::new("Stop",
                            vec![],
                            vec![],
                            Box::new(|_msg| {
                                self.stop.set(true);
                                Ok( vec![])
                            })
                ),

                Method::new("Foo",
                            vec![], // input args
                            vec![], // output args
                            Box::new(|msg| {    // Callback
                                println!("Foo {:?}", msg);
                                Ok( vec![])
                            })
                ),
            ],

            vec![],     // no properties
            vec![]  // no signals
        );

        object_path.insert_interface("com.gaswarnanlagen.server", server_interface);

        for n in connection.iter(10) {
            match n {
                ConnectionItem::MethodCall(mut m) =>
                if object_path.handle_message(&mut m).is_none() {
                    connection.send(Message::new_error(&m, DBUS_ERROR_FAILED, "Object path not found").unwrap()).unwrap();
                },
                ConnectionItem::Signal(_m) => { },
                _ => (),
            }
            if self.stop.get() {
                println!("stopping server");
                break;
            }
        }
    }
}
