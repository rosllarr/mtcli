use mtcli::Config;

pub mod cli;
pub mod commands;

fn main() {
    let mut config: Option<Config> = None;
    cli::main(&mut config)
}
