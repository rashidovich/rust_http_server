use server::Server;
use http::ParseError;

mod server;
mod http;

fn main() {
    let addr = String::from("127.0.0.1:8080");
    let server = Server::new(addr);
    server.run();
}

