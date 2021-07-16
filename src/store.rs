use std::time::Instant;

pub struct Order<'a> {
    name: &'a str,
    item: &'a str,
    // time:
}

impl Order<'_> {
    pub fn new(name: &str, item: &str) -> Self {
        Order { name, item }
    }
}
