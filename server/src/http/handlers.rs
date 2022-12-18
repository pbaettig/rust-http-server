use crate::http::{Method, Request};
use glob::Pattern;
type HandlerFn = fn(Request);

pub struct Handlers {
    handlers: Vec<(Method, Pattern, HandlerFn)>,
}

impl Handlers {
    pub fn new() -> Self {
        let h = Handlers { handlers: vec![] };

        return h;
    }

    pub fn get(&self, m: &Method, uri: &str) -> Result<(usize, HandlerFn), ()> {
        for (i, (rm, rp, hf)) in self.handlers.iter().enumerate() {
            if *rm == *m && rp.matches(uri) {
                return Ok((i, *hf));
            }
        }

        Err(())
    }

    pub fn register(&mut self, m: Method, p: Pattern, h: HandlerFn) {
        let idx = self.handlers.iter().position(|x| {
            let (rm, rp, _) = &x;
            *rm == m && *rp == p
        });
        match idx {
            Some(i) => {
                self.handlers[i] = (m, p, h);
            }
            None => self.handlers.push((m, p, h)),
        };
    }
}
