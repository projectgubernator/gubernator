use clap::{App, Arg};

pub fn subcommand<'a>() -> App<'a> {
    return App::new("init")
        .about("Initialize a Gubernator cluster")
        .arg(
            Arg::new("debug")
                .short('d')
                .about("print debug information verbosely"),
        );
}
