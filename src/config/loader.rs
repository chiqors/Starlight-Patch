use std::fs::File;
use std::io::{Read, Write};
use std::str::FromStr;
use std::sync::atomic::Ordering;
use std::sync::RwLock;
use crate::config::{get_remote_key_config, Config, EncryptionConfig, CONFIG, FROM_REMOTE};
use crate::misc::exe_dir;

pub fn load_config() {
    tracing::info!("Detecting settings...");

    let exe_path = exe_dir().expect("couldn't detect the executable folder. open an issue!");
    let config_path = exe_path.join("config.toml");

    if !config_path.exists() {
        tracing::warn!("The settings file doesn't exist, or couldn't be found for the current installation. Creating one for you...");

        let config = Config::default();
        let result = toml::to_string(&config).expect("couldn't serialize the config file!");
        let mut file = File::create(&config_path).unwrap();

        file.write_all(result.as_bytes()).expect("couldn't write the config file!");
    }

    let mut file = File::open(&config_path).expect("couldn't open the config file!");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("couldn't read the config file!");

    let mut config = toml::from_str::<Config>(contents.as_str()).expect("couldn't deserialize the config file!");

    // apply local overrides
    let args: Vec<String> = std::env::args().collect();

    if let Some((_, addr)) = args
        .windows(2)
        .find(|w| w[0] == "--ps-addr")
        .map(|w| (&w[0], &w[1]))
    {
        config.network.address = addr.to_string();
        tracing::debug!("Override applied to address: {addr}");
    }

    if let Some((_, port)) = args
        .windows(2)
        .find(|w| w[0] == "--ps-port")
        .map(|w| (&w[0], &w[1]))
    {
        config.network.port = i32::from_str(port).unwrap();
        tracing::debug!("Override applied to port: {port}");
    }

    // apply remote overrides
    let remote_cfg = get_remote_key_config(&config.network.address, &config.network.port);

    if let Ok(remote_cfg) = remote_cfg {
        let deserialized = serde_json::from_str::<EncryptionConfig>(remote_cfg.as_str());

        if let Ok(deserialized) = deserialized {
            config.encryption = deserialized;
            FROM_REMOTE.store(true, Ordering::Relaxed);
        }
    }

    CONFIG.set(RwLock::new(config)).unwrap();
}