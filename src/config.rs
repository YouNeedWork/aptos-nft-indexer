use anyhow::{anyhow, Result};
use config::{Config, File, FileFormat};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IndexConfig {
    pub market_posgres: String,
    pub indexer_db_posgres: String,
    pub work_number: i32,
    pub key: String,
    pub secret: String,
    pub fetch_millis: i32,
}

impl Default for IndexConfig {
    fn default() -> Self {
        Self {
            market_posgres: String::from(""),
            indexer_db_posgres: String::from(""),
            work_number: 4,
            key: String::from(""),
            secret: String::from(""),
            fetch_millis: 1000,
        }
    }
}

impl IndexConfig {
    pub fn new() -> Result<Self> {
        let config_build = Config::builder()
            .set_default("default", "1")?
            .add_source(File::new("config.yaml", FileFormat::Yaml))
            .set_override("override", "1")?
            .build()?;

        config_build
            .try_deserialize::<Self>()
            .map_err(|e| anyhow!(e))
    }
}
