use clap::{App, Arg};

pub fn subcommand<'a>() -> App<'a> {
    return App::new("config").about("Manage Gubernator config").arg(
        Arg::new("debug")
            .short('d')
            .about("print debug information verbosely"),
    );
}
