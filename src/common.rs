/// Common vars for client and server
pub static SOCKET_PATH: &'static str = "xmz-socket";


macro_rules! or_panic {
    ($e:expr) => {
        match $e {
            Ok(e) => e,
            Err(e) => panic!("{}", e),
        }
    }
}
