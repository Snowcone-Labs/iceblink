use rand::distributions::{Alphanumeric, DistString};

pub fn generate_id(len: usize) -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), len)
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn test_generate_id() {
        let regex = Regex::new(r"^[a-zA-Z0-9]*$").unwrap();
        let ids: Vec<String> = vec![];

        for _ in 0..50 {
            let id = generate_id(16);
            assert!(regex.is_match(id.as_str()));
            assert_eq!(id.len(), 16);
            assert!(!ids.contains(&id));
        }

        for x in 0..10 {
            let id = generate_id(x);
            assert_eq!(id.len(), x);
        }
    }
}
