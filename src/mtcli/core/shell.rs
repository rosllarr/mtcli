use std::io::prelude::*;

pub struct Shell {
    output: Box<dyn Write>,
}

impl Shell {
    pub fn new(out: Box<dyn Write>) -> Shell {
        Shell { output: out }
    }
}
