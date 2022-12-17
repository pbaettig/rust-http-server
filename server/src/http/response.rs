
use std::fmt::format;

use super::{Headers, HttpVersion};
#[derive(Clone, Copy)]
enum Status {
    Ok = 200,
    Created = 201,
    Accepted = 202,
    NonAuthoritativeInformation = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,

    InternalServerError = 500,
    HttpVersionNotSupported = 505,

}

impl std::string::ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Self::Ok => format!("{} OK", *self as u16).to_string(),
            Self::Created => format!("{} Created", *self as u16).to_string(),
            Self::Accepted => format!("{} Accepted", *self as u16).to_string(),
            Self::NonAuthoritativeInformation => format!("{} Non-Authoritative Information", *self as u16).to_string(),
            Self::NoContent => format!("{} No Content", *self as u16).to_string(),
            Self::ResetContent => format!("{} Reset Content", *self as u16).to_string(),
            Self::PartialContent => format!("{} Partial Content", *self as u16).to_string(),
            Self::InternalServerError => format!("{} Internal Server Error", *self as u16).to_string(),
            Self::HttpVersionNotSupported => format!("{} HTTP Version Not Supported", *self as u16).to_string(),
        }
    }
}

pub struct Response {
    version: HttpVersion,
    status: Status,
    headers: Headers,
    payload: Vec<u8>,
}

impl Response {
    pub fn ok(v: HttpVersion) -> Self {
        return Self {
            version: v,
            status: Status::Ok,
            headers: Headers::default(),
            payload: vec![],
        }
    }
    pub fn internal_server_error(v: HttpVersion) -> Self {
        return Self {
            version: v,
            status: Status::InternalServerError,
            headers: Headers::default(),
            payload: vec![],
        }
    }
    
    pub fn ok_1() -> Self {
        return Self {
            version: HttpVersion::HTTP1_0,
            status: Status::Ok,
            headers: Headers::default(),
            payload: vec![],
        }
    } 
    pub fn ok_1_1() -> Self {
        return Self {
            version: HttpVersion::HTTP1_1,
            status: Status::Ok,
            headers: Headers::default(),
            payload: vec![],
        }
    }
}
impl std::string::ToString for Response {
    fn to_string(&self) -> String {
        let mut s = String::from(
            format!("{} {}\r\n", self.version.to_string(), self.status.to_string())
        );
        s.push_str(&self.headers.to_string());
       
        if let Ok(p) = String::from_utf8(self.payload.clone()) {
            s.push_str(&p)
        }
        
        s
    }
}