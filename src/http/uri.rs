use std::collections::{hash_map::RandomState, HashMap};

// TODO: Add support for multiple values with the same name
type QueryParams = HashMap<String, Option<String>, RandomState>;

#[derive(Default, Debug)]
pub struct Uri {
    pub path: String,
    pub params: QueryParams,
    pub raw: String,
}

impl std::str::FromStr for Uri {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (path, query) = match s.split_once('?') {
            Some(p) => p,
            None => (s, ""),
        };

        let mut params: QueryParams = HashMap::new();
        for p in query.split('&') {
            let Some((k,v)) = p.split_once('=').or(Some((p, ""))) else {
                continue;
            };

            params.insert(
                k.to_string(),
                match v {
                    "" => None,
                    s => Some(s.to_string()),
                },
            );
        }

        Ok(Uri {
            path: path.to_string(),
            params,
            raw: s.to_string(),
        })
    }
}

impl std::string::ToString for Uri {
    fn to_string(&self) -> String {
        self.raw.to_string()
    }
}
