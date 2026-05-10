use argon2::{
    password_hash::{
        phc::PasswordHash, PasswordHasher, PasswordVerifier
    },
    Argon2,
};

pub fn hash_password(password: &str) -> String {

    let hash = Argon2::default()
        .hash_password(password.as_bytes())
        .expect("hash failed")
        .to_string();

    hash
}

pub fn verify_password(hash: &str, password: &str) -> bool {
    let parsed = PasswordHash::new(hash).ok();
    if parsed.is_none() {
        return false;
    }

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed.unwrap())
        .is_ok()
}