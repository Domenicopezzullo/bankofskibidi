use dioxus::{logger::tracing, prelude::*};

pub fn add_user(username: &str, password_hash: &str) {
    tracing::info!("Adding user {} with password hash {}", username, password_hash);
}

pub fn get_user<'a>(username: &str) -> Option<(&'a str, &'a str)> {
    None
}