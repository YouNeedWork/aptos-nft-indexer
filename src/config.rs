use anyhow::{anyhow, Result};
use config::{Config, File, FileFormat};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct IndexConfig<'a> {
    pub posgres: &'a str,
    pub work_number: i32,
}

impl<'a> Default for IndexConfig<'a> {
    fn default() -> Self {
        Self {
            posgres: "",
            work_number: 4,
        }
    }
}

impl<'a> IndexConfig<'a> {
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
