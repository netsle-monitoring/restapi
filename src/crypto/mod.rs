use ring::{digest, pbkdf2};
mod config;

static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
type Credential = [u8; CREDENTIAL_LEN];

pub enum Error {
    WrongUsernameOrPassword,
}

/**
 * Hash a password based on the username of the client
 */
pub fn hash_password(username: &str, password: &str) -> (Vec<u8>, Vec<u8>) {
    let salt = generate_salt(username);
    let mut final_hash: Credential = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        PBKDF2_ALG,
        config::PBKDF2_ITERATIONS,
        &salt,
        password.as_bytes(),
        &mut final_hash,
    );

    (salt, final_hash.to_vec())
}

pub fn verify_password(
    username: &str,
    hashed_password: Vec<u8>,
    attempted_password: &str,
) -> Result<(), Error> {
    let salt = generate_salt(&username);
    pbkdf2::verify(
        PBKDF2_ALG,
        config::PBKDF2_ITERATIONS,
        &salt,
        (&attempted_password).as_bytes(),
        &hashed_password,
    )
    .map_err(|_| Error::WrongUsernameOrPassword)
}

/**
 * Have a salt based on the username of the client.
 */
fn generate_salt(username: &str) -> Vec<u8> {
    let mut salt = Vec::with_capacity(config::DB_SALT_COMPONENT.len() + username.as_bytes().len());
    salt.extend(config::DB_SALT_COMPONENT.as_ref());
    salt.extend(username.as_bytes());
    salt
}
