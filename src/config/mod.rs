pub mod loader;

use std::sync::OnceLock;
use reqwest::{blocking::Client, Error, StatusCode};
use serde::{Deserialize, Serialize};

pub static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub network: NetworkConfig,
    pub encryption: EncryptionConfig
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub address: String,
    pub port: i32,
    pub https: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EncryptionConfig {
    pub sdk_key: String,
    pub check_sign_key: String,
    pub use_sdk_rsa: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            network: NetworkConfig {
                address: "localhost".to_string(),
                port: 8080,
                https: false,
            },
            encryption: EncryptionConfig {
                sdk_key: include_str!("../../default_sdk_key.xml").to_string(),
                check_sign_key: include_str!("../../default_check_sign_key.pem").to_string(),
                use_sdk_rsa: true
            }
        }
    }
}

pub fn get_remote_key_config(addr: &String, port: &i32) -> Result<String, Error> {
    let client = Client::new();

    let http_url = format!("http://{addr}:{port}/starlight/patchConfig");
    let https_url = format!("https://{addr}:{port}/starlight/patchConfig");

    let response = match client.get(&http_url).send() {
        Ok(resp) => resp,
        Err(_) => client.get(&https_url).send()?,
    };

    if response.status() == StatusCode::NOT_FOUND {
        tracing::warn!("No remote patch config found! Is this a Starlight server? Defaulting to Grasscutter keys.");
        return Ok(String::new());
    }

    Ok(response.error_for_status()?.text()?)
}