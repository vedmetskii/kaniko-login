mod registry;
mod panics;
mod password;

pub mod prelude {
    pub use crate::password::get_password;
    pub use crate::registry::{Registry, DockerRegistry, Config};
    pub use crate::Args;
}

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = "Cli util for registry kaniko in docker registry")]
pub struct Args {
    #[arg(short = 'u', long)]
    pub username: String,

    #[arg(short = 'p', long)]
    pub password: Option<String>,

    #[arg(long)]
    pub password_stdin: Option<bool>,

    #[arg()]
    pub host: String
}