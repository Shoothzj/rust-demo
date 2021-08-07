extern crate websocket;

use websocket::sync::Server;

fn main() {
    let server = Server::bind("localhost:8080");
}