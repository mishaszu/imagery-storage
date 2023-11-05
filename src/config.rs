use std::{str::FromStr, sync::OnceLock};

use crate::{crypt::b64u_decode, Error, Result};

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::load_from_env().unwrap_or_else(|e| panic!("Failed to load config from env: {e:}"))
    })
}

#[allow(non_snake_case)]
pub struct Config {
    pub DB_URL: String,
    pub WEB_FOLDER: String,
    pub LUST_URL: String,
    pub TOKEN_SECRET: Vec<u8>,
    pub PWD_KEY: Vec<u8>,
    pub TOKEN_DURATION: i64,
}

impl Config {
    fn load_from_env() -> Result<Config> {
        Ok(Config {
            DB_URL: get_env("DATABASE_URL")?,
            WEB_FOLDER: get_env("WEB_FOLDER")?,
            LUST_URL: get_env("LUST_URL")?,
            TOKEN_SECRET: get_env_b64u_as_u8s("TOKEN_SECRET")?,
            PWD_KEY: get_env_b64u_as_u8s("PWD_KEY")?,
            TOKEN_DURATION: get_env_parse("TOKEN_DURATION")?,
        })
    }
}

fn get_env(key: &str) -> Result<String> {
    std::env::var(key).map_err(|e| Error::EnvVar(key.to_string(), e.to_string()))
}

fn get_env_parse<T: FromStr>(name: &'static str) -> Result<T> {
    let val = get_env(name)?;
    val.parse::<T>()
        .map_err(|_| Error::ConfigWrongFormat(name.to_string()))
}

fn get_env_b64u_as_u8s(name: &'static str) -> Result<Vec<u8>> {
    b64u_decode(&get_env(name)?).map_err(|_| Error::ConfigWrongFormat(name.to_string()))
}
