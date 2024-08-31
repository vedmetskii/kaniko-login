use serde::{Serialize, Serializer};
use serde::ser::{SerializeMap, SerializeStruct};
use base64::prelude::*;
use std::fs::File;
use std::io::Write;

pub struct DockerRegistry {
    host: String,
    password: String,
    username: String
}

impl DockerRegistry {
    pub fn new(host: String, password: String, username: String) -> Self {
        DockerRegistry {
            host,
            password,
            username
        }
    }
}

pub struct Registry {
    registry: String,
    auth: Auth
}

impl Serialize for Registry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_map(Some(1))?;
        s.serialize_entry(&self.registry, &self.auth)?;
        s.end()
    }
}

impl From<DockerRegistry> for Registry {
    fn from(value: DockerRegistry) -> Self {
        Registry {
            auth: Auth::new(value.username, value.password),
            registry: value.host
        }
    }
}

pub struct Auth {
    auth: String
}

impl Auth {
    pub fn new(username: String, password: String) -> Self {
        Auth {
            auth: BASE64_STANDARD.encode(format!("{}:{}", username, password))
        }
    }
}

impl Serialize for Auth {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Auth", 1)?;
        s.serialize_field("auth", &self.auth)?;
        s.end()
    }
}

#[derive(Serialize)]
pub struct Config {
    auths: Registry
}

impl Config {
    pub fn new(auths: Registry) -> Self {
        Config { auths }
    }

    pub fn save_to_file(&self, path: String) {
        let output = serde_json::to_string(&self);

        if let Ok(j) = output {
            let mut file = File::create(path.as_str());

            match file {
                Ok(mut file) => {
                    let res = file.write(j.as_bytes());
                }
                Err(e) => {}
            }
        }
    }
}

