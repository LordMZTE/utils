use std::{
    fs::File,
    io::{BufReader, Read, Write},
};

use anyhow::{Context, Result};
use once_cell::sync::OnceCell;
use serde::Deserialize;

static CONFIG: OnceCell<Config> = OnceCell::new();

pub fn get_config<'a>() -> Result<&'a Config> {
    Ok(match CONFIG.get() {
        None => {
            let data = read_or_init()?;
            let bytes = BufReader::new(data)
                .bytes()
                .collect::<Result<Vec<_>, _>>()?;
            let conf = toml::from_slice(&bytes)?;

            CONFIG.get_or_init(|| conf)
        },
        Some(x) => x,
    })
}

fn read_or_init() -> Result<File> {
    let mut config_file = dirs::config_dir().context("Error getting config dir")?;
    config_file.push("figclock.toml");

    if !config_file.exists() {
        File::create(&config_file)?.write(include_bytes!("../assets/default_config.toml"))?;
    }

    // We cannot use the same file as it would
    // not take into account the default data
    Ok(File::open(&config_file)?)
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub format: String,
    pub update_interval: u64,
    pub font: Option<String>,
}
