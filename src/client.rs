extern crate dbus;

use dbus::{BusType,Connection,Message};
use dbus::arg::Array;

fn main() {
    let connection = Connection::get_private(BusType::Session).unwrap();
    let message = Message::new_method_call(
        "com.gaswarnanlagen.server",    // destination
        "/com/gaswarnanlagen/server",   // path
        "com.gaswarnanlagen.server",    // interface
        "Foo").unwrap();                // method
    let _replay = connection.send_with_reply_and_block(message, 2000).unwrap();
}
