use crate::http::{Handlers, HttpVersion, Request, Response, Status};
use std::net::{TcpListener, TcpStream};
use std::process::exit;
use std::time::Duration;

pub struct Server {
    addr: String,
    pub handlers: Handlers<TcpStream>,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr,
            handlers: Handlers::new(),
        }
    }

    fn dispatch_request(&self, r: Request, w: TcpStream) -> bool {
        let (found, handler) = self.handlers.get(&r.method, &r.uri.path);
        handler.handle(r, w).unwrap();

        return found;
    }

    pub fn run(self) {
        println!("binding to {}", self.addr);
        let listener = match TcpListener::bind(&self.addr) {
            Ok(l) => l,
            Err(error) => {
                println!("couldn't bind to {}: {}", self.addr, error.to_string());
                exit(1);
            }
        };
        println!("running on {}", self.addr);
        loop {
            let (stream, remote_addr) = match listener.accept() {
                Ok((stream, addr)) => (stream, addr),
                Err(e) => {
                    println!("failed to accept connection: {}", e.to_string());
                    continue;
                }
            };
            stream.set_write_timeout(Some(Duration::new(5, 0))).unwrap();
            stream.set_read_timeout(Some(Duration::new(5, 0))).unwrap();

            println!("new connection from {:?}", remote_addr.ip());
            let Ok(req) = Request::new(&stream) else {
                println!("failed to parse request");
                continue;
            };
            if req.version != HttpVersion::HTTP1_1 {
                let mut resp = Response::new(
                    Status::HttpVersionNotSupported,
                    "HTTP version is not supported",
                );
                resp.write_to(stream).unwrap();
                continue;
            }

            println!("************");
            println!("{}", req.to_string());
            println!("************");

            if !self.dispatch_request(req, stream) {
                println!("Couldn´t dispatch request");
            };
        }
    }

    // fn read_request(stream: &TcpStream) -> Result<Request, ()> {
    //     let mut line_buf = String::new();
    //     let mut reader = BufReader::new(stream);

    //     // read first line (GET /index HTTP/1.1)
    //     reader.read_line(&mut line_buf).map_err(|_| ())?;
    //     let (method, uri, version) = http::Request::parse_first_line(line_buf.as_str()).map_err(|_| ())?;
    //     println!("{:?} {:?} {:?}", method, uri, version);

    //     // Read headers
    //     let mut headers = http::Headers::new();
    //     loop {
    //         line_buf.clear();
    //         let Ok(_) = reader.read_line(&mut line_buf) else {
    //             break;
    //         };

    //         match line_buf.as_str() {
    //             "\r\n" => {
    //                 // println!("Empty!");
    //                 break
    //             }
    //             l => {
    //                 if let Ok((k,v)) = headers.add_from_line(l) {
    //                    println!("{}: {:?}", k, v);
    //                 };
    //             } ,
    //         }
    //     }

    //     // Read rest of the request according to specified Content-Length
    //     let cl = headers.content_length().unwrap();
    //     let mut payload = vec![0u8; cl];

    //     if cl > 0 {
    //         let Ok(n) = reader.read(&mut payload) else {
    //             println!("Couldn't read payload");
    //             return Err(());
    //         };
    //     }

    //     Ok(Request {method, uri, headers, version, payload})
    // }
}
