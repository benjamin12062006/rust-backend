use pwhash::bcrypt;

use crate::shared::common::error::Result;

pub fn hash_password(password: &str) -> Result<String> {
    Ok(bcrypt::hash(password)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_password_returns_a_hash_different_from_plain_password() {
        let hash = hash_password("secret").expect("password should be hashed");

        assert_ne!(hash, "secret");
        assert!(hash.starts_with("$2"));
    }

    #[test]
    fn hash_password_can_be_verified_with_original_password() {
        let hash = hash_password("secret").expect("password should be hashed");

        assert!(bcrypt::verify("secret", &hash));
        assert!(!bcrypt::verify("wrong-secret", &hash));
    }
}
