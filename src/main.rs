use clap::Parser;

use kaniko_login::{Args, get_password};
use kaniko_login::registry::*;


fn main() {
    let input_args = Args::parse();

    let Args {
        password,
        username,
        password_stdin,
        args
    } = input_args;

    let host = args.first();

    let mut registry_host: String;

    if let Some(host) = host {
        registry_host = host.clone();
    } else {
        eprintln!("Host not get");
        return ;
    }

    let docker_registry = DockerRegistry::new(
        registry_host,
        get_password(password, password_stdin),
        username
    );

    let config = Config::new(Registry::from(docker_registry));

    config.save_to_file("config.json".to_string());
}
