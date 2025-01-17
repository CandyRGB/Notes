mod server;
mod router;
mod handler;

use server::*;

fn main() {
    let server = Server::new("127.0.0.1:3000");
    server.run();
}
