use anyhow::Result;
use async_channel::Receiver;
use async_trait::async_trait;
use log::{info, trace};
use tokio::{runtime::Handle, task::JoinHandle};

use crate::models::current_collection_datas::CurrentCollectionDataQuery;
use crate::models::current_token_datas::CurrentTokenData;
use crate::models::market_collections::{insert_collection, CollectionInsert};
use crate::models::tokens::{insert_token, TokenInsert};

use crate::db::DbPool;

#[async_trait]
pub trait WorkerTrait {
    async fn run(&mut self, runtime_handle: &Handle) -> JoinHandle<Result<()>>;
}

#[derive(Debug)]
pub enum Worker {
    COLLECTION(CurrentCollectionDataQuery),
    NEW_NFTS_OR_OWNER_CHANGED(CurrentTokenData), //TODO all type here are holder DB origin type.like(DB)
}

impl From<CurrentCollectionDataQuery> for Worker {
    fn from(value: CurrentCollectionDataQuery) -> Self {
        Self::COLLECTION(value)
    }
}

impl From<CurrentTokenData> for Worker {
    fn from(v: CurrentTokenData) -> Self {
        Self::NEW_NFTS_OR_OWNER_CHANGED(v)
    }
}

#[derive(Clone, Debug)]
pub struct WorkerService {
    rx: Receiver<Worker>,
    db: DbPool,
}

impl WorkerService {
    pub fn new(rx: Receiver<Worker>, db: DbPool) -> Self {
        Self { rx, db }
    }
}

#[async_trait]
impl WorkerTrait for WorkerService {
    async fn run(&mut self, runtime_handle: &Handle) -> JoinHandle<Result<()>> {
        let rx = self.rx.clone();
        let mkdb = self.db.clone();

        runtime_handle.spawn(async move {
	    let mut db = mkdb
                    .get()
                .expect("couldn't get indexer_db connection from pool");
            loop {
                tokio::select! {
                    new_worker = rx.recv() => {
			match new_worker {
			    Ok(Worker::COLLECTION(c)) => {
				let id = c.collection_data_id_hash.clone();
				trace!("Got new collection_id {}",id);
				// Insert to
				insert_collection(&mut db,CollectionInsert::from(c)).expect("Fail to insert db");
				trace!("finesh the collection_id {}",id);
			    }
			    Ok(Worker::NEW_NFTS_OR_OWNER_CHANGED(nft)) => {
				let id = nft.token_data_id_hash.clone();
				trace!("Got new nft id:{}",id);
				// Insert to
				insert_token(&mut db,TokenInsert::from(nft)).expect("Fail to insert db");
				trace!("finesh the nft id {}",id);				
			    }
			    _=> {
				unreachable!();
			    }
			}
                    },
                }
            }
        })
    }
}
