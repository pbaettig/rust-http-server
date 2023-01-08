use std::{collections::{HashMap, hash_map::RandomState}, fmt::Display, error::Error};

#[derive(Debug)]
#[derive(Default)]
pub struct Headers {
    map: HashMap<String, Option<String>, RandomState>
}


#[derive(Debug)]
pub enum HeaderError {
    NotFound,
    NoValue,
    InvalidValue,
}

impl Display for HeaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoValue => write!(f, "header is defined but has no value"),
            Self::NotFound => write!(f, "header is not defined"),
            Self::InvalidValue => write!(f, "header is defined but has an invalid value, e.g. invalid int"),
        }
    }
}

impl Error for HeaderError {}




impl std::string::ToString for Headers {
    fn to_string(&self) -> String {
        let mut s = String::new();
        for (k,v) in &self.map {
            let vs = match v {
                Some(s) => s.to_owned(),
                None => "".to_string(),
            };
            s.push_str(&format!("{}: {}\r\n", k,vs));
        }
        s.push_str("\r\n");
        
        s
    }
}

impl Headers {
    pub fn new() -> Self {
        Headers {map: HashMap::new()}
    }

    pub fn default() -> Self {
        let mut h = Headers {map: HashMap::new()};
        h.add("Server".to_string(), Some("rusty-server".to_string()));
        h.add("Connection".to_string(), Some("close".to_string()));
        
        h
    }

    pub fn content_length(&self) -> Result<usize, HeaderError> {
        match self.get(String::from("content-length")) {
            Err(e) => Err(e),
            Ok(v) =>  v.parse::<usize>().map_err(|_| HeaderError::InvalidValue)
        }

    }

    pub fn server(&self) -> Result<String, HeaderError> {
        self.get(String::from("server"))
    }

    pub fn add(&mut self, k: String, v: Option<String>) ->Option<Option<String>>{
        self.map.insert(k.to_lowercase(), v)
    }

    pub fn get(&self, k: String) -> Result<String, HeaderError> {
        let v = self.map.get(&k.to_ascii_lowercase());
        match v {
            Some(o) => {
                match o {
                    Some(v) => Ok(v.to_owned()),
                    None => Err(HeaderError::NoValue),
                }
            },
            None => Err(HeaderError::NotFound)
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
