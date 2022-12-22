use std::io::Write;

use crate::{
    http::{Method, Request},
    Response, Status,
};
use glob::Pattern;

pub struct Handler<T: Write> {
    pub methods: Vec<Method>,
    pub pattern: Pattern,
    pub func: fn(Request, T) -> Result<usize, std::io::Error>,
}

impl<T: Write> Handler<T> {
    pub fn handle(&self, r: Request, w: T) -> Result<usize, std::io::Error> {
        (self.func)(r, w)
    }
}

pub struct Handlers<T: Write> {
    handlers: Vec<Handler<T>>,
    default_handler: Handler<T>,
}

impl<T: Write> Handlers<T> {
    pub fn new() -> Self {
        let h = Handlers {
            handlers: vec![],
            default_handler: Handler {
                methods: vec![Method::GET],
                pattern: Pattern::new("/").unwrap(),
                func: |r, w: T| {
                    let mut resp = Response::new(
                        Status::NotFound,
                        &format!(
                            "no handler found for {} {}",
                            r.method.to_string(),
                            r.uri.path
                        ),
                    );

                    resp.write_to(w)
                },
            },
        };
        return h;
    }

    pub fn get(&self, m: &Method, uri: &str) -> (bool, &Handler<T>) {
        for h in self.handlers.iter() {
            if h.methods.contains(m) && h.pattern.matches(uri) {
                return (true, h);
            }
        }

        (false, &self.default_handler)
    }

    pub fn register(&mut self, h: Handler<T>) {
        self.handlers.insert(0, h)
    }
}
