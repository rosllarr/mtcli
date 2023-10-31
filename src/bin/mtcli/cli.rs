use clap::Command;
use std::io::Error;

use mtcli::Config;

use super::commands;

pub fn main(config: &mut Config) -> Result<(), Error> {
    let args = cli().try_get_matches()?;
}

pub fn cli() -> Command {
    Command::new("mtcli").subcommands(commands::builtin())
}
