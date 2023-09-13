use std::time;

use rand::distributions::{Alphanumeric, DistString};
use flaken::Flaken;

pub fn hash(password: &str) -> String {
    bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap()
}

pub fn verify(password: &str, hash: &str) -> bool {
    bcrypt::verify(password, hash).unwrap()
}

pub fn random_id() -> String {
    let ts = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap();
    let ts_ms = ts.as_secs() * 1000 + (ts.subsec_nanos() as u64) / 1000000;
    let mf = Flaken::default();
    mf.encode(ts_ms, 10, 100).to_string()
}

pub fn generate_token() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 64)
}
