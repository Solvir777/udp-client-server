mod server;

use std::io::{Read, Write};
use crate::server::Server;
use shared::{Serializable};


fn main() {
    let mut server = Server::new();
    server.run();
}

