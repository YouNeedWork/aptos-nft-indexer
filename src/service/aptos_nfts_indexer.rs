use crate::config::IndexConfig;
use crate::db::DbPool;
use crate::models::{current_token_datas, tokens};

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
            cfg,
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

            let mut version: i64 = tokens::query_max_token_version(&mut mkdb).unwrap_or_default();
            let batch = 10000;

            loop {
                use tokio::time::Duration;
                // Fetch market db for last_version
                info!("Fetch bigger then {} version tokens", version);

                // and fetch bigger then last_version colleact. and issert or repeact
                let tokens =
                    current_token_datas::query_bigger_then_version(&mut db, version, batch)
                        .unwrap_or_default();

                if tokens.len() == 0 {
                    let t = current_token_datas::query_bigger_then_version(
                        &mut db,
                        version,
                        batch + 10000000000000,
                    )
                    .unwrap_or_default();

                    if t.len() != 0 {
                        version += batch;
                        continue;
                    }

                    tokio::time::sleep(Duration::from_millis(1000)).await;
                    continue;
                }

                info!("The new token batch is {} length", tokens.len());

                for token in tokens {
                    if token.last_transaction_version > version {
                        version = token.last_transaction_version;
                    }

                    tx.send(Worker::from(token))
                        .await
                        .expect("Send to Worker channel failed.");
                }

                version += 1;

                trace!("end fetch nfts");
                tokio::time::sleep(Duration::from_millis(cfg.fetch_millis as u64)).await;
            }
        })
    }
}
