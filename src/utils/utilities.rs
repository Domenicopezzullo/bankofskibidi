use std::hash::DefaultHasher;
use std::hash::Hasher;


pub fn calculate_hash(password: &str) -> String {
    let mut hasher = DefaultHasher::new();
    hasher.write(password.as_bytes());
    hasher.finish().to_string()
}