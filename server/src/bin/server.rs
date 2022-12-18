

use server::{Method, Server, Request, Response,Status};
use glob::Pattern;

fn main() {
    let mut server = Server::new(String::from("localhost:8080"));
    server.handlers.register(
            Method::GET,
            Pattern::new("/ok*").unwrap(), 
            |r: Request| {
                let resp = Response::new(Status::Ok, "OK!");
                resp.write_to(r.stream).unwrap();
            }
    );

    server.handlers.register(
        Method::GET,
        Pattern::new("/nok*").unwrap(), 
        |r: Request| {
            let resp = Response::new(Status::InternalServerError, "Not OK");
            resp.write_to(r.stream).unwrap();
            
        }
    );

    server.handlers.register(
        Method::POST,
        Pattern::new("/form*").unwrap(), 
        |r: Request| {
            let resp = Response::new(Status::Ok, "got it");
            println!("{}", String::from_utf8(r.payload).unwrap());
            resp.write_to(r.stream).unwrap();
            
        }
    );

    server.run()
}


