use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, BufReader, BufRead, Write};
use std::process::exit;
use std::str::{self, FromStr};
use std::time::Duration;
use glob::Pattern;

use crate::{http, Method};

use super::http::Request;
use super::http::Response;

type HandlerFn = fn(Request);


pub struct Handlers {
    handlers: Vec<(Method, Pattern, HandlerFn)>
}

impl Handlers {
    pub fn new() -> Self {
        Handlers { handlers: vec![(http::Method::GET, Pattern::new("/ok*").unwrap(), |mut r: Request| {
            println!("here I am!!");
            let mut resp = Response::ok(r.version).to_string();
            resp.push_str("\r\nHello there!");
            
            r.stream.write(resp.as_bytes());
        } )] 
        }
    }

    pub fn get(&self, m: &Method, uri: &str) -> Result<(usize, HandlerFn), ()> {
        for (i, (rm, rp, hf)) in self.handlers.iter().enumerate() {
            if *rm == *m && rp.matches(uri) {
                return Ok((i, *hf))
            }
        }

        Err(())
    }

    // pub fn register(&mut self, m: Method, p: Pattern, h: HandlerFn, ) {
    //     if let Some((idx, _)) = self.get(&m, &p) {
    //         self.handlers[idx] = (m, p, h);
    //         return
    //     }

    //     self.handlers.push((m, p, h));

    // }
}

pub struct Server {
    addr: String,
    pub handlers: Handlers,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self{addr, handlers: Handlers::new()}
    }

    fn dispatch_request(&self, mut r: Request) -> Result<(), ()> {
        match self.handlers.get(&r.method, &r.uri.path) {
            Ok((_, h)) => { h(r); return Ok(()) },
            _ => {
                println!("uh oh...");
                r.stream.write(Response::internal_server_error(r.version).to_string().as_bytes());
                return Err(())
            }
        }
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
            let (mut stream, remote_addr) = match listener.accept() {
                Ok((stream, addr)) => (stream, addr),
                Err(e) => {
                    println!("failed to accept connection: {}", e.to_string());
                    continue;
                }
            };
            stream.set_write_timeout(Some(Duration::new(5,0))).unwrap();
            stream.set_read_timeout(Some(Duration::new(5,0))).unwrap();


            println!("new connection from {:?}", remote_addr.ip());
            let Ok(mut req) = Request::new(stream) else {
                println!("failed to parse request");
                continue;
            };


            println!("************");
            println!("{}", req.to_string()); 
            println!("************");

            
            self.dispatch_request(req);
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