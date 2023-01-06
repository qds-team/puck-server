use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    return argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string();
}

pub fn verify_password(password_hash: &str, password: &str) -> bool {
    let parsed_hash = PasswordHash::new(&password_hash).unwrap();
    return Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok(); 
}