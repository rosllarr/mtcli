use crate::Shell;
use std::cell::RefCell;

pub struct Config {
    shell: RefCell<Shell>,
}
