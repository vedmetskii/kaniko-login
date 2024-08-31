use clap::Parser;

use kaniko_login::prelude::*;


fn main() {
    let input_args = Args::parse();

    let Args {
        password,
        username,
        password_stdin,
        host
    } = input_args;

    let docker_registry = DockerRegistry::new(
        host,
        get_password(password, password_stdin),
        username
    );

    let config = Config::new(Registry::from(docker_registry));

    config.save_to_file("config.json".to_string());
}
