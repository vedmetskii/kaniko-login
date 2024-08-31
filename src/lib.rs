pub mod registry;
pub mod panics;

use clap::Parser;
use std::io;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use base64::prelude::*;
use crate::panics::not_password;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    #[arg(short = 'u', long)]
    pub username: String,

    #[arg(short = 'p', long)]
    pub password: Option<String>,

    #[arg(long)]
    pub password_stdin: Option<bool>,

    #[arg()]
    pub args: Vec<String>
}

pub fn get_password(password: Option<String>, password_stdin: Option<bool>) -> String {
    let mut registry_password: String = String::new();

    if password_stdin == None || password_stdin == Some(false) {
        if let Some(pass) = password {
            registry_password = pass;
        } else {
            not_password();
        }
    } else {
        let mut stdin_password = String::new();

        let is_ok = io::stdin().read_line(&mut stdin_password);

        if let Ok(_) = is_ok {
            registry_password = stdin_password.trim().to_string();
        } else {
            not_password();
        }
    }

    registry_password
}