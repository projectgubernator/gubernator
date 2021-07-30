use clap::App;

mod auth;
mod config;
mod init;

fn main() {
    let _matches = App::new("Gubernator")
        .version("0.1.0")
        .author("Samuel Roth <contact@@samuelroth.net>")
        .about("Interact with Gubernator clusters")
        .subcommand(auth::subcommand())
        .subcommand(config::subcommand())
        .subcommand(init::subcommand())
        .get_matches();
}
