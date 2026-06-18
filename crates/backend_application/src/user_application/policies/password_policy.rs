use pwhash::bcrypt;

use crate::shared::common::error::Result;

pub fn hash_password(password: &str) -> Result<String> {
    Ok(bcrypt::hash(password)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::basic_password("secret")]
    #[case::password_with_symbols("S3cret!")]
    fn hash_password_returns_a_hash_different_from_plain_password(#[case] password: &str) {
        let hash = hash_password(password).expect("password should be hashed");

        assert_ne!(hash, password);
        assert!(hash.starts_with("$2"));
    }

    #[rstest]
    #[case::original_password("secret", "secret", true)]
    #[case::wrong_password("secret", "wrong-secret", false)]
    fn hash_password_verification_behaves_as_expected(
        #[case] password: &str,
        #[case] candidate: &str,
        #[case] expected: bool,
    ) {
        let hash = hash_password(password).expect("password should be hashed");

        assert_eq!(bcrypt::verify(candidate, &hash), expected);
    }
}
