use crate::config::IndexConfig;
use crate::db::DbPool;
use crate::service::Service;

use anyhow::Result;
use async_trait::async_trait;
use log::info;
use tokio::runtime::Handle;
use tokio::task::JoinHandle;

#[derive(Debug, Clone)]
pub struct AptosNFTService {
    //Config ref
    cfg: IndexConfig,
    // Database conn.
    indexer_db: DbPool,
    // Dataquery function.
}

impl AptosNFTService {
    pub fn new(cfg: IndexConfig, indexer_db: DbPool) -> Self {
        // Create a new channel
        // Do query database in runtime. and privade Vec<NFT> to insert databasse work
        //let (tx,tx) = tokio::sync::mpsc::new();
        Self { cfg, indexer_db }
    }
}

#[async_trait]
impl Service for AptosNFTService {
    async fn run(&self, runtime_handle: &Handle) -> JoinHandle<Result<()>> {
        let Self { cfg, indexer_db } = self.clone();

        runtime_handle.spawn(async move {
            loop {
                use tokio::time::Duration;
                info!("start fetch nfts");
                //fetch market db for last_version

                //and fetch bigger then last_version colleact. and issert or repeact

                info!("end fetch nfts");
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        })
    }
}
