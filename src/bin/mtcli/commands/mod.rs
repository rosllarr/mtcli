use clap::Command;

pub mod apply;
pub mod delete;

pub fn builtin() -> Vec<Command> {
    vec![apply::cli(), delete::cli()]
}
