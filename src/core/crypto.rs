use anyhow::Result;
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
    let params = ParamsBuilder::new()
        .m_cost(2097152)
        .t_cost(1)
        .p_cost(1)
        .build()?;
    Ok(params)
}

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = get_argon2()?;
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    Ok(password_hash)
}

pub fn validate_hash(hash: &str, password: &str) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(password)?;
    let argon2 = get_argon2()?;
    Ok(argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

pub fn gen_otp() -> String {
    // for now just use a random salt value that's base64-encoded
    SaltString::generate(&mut OsRng).to_string()
}
