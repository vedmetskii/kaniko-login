use std::process::exit;

pub fn not_password() {
    eprintln!("Error: Password Required");
    exit(1);
}
