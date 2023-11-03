use clap::Command;

use mtcli::command_prelude::*;
use mtcli::drop_print;
use mtcli::Config;

use super::commands;

pub fn main(config: &mut Config) -> anyhow::Result<()> {
    let args = cli().try_get_matches()?;

    let config = config.default().unwrap();

    if args.flag("version") {
        let version = get_version_string();
        drop_print!(config, "{}", version);
        return Ok(());
    }

    Ok(())
}

pub fn cli() -> Command {
    Command::new("mtcli").subcommands(commands::builtin())
}

pub fn get_version_string() -> String {
    let version = mtcli::version();
    let mut version_string = format!("cargo {}\n", version);
    version_string
}
