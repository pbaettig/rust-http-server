use std::net::TcpListener;
use std::io::Read;
use std::process::exit;
use std::str;
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
            let mut buf: [u8; 1024] = [0;1024];
            let n = match stream.read(&mut buf) {
                Ok(n) => n,
                Err(e) => {
                    println!("couldn't read from {}: {}", remote_addr.ip(), e);
                    continue;
                }
            };
            println!("read {} bytes from {}", n, remote_addr.ip());
            println!("{}", str::from_utf8(&buf).unwrap());
        }
        
    }
}