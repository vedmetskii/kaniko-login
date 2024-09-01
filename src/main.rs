use clap::Parser;

use kaniko_login::prelude::*;


fn main() {
    let input_args = Args::parse();

    let Args {
        password,
        username,
        password_stdin,
        host,
        config,
        output_file,
    } = input_args;

    let path_to_config = config;

    let docker_registry = DockerRegistry::new(
        host,
        get_password(password, password_stdin),
        username
    );

    let mut config = Config::from(vec![docker_registry]);

    let exist_config;

    if let Some(c) = path_to_config {
        exist_config = get_exist_config(c).unwrap();
        config.add_from_other(exist_config);
    }

    if let Some(file) = output_file {
        config.save_to_file(file);
    } else {
        config.save_to_file("config.json".to_string());
    }
}
