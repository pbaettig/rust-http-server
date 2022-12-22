use std::io::Write;

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

    pub fn set_content_length(&mut self) {
        self.headers.add(
            "content-length".to_string(),
            Some(self.payload.len().to_string()),
        );
    }

    pub fn write_to<T: Write>(&mut self, mut s: T) -> Result<usize, std::io::Error> {
        self.set_content_length();

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

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::{Headers, HttpVersion, Response, Status};

    #[test]
    fn response_write_to() {
        let buf = Cursor::new(Vec::<u8>::new());
        let mut h = Headers::new();

        h.add("server".to_string(), Some("rust-tests".to_string()));

        let _resp = Response {
            version: HttpVersion::HTTP1_1,
            status: Status::Ok,
            headers: h,
            payload: String::from("Hello there!").bytes().collect(),
        };

        //assert_eq!(resp.headers.content_length().unwrap(), 12, "Content-Length is wrong");

        let rs = String::from_utf8(buf.into_inner()).unwrap();
        println!("{rs}");
        // TODO: more tests on response format
    }
}
