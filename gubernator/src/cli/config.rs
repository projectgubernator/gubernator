use clap::{App, Arg};

pub fn subcommand<'a>() -> App<'a> {
    return App::new("config").about("Manage Gubernator config").arg(
        Arg::new("debug")
            .short('d')
            .help("print debug information verbosely"),
    );
}
