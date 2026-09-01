pub mod loader;

use std::fs;
use std::sync::{OnceLock, RwLock};
use std::sync::atomic::AtomicBool;
use reqwest::{blocking::Client, Error, StatusCode};
use serde::{Deserialize, Serialize};
use crate::config::loader::load_config;
use crate::misc::exe_dir;

pub static CONFIG: OnceLock<RwLock<Config>> = OnceLock::new();
pub static FROM_REMOTE: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub network: NetworkConfig,
    pub encryption: EncryptionConfig,
    pub fps: FpsConfig,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct FpsConfig {
    pub target_max_fps: i32,
    pub enabled: bool
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct NetworkConfig {
    pub address: String,
    pub port: i32,
    pub https: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct EncryptionConfig {
    pub sdk_key: String,
    pub check_sign_key: String,
    pub use_sdk_rsa: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            network: NetworkConfig::default(),
            encryption: EncryptionConfig::default(),
            fps: FpsConfig::default(),
        }
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            address: "localhost".to_string(),
            port: 8080,
            https: false,
        }
    }
}

impl Default for EncryptionConfig {
    fn default() -> Self {
        Self {
            sdk_key: include_str!("../../default_sdk_key.xml").to_string(),
            check_sign_key: include_str!("../../default_check_sign_key.pem").to_string(),
            use_sdk_rsa: true,
        }
    }
}

impl Default for FpsConfig {
    fn default() -> Self {
        Self {
            target_max_fps: 60,
            enabled: false,
        }
    }
}

pub fn save_config() -> std::io::Result<()> {
    let config = CONFIG.get().unwrap().read()
        .expect("Config lock is broken");

    let exe_path = exe_dir().expect("couldn't detect the executable folder. open an issue!");
    let config_path = exe_path.join("config.toml");

    let toml = toml::to_string_pretty(&*config).map_err(std::io::Error::other)?;

    fs::write(config_path, toml)?;

    drop(config);

    // reload config
    Ok(load_config())
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