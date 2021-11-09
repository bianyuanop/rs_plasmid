mod lobby;

use crate::lobby::server::Server;

fn main() {
    let server = Server::new(100, "Server1".to_string(), "server test ".to_string(), 1000, 0, "127.0.0.1:9000".to_string());


}