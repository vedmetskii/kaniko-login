mod registry;
mod panics;
mod password;

pub mod prelude {
    pub use crate::password::get_password;
    pub use crate::registry::{DockerRegistry, Config};
    pub use crate::Args;
    pub use crate::get_exist_config;
}

use clap::Parser;
use crate::panics::{config_is_not_allowed, config_not_found};
use crate::registry::Config;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "Cli util for registry kaniko in docker registry")]
pub struct Args {
    #[arg(short = 'u', long)]
    pub username: String,

    #[arg(short, long)]
    pub config: Option<String>,

    #[arg(long)]
    pub output_file: Option<String>,

    #[arg(short, long)]
    pub password: Option<String>,

    #[arg(long)]
    pub password_stdin: bool,

    #[arg()]
    pub host: String
}

pub fn get_exist_config(path: String) -> Option<Config> {
    let res_json = std::fs::read_to_string(path);

    if let Ok(json) = res_json {
        let res_config: Result<Config, _> = serde_json::from_str(json.as_str());

        match res_config {
            Ok(config) => {
                Some(config)
            },
            Err(_) => {
                config_is_not_allowed();
                None
            }
        }
    } else {
        config_not_found();
        None
    }
}