#[derive(Default, PartialEq, Eq, Debug)]
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
            _ => Err(()),
        }
    }
}
