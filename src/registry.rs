use std::collections::HashMap;
use serde::{Deserialize, Serialize, Serializer};
use serde::ser::{SerializeStruct};
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

#[derive(Deserialize, Debug)]
pub struct Config {
    auths: HashMap<String, Auth>
}

impl From<Vec<DockerRegistry>> for Config {
    fn from(value: Vec<DockerRegistry>) -> Self {
        let mut auths = HashMap::new();
        for registry in value.into_iter() {
            auths.insert(registry.host, Auth::new(registry.username, registry.password));
        }

        Config {
            auths
        }
    }
}

impl Serialize for Config {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("NewConfig", 1)?;
        s.serialize_field("auths", &self.auths)?;
        s.end()
    }
}

impl Config {
    pub fn get_auths(self) -> HashMap<String, Auth> {
        let mut answer = HashMap::new();

        for (key, value) in self.auths.into_iter() {
            answer.insert(key, value);
        }

        answer
    }

    pub fn add_from_other(&mut self, other: Config) {
        for (key, value) in other.get_auths() {
            if let Some(_) = self.auths.get(&key) {
                continue;
            }
            self.auths.insert(key, value);
        }
    }

    pub fn save_to_file(&self, path: String) {
        let output = serde_json::to_string(&self);

        if let Ok(j) = output {
            let file = File::create(path.as_str());

            match file {
                Ok(mut file) => {
                    let _res = file.write(j.as_bytes());
                }
                Err(_e) => {}
            }
        }
    }
}

#[derive(Deserialize, Debug)]
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
