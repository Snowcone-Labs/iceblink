use crate::models::{codes::Code, user::User};
use rand::distributions::{Alphanumeric, DistString};
use sha2::{Digest, Sha256};

pub fn generate_id(len: usize) -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), len)
}

pub fn checksum(codes: Vec<Code>, _user: &User) -> String {
    let mut content = "".to_owned();

    for code in codes {
        content += &code.fmt_for_hasher()
    }

    crc32fast::hash(content.as_bytes()).to_string()
}

pub fn hash_domain(domain: &str) -> String {
    base16ct::lower::encode_string(&Sha256::digest(domain))
}

pub const USER_AGENT: &str = concat!("Snowcone-Labs/Iceblink/", env!("CARGO_PKG_VERSION"));

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn test_generate_id() {
        let regex = Regex::new(r"^[a-zA-Z0-9]*$").unwrap();
        let mut ids: Vec<String> = vec![];

        for _ in 0..500 {
            let id = generate_id(16);

            assert_eq!(id.len(), 16);
            assert!(regex.is_match(id.as_str()));
            assert!(!ids.contains(&id));

            ids.push(id.clone());
        }

        for x in 0..100 {
            let id = generate_id(x);
            assert_eq!(id.len(), x);
            assert!(regex.is_match(id.as_str()));
        }
    }

    #[test]
    fn hash_domain_always_returns_same() {
        let hash1 = hash_domain("google.com");

        for _ in 1..25 {
            assert_eq!(hash1, hash_domain("google.com"))
        }
    }
}
