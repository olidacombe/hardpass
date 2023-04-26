use sha_crypt::{sha512_crypt_b64, Sha512Params};

pub fn sha512(password: &str, salt: &str) -> String {
    sha512_crypt_b64(
        password.as_bytes(),
        salt.as_bytes(),
        &Sha512Params::default(),
    )
    .unwrap()
}

mod test {
    use super::*;

    #[test]
    fn or_sha512_hashes_correctly() {
        let hash = sha512("booboo", "blablabla");
        assert_eq!(hash,"8FEsMpJwyF5zn0A.L.5TlCZnB9cjMNoPyi5.Txixlwd3DgoiY4AWObVO8isGC.N5ITUQe.q4sjhlzbkG6YHZR.");
    }
}
