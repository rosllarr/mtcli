use crate::Shell;
use anyhow::Context as _;
use std::cell::RefCell;
use std::env;
use std::path::PathBuf;

pub struct Config {
    shell: RefCell<Shell>,
    cwd: PathBuf,
}

impl Config {
    pub fn new(shell: Shell, cwd: PathBuf) -> Config {
        Config {
            shell: RefCell::new(shell),
            cwd,
        }
    }

    // Create a new Config instance, with all default settings.
    pub fn default(&mut self) -> anyhow::Result<Config> {
        let shell = Shell::new(Box::new(Vec::new()));
        let cwd = env::current_dir()
            .with_context(|| "couldn't get the current directory of the process")?;
        Ok(Config::new(shell, cwd))
    }
}
