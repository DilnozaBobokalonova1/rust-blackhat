use crate::Error;

#[derive(Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub database_url: String,
    pub client_identity_public_key: ed25519_dalek::PublicKey,
}

const ENV_DATABASE_URL: &str = "DATABASE_URL";
const ENV_PORT: &str = "PORT";
const ENV_CLIENT_IDENTITY_PUBLIC_KEY: &str = "CLIENT_IDENTITY_PUBLIC_KEY";

const DEFAULT_PORT: u16 = 8080;

impl Config {
    pub fn load() -> Result<Config, Error> {}
}
