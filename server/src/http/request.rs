
use std::{collections::{HashMap, hash_map::RandomState}, str::{Bytes, FromStr}, hash::Hash, fmt::{self, format}};
use std::string::ToString;
use super::method::Method;


// TODO: Add support for multiple values with the same name
type QueryParams = HashMap<String, Option<String>, RandomState>;

#[derive(Default)]
#[derive(Debug)]
pub struct Uri {
    path: String,
    params: QueryParams,
    raw: String,
}

impl std::str::FromStr for Uri {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (path, query) = match s.split_once('?') {
            Some(p) => p,
            None => return Err(()),
        };

        let mut params: QueryParams = HashMap::new();
        for p in query.split('&') {
            let Some((k,v)) = p.split_once('=').or(Some((p, ""))) else {
                continue;
            };

            params.insert(k.to_string(), match v {
                "" => None,
                s => Some(s.to_string()),
            });
        }

        Ok(Uri { path: path.to_string(), params, raw: s.to_string() })
    }

}

impl std::string::ToString for Uri {
    fn to_string(&self) -> String {
        self.raw.to_string()
    }
}

// impl fmt::Display for Uri {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//       write!(f, "{}", self.raw)
//     }
// }

#[derive(Debug)]
#[derive(Default)]
pub struct Headers {
    map: HashMap<String, Option<String>, RandomState>
}

// impl fmt::Display for Headers {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         for (k,v) in self.map {
//             write!(f, "{}: {:?}", k, v)?
//         }
        
//         Ok(())
//     }
// }
impl std::string::ToString for Headers {
    fn to_string(&self) -> String {
        let mut s = String::new();
        for (k,v) in &self.map {
            s.push_str(&format!("{}: {:?}\n", k, v));
        }
        s.push_str("\n");
        s
    }
}

impl Headers {
    pub fn new() -> Self {
        Headers {map: HashMap::new()}
    }

    pub fn content_length(&self) -> Result<usize, ()> {
        let v = match self.get(String::from("content-length")) {
            None => 0,
            Some(v) => {
                
                let Some(vn) = v else {
                    // Content-Length header exists but has no value
                    return Err(());
                };


               vn.parse::<usize>().map_err(|_| ())?

            }
        };

        return Ok(v)
    }

    pub fn add(&mut self, k: String, v: Option<String>) ->Option<Option<String>>{
        self.map.insert(k.to_lowercase(), v)
    }

    pub fn get(&self, k: String) -> Option<Option<String>> {
        let v = self.map.get(&k.to_ascii_lowercase());
        match v {
            Some(o) => Some(o.to_owned()),
            None => None
        }
    }

    pub fn add_from_line(&mut self, l: &str) -> Result<(String, Option<String>),()> {
        //"User-Agent: curl/7.81.0\r\n"
        let Some((k,v)) = l.split_once(':').or(Some((l, ""))) else {
            return Err(())
        };

        let v = match v {
            "" => None,
            s => Some(s.trim().to_string()),
        };

        self.add(k.to_string(), v.clone());

        Ok((k.to_string(), v))
    } 
}

#[derive(Default)]
#[derive(Debug)]
pub enum HttpVersion {
    HTTP1_0,
    #[default]
    HTTP1_1,
}


impl std::string::ToString for HttpVersion {
    fn to_string(&self) -> String {
        match self {
            HttpVersion::HTTP1_0 => String::from("HTTP/1.0"),
            HttpVersion::HTTP1_1 => String::from("HTTP/1.1"),
        }
    }
}

impl std::str::FromStr for HttpVersion {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HTTP/1.1" => Ok(Self::HTTP1_1),
            "HTTP/1.0" => Ok(Self::HTTP1_0),
            _ => Err(())
        }
        
    }
}




#[derive(Default)]
#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub uri: Uri,
    pub headers: Headers,
    pub payload: Vec<u8>,
    pub version: HttpVersion,
}

#[derive(Debug)]
pub enum RequestParseError  {
    CantReadRequest,
    Malformed,
    UnknownMethod,
    UnsupportedVersion,
}


impl Request {
    pub fn parse_first_line(l: &str) -> Result<(Method, Uri, HttpVersion), RequestParseError> {
        let mut tokens = l.split_ascii_whitespace();
        let method = match tokens.next() {
            Some(m) => Method::from_str(m).map_err(|_| RequestParseError::UnknownMethod)?,
            _ => return Err(RequestParseError::Malformed),
        };
    
        // let Some(path) = req_tokens.next() else {
        //     return Err(RequestParseError::Malformed)
        // };
        
        let uri = match tokens.next() {
            Some(p) =>  Uri::from_str(p).map_err(|_| RequestParseError::Malformed)?,
            None => return Err(RequestParseError::Malformed),
        };
    
        let version = match tokens.next() {
            Some(p) =>  HttpVersion::from_str(p).map_err(|_| RequestParseError::Malformed)?,
            None => return Err(RequestParseError::Malformed),
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

// GET / HTTP/1.1
// Host: localhost:8888
// User-Agent: curl/7.81.0
// Accept: */*

// impl std::str::FromStr for Request {
//     type Err = RequestParseError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let mut lines = s.lines();
//         let Some(req) = lines.next() else {
//            return Err(RequestParseError::CantReadRequest)
//         };

//         let mut req_tokens = req.split_ascii_whitespace();
//         let method = match req_tokens.next() {
//             Some(m) => Method::from_str(m).map_err(|_| RequestParseError::UnknownMethod)?,
//             _ => return Err(RequestParseError::Malformed),
//         };

//         // let Some(path) = req_tokens.next() else {
//         //     return Err(RequestParseError::Malformed)
//         // };
        
//         let uri = match req_tokens.next() {
//             Some(p) =>  Uri::from_str(p).map_err(|_| RequestParseError::Malformed)?,
//             None => return Err(RequestParseError::Malformed),
//         };


//         let version = match req_tokens.next() {
//             Some(v) => HttpVersion::from_str(v).map_err(|_| RequestParseError::UnsupportedVersion)?,
//             _ => { return Err(RequestParseError::Malformed) }
//         };


//         Ok(Self { method, uri, version})



    

//     }
// }

