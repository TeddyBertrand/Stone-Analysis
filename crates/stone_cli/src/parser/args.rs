use super::mode::Mode;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ParsedArgs {
    pub flags: HashMap<&'static str, String>,
    pub positionals: Vec<String>,
    pub mode: Mode,
}

impl ParsedArgs {
    pub fn value(&self, long: &'static str) -> Option<&str> {
        self.flags.get(long).map(String::as_str)
    }

    pub fn has(&self, long: &'static str) -> bool {
        self.flags.contains_key(long)
    }

    pub fn positional(&self, index: usize) -> Option<&str> {
        self.positionals.get(index).map(String::as_str)
    }

    pub fn positionals(&self) -> &[String] {
        &self.positionals
    }
}
