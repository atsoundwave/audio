use rand::distributions::{Alphanumeric, DistString};
use snowflake::ProcessUniqueId;

pub fn hash(password: &str) -> String {
    bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap()
}

pub fn verify(password: &str, hash: &str) -> bool {
    bcrypt::verify(password, hash).unwrap()
}

pub fn random_id() -> String {
    ProcessUniqueId::new().to_string().replace("-", "")
}

pub fn generate_token() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 64)
}
