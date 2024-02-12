use anyhow::{Error, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Algorithm, Argon2, Params, ParamsBuilder, Version,
};

// TODO: use lazy_static here or add to AppState?
fn get_argon2() -> Result<Argon2<'static>, argon2::password_hash::Error> {
    Ok(Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        argon2_params()?,
    ))
}

fn argon2_params() -> Result<Params, argon2::password_hash::Error> {
    // RFC9106 - can also do t=3 m=65536 for lower memory usage
    // OWASP suggests some more conservative values here:
    // https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html#argon2id
    let params = ParamsBuilder::new()
        .m_cost(2097152)
        .t_cost(1)
        .p_cost(1)
        .build()?;
    Ok(params)
}

pub fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    // we consume argon2 errors here because they don't play nicely with anyhow
    if let Ok(argon2) = get_argon2() {
        if let Ok(password_hash) = argon2.hash_password(password.as_bytes(), &salt) {
            return Ok(password_hash.to_string());
        }
    }
    Err(Error::msg("Argon2 error"))
}

pub fn validate_hash(hash: &str, password: &str) -> Result<bool> {
    // we consume argon2 errors here because they don't play nicely with anyhow
    if let Ok(parsed_hash) = PasswordHash::new(hash) {
        if let Ok(argon2) = get_argon2() {
            return Ok(argon2
                .verify_password(password.as_bytes(), &parsed_hash)
                .is_ok());
        }
    } else {
        // at least differentiate if the hash is unparseable (other conditions should be basically
        // impossible to trigger)
        return Err(Error::msg("Unable to parse Argon2 hash"));
    }
    Err(Error::msg("Argon2 error"))
}

pub fn gen_otp() -> String {
    // for now just use a random salt value that's base64-encoded
    // we replace some symbols that can cause issues in URLs with hyphens
    // TODO: make this nicer and URL-safe
    SaltString::generate(&mut OsRng)
        .to_string()
        .replace(['+', '/'], "-")
}
