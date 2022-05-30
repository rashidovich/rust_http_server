use crate::http::{HttpMethod, Request};
use std::convert::{TryFrom, TryInto};
use std::net::TcpListener;
use std::io::Read;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    pub fn run(self) {
        println!("Listening on: {}", self.addr);
        let tcp_listener = TcpListener::bind(&self.addr).unwrap();

        'outer: loop {
            match tcp_listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {

                                },
                                Err(e) => println!("Failed to parse a request {}", e)
                            };
                        },
                        Err(e) => println!("Failed to read from connection {}", e)
                    };
                },
                Err(e) => println!("Failed to establish a connection: {}", e)
            };
        }
    }
}