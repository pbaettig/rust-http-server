use std::{io::Write, net::TcpStream};

use super::{Headers, HttpVersion};
#[derive(Clone, Copy)]
pub enum Status {
    Ok = 200,
    Created = 201,
    Accepted = 202,
    NonAuthoritativeInformation = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,

    NotFound = 404,

    InternalServerError = 500,
    HttpVersionNotSupported = 505,
}

impl std::string::ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Self::Ok => format!("{} OK", *self as u16).to_string(),
            Self::Created => format!("{} Created", *self as u16).to_string(),
            Self::Accepted => format!("{} Accepted", *self as u16).to_string(),
            Self::NonAuthoritativeInformation => {
                format!("{} Non-Authoritative Information", *self as u16).to_string()
            }
            Self::NoContent => format!("{} No Content", *self as u16).to_string(),
            Self::ResetContent => format!("{} Reset Content", *self as u16).to_string(),
            Self::PartialContent => format!("{} Partial Content", *self as u16).to_string(),
            Self::NotFound => format!("{} Not Found", *self as u16).to_string(),
            Self::InternalServerError => {
                format!("{} Internal Server Error", *self as u16).to_string()
            }
            Self::HttpVersionNotSupported => {
                format!("{} HTTP Version Not Supported", *self as u16).to_string()
            }
        }
    }
}

pub struct Response {
    version: HttpVersion,
    pub status: Status,
    pub headers: Headers,
    pub payload: Vec<u8>,
}

impl Response {
    pub fn new(status: Status, msg: &str) -> Self {
        Self {
            version: HttpVersion::HTTP1_1,
            status: status,
            headers: Headers::default(),
            payload: msg.bytes().collect(),
        }
    }
}

impl Response {
    pub fn write_to(&self, mut s: TcpStream) -> Result<usize, std::io::Error> {
        s.write(self.to_string().as_bytes())
    }
}

impl std::string::ToString for Response {
    fn to_string(&self) -> String {
        let mut s = String::from(format!(
            "{} {}\r\n",
            self.version.to_string(),
            self.status.to_string()
        ));
        s.push_str(&self.headers.to_string());

        if let Ok(p) = String::from_utf8(self.payload.clone()) {
            s.push_str(&p)
        }

        s
    }
}
