use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::Serialize;

use crate::hash::sha512;

pub fn gen(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

#[derive(Serialize)]
pub struct Pass {
    plaintext: String,
    sha512: String,
}

impl Pass {
    pub fn new(length: usize) -> Self {
        let plaintext = gen(length);
        let salt = gen(16);
        let hash = sha512(&plaintext, &salt);
        let sha512 = format!("$6${}${}", salt, hash);
        Self { plaintext, sha512 }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_produces_legit_looking_data() {
        let length: usize = 13;
        let pass = Pass::new(length);

        assert_eq!(&pass.plaintext.len(), &length);

        let mut hash_components: Vec<&str> = pass.sha512.split('$').collect();

        assert_eq!(hash_components.len(), 4);
        let hash = hash_components.pop().unwrap();
        let salt = hash_components.pop().unwrap();
        let algo = hash_components.pop();
        let nothing = hash_components.pop();

        assert_eq!(nothing, Some(""));
        assert_eq!(algo, Some("6"));
        // legit salt would be between 8 and 16 chars, we use 16 in this crate
        assert_eq!(salt.len(), 16_usize);
        // a sha512 in base 64 is 86 chars, or 88 with padding
        assert_eq!(hash.len(), 86_usize);
    }
}
