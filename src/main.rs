use clap::Parser;
use std::io;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use base64::prelude::*;
use std::fs;
use std::io::{Error, Write};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short = 'u', long)]
    username: String,

    #[arg(short = 'p', long)]
    password: Option<String>,

    #[arg(long)]
    password_stdin: Option<bool>,

    #[arg()]
    args: Vec<String>
}

fn not_password() {
    eprintln!("Password not get");
    return;
}

fn get_password(password: Option<String>, password_stdin: Option<bool>) -> Option<String> {
    let mut registry_password: String = String::new();

    if password_stdin == None || password_stdin == Some(false) {
        if let Some(pass) = password {
            registry_password = pass;
        } else {
            not_password();
            return None;
        }
    } else {
        let mut stdin_password = String::new();

        let is_ok = io::stdin().read_line(&mut stdin_password);

        if let Ok(_) = is_ok {
            registry_password = stdin_password.trim().to_string();
        } else {
            not_password();
            return None;
        }
    }

    Some(registry_password)
}

struct DockerRegistry {
    host: String,
    password: String,
    username: String
}

struct Auth {
    registry: String,
    auth: String
}

impl Serialize for Auth {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Auth", 1)?;
        s.serialize_field("registry", &self.auth)?;
        s.end()
    }
}

#[derive(Serialize)]
struct Config {
    auths: Vec<Auth>
}

impl From<DockerRegistry> for Auth {
    fn from(value: DockerRegistry) -> Self {
        let input = format!("{}:{}", value.username, value.password);

        dbg!(&input);

        let auth = BASE64_STANDARD.encode(input);

        dbg!(&auth);

        Auth {
            auth,
            registry: value.host
        }
    }
}

fn main() -> Result<(), Error> {
    let input_args = Args::parse();

    let Args {
        password,
        username,
        password_stdin,
        args
    } = input_args;

    let mut registry_password: String;

    if let Some(password) = get_password(password, password_stdin) {
        registry_password = password;
    } else {
        return Ok(());
    }

    let host = args.get(0);

    let mut registry_host: String;

    if let Some(host) = host {
        registry_host = host.clone();
    } else {
        eprintln!("Host not get");
        return Ok(());
    }

    let docker_registry = DockerRegistry {
        host: registry_host,
        password: registry_password,
        username
    };

    let config = Config {
        auths: vec![Auth::from(docker_registry)]
    };

    let output = serde_json::to_string(&config);

    if let Ok(j) = output {
        let mut file = fs::File::create("output.json")?;
        file.write(j.as_bytes())?;
    }

    Ok(())
}
