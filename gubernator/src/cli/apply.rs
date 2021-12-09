use clap::{App, Arg};

pub fn subcommand<'a>() -> App<'a> {
    return App::new("apply").about("Apply a Gubernator manifest").arg(
        Arg::new("file")
            .short('f')
            .help("The manifest file path")
            .required(true),
    );
}
