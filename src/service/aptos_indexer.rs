use crate::config::IndexConfig;
use crate::db::DbPool;
use crate::models::current_collection_datas;
use crate::service::Service;
use crate::worker::Worker;



use anyhow::Result;
use async_trait::async_trait;
use log::info;
use tokio::runtime::Handle;
use tokio::task::JoinHandle;
use tokio::sync::mpsc::Sender;



#[derive(Debug, Clone)]
pub struct AptosNFTService {
    //Config ref
    cfg: IndexConfig,
    // Database conn.
    indexer_db: DbPool,
    market_db:DbPool,
    tx:Sender<Worker>,
    // Dataquery function.
}

impl AptosNFTService {
    pub fn new(cfg: IndexConfig, indexer_db: DbPool,market_db:DbPool,tx:Sender<Worker>) -> Self {
        // Create a new channel
        // Do query database in runtime. and privade Vec<NFT> to insert databasse work
        //let (tx,tx) = tokio::sync::mpsc::new();
        Self { cfg, indexer_db, market_db,tx}
    }
}

#[async_trait]
impl Service for AptosNFTService {
    async fn run(&self, runtime_handle: &Handle) -> JoinHandle<Result<()>> {
        let Self { cfg, indexer_db,market_db,tx } = self.clone();
        runtime_handle.spawn(async move {
            loop {
                use tokio::time::Duration;
                info!("start fetch nfts");
		
                let mut db = indexer_db
                    .get()
                    .expect("couldn't get indexer_db connection from pool");
		//let mut mkdb= market_db.get().expect("couldn't get market_db connect from pool:");
		
                //fetch market db for last_version
		
                //and fetch bigger then last_version colleact. and issert or repeact
                let collections =
                    current_collection_datas::query_bigger_then_version(db, 0).unwrap();
                for collection in collections {
		    tx.send(Worker::from(collection)).await.unwrap();//expect("Send to Worker channel failed.");
		    //sender to channel.
                    //dbg!(&collection);
                }
		
                info!("end fetch nfts");
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        })
    }
}
