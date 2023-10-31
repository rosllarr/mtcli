use std::fmt::Write;

pub struct Shell {
    output: Box<dyn Write>,
}
