
use server::Server;
fn main() {
    let server = Server::new(String::from("localhost:8080"));

    server.run()
}


