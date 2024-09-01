use std::process::exit;

pub fn not_password() {
    eprintln!("Error: Password Required");
    exit(1);
}

pub fn config_not_found() {
    eprintln!("Error: File of config not found");
    exit(1);
}

pub fn config_is_not_allowed() {
    eprintln!("Error: Config is not allowed");
    exit(1);
}