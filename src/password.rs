use std::io;
use crate::panics::not_password;

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