use crate::config::IndexConfig;
use crate::db::DbPool;
use crate::models::current_collection_datas;
use crate::models::market_collections::query_collections;
use crate::service::Service;
use crate::worker::Worker;

use anyhow::Result;
use async_channel::Sender;
use async_trait::async_trait;
use log::{info, trace};
use tokio::runtime::Handle;
use tokio::task::JoinHandle;

#[derive(Debug, Clone)]
pub struct AptosService {
    //Config ref
    cfg: IndexConfig,
    // Database conn.
    indexer_db: DbPool,
    market_db: DbPool,
    tx: Sender<Worker>,
    // Dataquery function.
}

impl AptosService {
    pub fn new(
        cfg: IndexConfig,
        indexer_db: DbPool,
        market_db: DbPool,
        tx: Sender<Worker>,
    ) -> Self {
        // Create a new channel
        // Do query database in runtime. and privade Vec<NFT> to insert databasse work
        //let (tx,tx) = tokio::sync::mpsc::new();
        Self {
            cfg,
            indexer_db,
            market_db,
            tx,
        }
    }
}

#[async_trait]
impl Service for AptosService {
    async fn run(&self, runtime_handle: &Handle) -> JoinHandle<Result<()>> {
        let Self {
            cfg: _,
            indexer_db,
            market_db,
            tx,
        } = self.clone();

        runtime_handle.spawn(async move {
            let mut db = indexer_db
                .get()
                .expect("couldn't get indexer_db connection from pool");

            let mut mkdb = market_db
                .get()
                .expect("couldn't get market_db connect from pool:");

            // Fetch market db for last_version
            let mut version: i64 = query_collections(&mut mkdb).unwrap_or_default();

            loop {
                use tokio::time::Duration;
                trace!("start fetch nfts");

                trace!("Fetch bigger then {} version collections", version);

                // and fetch bigger then last_version colleact. and issert or repeact
                let collections =
                    current_collection_datas::query_bigger_then_version(&mut db, version)
                        .unwrap_or_default();

                trace!("The new batch is {} length", collections.len());
                let mut max_version = version;

                for collection in collections {
                    if collection.last_transaction_version > max_version {
                        max_version = collection.last_transaction_version;
                    }

                    tx.send(Worker::from(collection))
                        .await
                        .expect("Send to Worker channel failed.");
                }

                version = max_version + 1;
                trace!("end fetch nfts");
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        })
    }
}
