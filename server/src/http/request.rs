
use std::{collections::{HashMap, hash_map::RandomState}, str::{Bytes, FromStr}, hash::Hash, fmt::{self, format}, net::TcpStream, io::{BufReader, BufRead, Read}};
use std::string::ToString;
use super::Method;
use super::Uri;
use super::Headers;
use super::HttpVersion;




#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub uri: Uri,
    pub headers: Headers,
    pub payload: Vec<u8>,
    pub version: HttpVersion,
    pub stream: TcpStream,
}

#[derive(Debug)]
pub enum RequestParseError  {
    CantReadRequest,
    RequestMalformed,
    UrlMalformed,
    VersionMalformed,
    UnknownMethod,
    UnsupportedVersion,
}


impl Request {
    pub fn new(s: TcpStream) -> Result<Self, ()> {
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

        Ok(Request { method, uri, headers, payload, version, stream: reader.into_inner() })
    }

    pub fn parse_first_line(l: &str) -> Result<(Method, Uri, HttpVersion), RequestParseError> {
        let mut tokens = l.split_ascii_whitespace();
        let method = match tokens.next() {
            Some(m) => Method::from_str(m).map_err(|_| RequestParseError::UnknownMethod)?,
            _ => return Err(RequestParseError::RequestMalformed),
        };
    
        // let Some(path) = req_tokens.next() else {
        //     return Err(RequestParseError::Malformed)
        // };
        
        let uri = match tokens.next() {
            Some(p) =>  Uri::from_str(p).map_err(|_| RequestParseError::UrlMalformed)?,
            None => return Err(RequestParseError::RequestMalformed),
        };
    
        let version = match tokens.next() {
            Some(p) =>  HttpVersion::from_str(p).map_err(|_| RequestParseError::VersionMalformed)?,
            None => return Err(RequestParseError::RequestMalformed),
        };

        Ok((method, uri, version))
    }

    // pub fn parse_header_line(l: &str) -> Result<(String, Option<String>), ()> {
    //     //"User-Agent: curl/7.81.0\r\n"
    //     let Some((k,v)) = l.split_once(':').or(Some((l, ""))) else {
    //         return Err(())
    //     };
    //     let v = match v {
    //         "" => None,
    //         s => Some(s.trim().to_string()),
    //     };
        
    //     Ok((k.trim().to_string(), v))
    // }
}

impl std::string::ToString for Request {
    fn to_string(&self) -> String {
        let mut s = String::from(
            format!("{} {} {}\n", self.method.to_string(), self.uri.to_string(), self.version.to_string())
        );
        s.push_str(&self.headers.to_string());
       
        if let Ok(p) = String::from_utf8(self.payload.clone()) {
            s.push_str(&p)
        }
        
        s
    }
}
