use glob::Pattern;
use server::{Handler, Method, Response, Server, Status};

fn main() {
    let mut server = Server::new(String::from("localhost:8080"));
    server.handlers.register(Handler {
        methods: vec![Method::GET, Method::POST],
        pattern: Pattern::new("/ok/*").unwrap(),
        func: |r, w| -> Result<usize, std::io::Error> {
            let mut resp = Response::new(Status::Ok, &format!("hello from {}!", r.uri.path));

            resp.write_to(w)
        },
    });
    server.run()
}
