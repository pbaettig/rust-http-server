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
        let mut buf = Cursor::new(Vec::<u8>::new());
        let mut h = Headers::new();

        let payload_string = String::from("Hello there!");
        let payload: Vec<u8> = payload_string.bytes().collect();
        let payload_len = payload.len();

        h.add("server".to_string(), Some("rust-tests".to_string()));

        let mut resp = Response {
            version: HttpVersion::HTTP1_1,
            status: Status::Ok,
            headers: h,
            payload,
        };

        let n= resp.write_to(&mut buf).unwrap();
        assert_eq!(resp.headers.content_length().unwrap(), payload_len, "Content-Length is wrong");
       
        let server_header = format!("server: {}\r\n", resp.headers.server().unwrap());
        let content_len_header =  format!("content-length: {}\r\n", resp.headers.content_length().unwrap());


        

        let rs = String::from_utf8(buf.into_inner()).unwrap();
        assert_eq!(rs.len(), n);

        let first_line_end = rs.find('\r').unwrap() + 2;
        assert_eq!(first_line_end, 17);

        let first_line = &rs[0..first_line_end];
        println!("{:?}", first_line);
        assert_eq!(first_line, "HTTP/1.1 200 OK\r\n");

        
        let rest = &rs[first_line_end..];
        println!("{:?}", rest);
        let line_end = rest.find("\r\n").unwrap() + 2;
        let header_1 = &rest[0..line_end];
        println!("1 {:?}", header_1);


        let rest = &rest[line_end..];
        let line_end = rest.find('\r').unwrap()+2;
        let header_2 = &rest[0..line_end];
        println!("2 {:?}", header_2);


        if header_1.starts_with("server") {
            assert_eq!(header_1, server_header);
            assert_eq!(header_2, content_len_header);

        } else if header_2.starts_with("server") {
            assert_eq!(header_1, content_len_header);
            assert_eq!(header_2, server_header);

        } else {
            panic!("no server header")
        }



        let rest = &rest[line_end..];
        let line_end = rest.find('\r').unwrap()+2;
        let new_line = &rest[0..line_end];
        println!("3 {:?}", new_line);
        assert_eq!(new_line, "\r\n");

        let payload = &rest[line_end..];
        println!("4 {:?}", payload);
        assert_eq!(payload_string, payload);

    
    }
}
