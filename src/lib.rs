use base64;
use rand_core::OsRng;
use x25519_dalek::{PublicKey, StaticSecret};
use regex::Regex;


pub fn trial_substring(prefix: &str, start: usize, end: usize) -> Option<(String, String)> {
    let private = StaticSecret::new(&mut OsRng);
    let public = PublicKey::from(&private);
    let public_b64 = base64::encode(public.as_bytes());
    if public_b64[start..end]
        .to_ascii_lowercase()
        .contains(&prefix)
    {
        let private_b64 = base64::encode(&private.to_bytes());
        Some((private_b64, public_b64))
    } else {
        None
    }
}


pub fn trial_regex(pattern: &Regex) -> Option<(String, String)> {
    let private = StaticSecret::new(&mut OsRng);
    let public = PublicKey::from(&private);
    let public_b64 = base64::encode(public.as_bytes());
    if pattern.is_match(&public_b64)
    {
        let private_b64 = base64::encode(&private.to_bytes());
        Some((private_b64, public_b64))
    } else {
        None
    }
}
