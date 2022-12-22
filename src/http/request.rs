use std::{
    io::{BufRead, BufReader, Read},
    net::TcpStream,
    str::FromStr,
};

use super::Headers;
use super::HttpVersion;
use super::Method;
use super::Uri;

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub uri: Uri,
    pub headers: Headers,
    pub payload: Vec<u8>,
    pub version: HttpVersion,
    // pub stream: TcpStream,
}

#[derive(Debug)]
pub enum RequestParseError {
    CantReadRequest,
    RequestMalformed,
    UrlMalformed,
    VersionMalformed,
    UnknownMethod,
    UnsupportedVersion,
}

impl Request {
    pub fn new(s: &TcpStream) -> Result<Self, ()> {
        let mut line_buf = String::new();
        let mut reader = BufReader::new(s);

        // read first line (GET /index HTTP/1.1)
        reader.read_line(&mut line_buf).map_err(|_| ())?;
        let (method, uri, version) = Self::parse_first_line(line_buf.as_str()).unwrap();
        println!("{:?} {:?} {:?}", method, uri, version);

        // Read headers
        let mut headers = Headers::new();
        loop {
            line_buf.clear();
            let Ok(_) = reader.read_line(&mut line_buf) else {
                break;
            };

            match line_buf.as_str() {
                "\r\n" => {
                    // println!("Empty!");
                    break;
                }
                l => {
                    if let Ok((k, v)) = headers.add_from_line(l) {
                        println!("{}: {:?}", k, v);
                    };
                }
            }
        }

        // Read rest of the request according to specified Content-Length
        let cl = headers.content_length().unwrap();
        let mut payload = vec![0u8; cl];

        if cl > 0 {
            let Ok(_) = reader.read(&mut payload) else {
                println!("Couldn't read payload");
                return Err(());
            };
        }

        Ok(Request {
            method,
            uri,
            headers,
            payload,
            version,
            // stream: reader.into_inner(),
        })
    }

    pub fn parse_first_line(l: &str) -> Result<(Method, Uri, HttpVersion), RequestParseError> {
        let mut tokens = l.split_ascii_whitespace();
        let method = match tokens.next() {
            Some(m) => Method::from_str(m).map_err(|_| RequestParseError::UnknownMethod)?,
            _ => return Err(RequestParseError::RequestMalformed),
        };

        let uri = match tokens.next() {
            Some(p) => Uri::from_str(p).map_err(|_| RequestParseError::UrlMalformed)?,
            None => return Err(RequestParseError::RequestMalformed),
        };

        let version = match tokens.next() {
            Some(p) => HttpVersion::from_str(p).map_err(|_| RequestParseError::VersionMalformed)?,
            None => return Err(RequestParseError::RequestMalformed),
        };

        Ok((method, uri, version))
    }
}

impl std::string::ToString for Request {
    fn to_string(&self) -> String {
        let mut s = String::from(format!(
            "{} {} {}\n",
            self.method.to_string(),
            self.uri.to_string(),
            self.version.to_string()
        ));
        s.push_str(&self.headers.to_string());

        if let Ok(p) = String::from_utf8(self.payload.clone()) {
            s.push_str(&p)
        }

        s
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    #[test]
    pub fn test_request() {
        // GET / HTTP/1.1
        // Host: localhost:8088
        // User-Agent: curl/7.81.0
        // Accept: */*

        let mut rs = String::new();
        rs.push_str("GET /index.php?param1=2&p2=2 HTTP/1.1\r\n");
        rs.push_str("Host: testy.mctest.local:8088");
        rs.push_str("User-Agent: unittest/1.2.3");
        rs.push_str("X-Test: 1234abc");
        rs.push_str("Content-Length: 12");
        rs.push_str("\r\n");
        rs.push_str("abcdef123456");

        let _c = Cursor::new(rs.as_bytes());
    }
}
