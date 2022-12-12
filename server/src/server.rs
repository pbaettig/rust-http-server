use std::net::{TcpListener, TcpStream};
use std::io::{Read, BufReader, BufRead};
use std::process::exit;
use std::str::{self, FromStr};

use crate::http;

use super::http::Request;
pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self{addr} // instead of Self { addr: addr }, because variable and field names match
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
            println!("new connection from {:?}", remote_addr.ip());
            let req = Self::read_request(stream).unwrap();
            println!("************");
            println!("{}", req.to_string()); 
            println!("************");

            
            

            // let n = match stream.read(&mut buf) {
            //     Ok(n) => n,
            //     Err(e) => {
            //         println!("couldn't read from {}: {}", remote_addr.ip(), e);
            //         continue;
            //     }
            // };


            // println!("read {} bytes from {}", n, remote_addr.ip());
            // let request_str = str::from_utf8(&buf).unwrap();

            // let request = Request::from_str(request_str);
            // println!("{:?}", request);

        }
        
    }

    fn read_request(stream: TcpStream) -> Result<Request, ()> {
        let mut line_buf = String::new();
        let mut reader = BufReader::new(stream);
        

        // read first line (GET /index HTTP/1.1)
        reader.read_line(&mut line_buf).map_err(|_| ())?;
        let (method, uri, version) = http::Request::parse_first_line(line_buf.as_str()).map_err(|_| ())?;
        println!("{:?} {:?} {:?}", method, uri, version);
        

        // Read headers
        let mut headers = http::Headers::new();
        loop {
            line_buf.clear();
            let Ok(_) = reader.read_line(&mut line_buf) else {
                break;
            };

           
            match line_buf.as_str() {
                "\r\n" => {
                    // println!("Empty!");
                    break
                }
                l => { 
                    if let Ok((k,v)) = headers.add_from_line(l) {
                       println!("{}: {:?}", k, v);
                    };
                } ,
            }
        }

        
        // Read rest of the request according to specified Content-Length
        let cl = headers.content_length().unwrap();
        let mut payload = vec![0u8; cl];
       
        if cl > 0 {
            let Ok(n) = reader.read(&mut payload) else {
                println!("Couldn't read payload");
                return Err(());
            };
        }


        Ok(Request {method, uri, headers, version, payload})
    }
}