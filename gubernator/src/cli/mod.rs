use clap::App;

mod apply;
mod config;

fn main() {
    let _matches = App::new("Gubernator")
        .version("0.1.0")
        .author("Samuel Roth <contact@samuelroth.net>")
        .about("Interact with Gubernator clusters")
        .subcommand(apply::subcommand())
        .subcommand(config::subcommand())
        .get_matches();
}
