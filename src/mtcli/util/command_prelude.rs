pub use clap::{value_parser, Arg, ArgAction, ArgMatches};

pub trait ArgMatchesExt {
    fn flag(&self, name: &str) -> bool;
}

impl ArgMatchesExt for ArgMatches {
    fn flag(&self, name: &str) -> bool {
        ignore_unknown(self.try_get_one::<bool>(name))
            .copied()
            .unwrap_or(false)
    }
}

pub fn ignore_unknown<T: Default>(r: Result<T, clap::parser::MatchesError>) -> T {
    match r {
        Ok(t) => t,
        Err(clap::parser::MatchesError::UnknownArgument { .. }) => Default::default(),
        Err(e) => {
            panic!("Mismatch between definition and access: {}", e);
        }
    }
}
