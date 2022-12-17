
use std::io::Write;

use server::{Method, Server, Request, Response};
use glob::Pattern;
fn main() {
    let mut server = Server::new(String::from("localhost:8080"));

    server.run()
}


