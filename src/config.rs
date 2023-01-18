use anyhow::{anyhow, Result};
use config::{Config, File, FileFormat};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct IndexConfig {}

impl Default for IndexConfig {
    fn default() -> Self {
        Self {}
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
