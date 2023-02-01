use anyhow::Result;
use async_trait::async_trait;
use async_channel::Receiver;
use log::{trace,info};
use tokio::{runtime::Handle, task::JoinHandle};


use crate::models::current_collection_datas::CurrentCollectionDataQuery;
use crate::models::market_collections::insert_collection;
use crate::models::market_collections::CollectionInsert;


use crate::db::DbPool;

#[async_trait]
pub trait WorkerTrait {
    async fn run(&mut self, runtime_handle: &Handle) -> JoinHandle<Result<()>>;
}

#[derive(Debug)]
pub enum Worker {
    COLLECTION(CurrentCollectionDataQuery),
    NEW_NFTS_OR_OWNER_CHANGED, //TODO all type here are holder DB origin type.like(DB)
}

impl From<CurrentCollectionDataQuery> for Worker {
    fn from(value: CurrentCollectionDataQuery) -> Self {
        Self::COLLECTION(value)
    }
}

#[derive(Clone,Debug)]
pub struct WorkerService {
    rx: Receiver<Worker>,
    db:DbPool,
}

impl WorkerService {
    pub fn new(rx: Receiver<Worker>,db:DbPool) -> Self {
        Self { rx ,db}
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
