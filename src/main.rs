use http::Request;
use http::HttpMethod;
use server::Server;

mod http;
mod server;

fn main() {
    let addr = String::from("127.0.0.1:8080");
    let server = Server::new(addr);
    server.run();
}

