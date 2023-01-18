use anyhow::Result;
use tokio::runtime::Runtime;
use tokio::{task, task::JoinHandle};

use crate::config::IndexConfig;
pub struct Service {}

impl Service {
    pub async fn run(&self) -> Result<()> {
        Ok(())
    }
}

pub struct IndexerService {
    cfg: IndexConfig,
    servers: Vec<Service>,
}

impl IndexerService {
    pub fn new(cfg: IndexConfig) -> Self {
        log::info!("IndexService init");
        Self {
            cfg,
            servers: vec![],
        }
    }

    pub fn run(&self) -> Result<()> {
        log::info!("IndexService Runing");

        let rt = Runtime::new()?;

        rt.block_on(async move { self.start_indexer().await });

        log::info!("IndexService Ended");
        Ok(())
    }

    async fn start_indexer(&self) -> Result<()> {
        // start all indexer include fetch nft collection,metadata,owner,rolay,creater address, more table.

        // let services_joinhandle = s
        //     .servers
        //     .iter()
        //     .collect::<Vec<JoinHandle<()>>>();

        todo!();

        Ok(())
    }
}
