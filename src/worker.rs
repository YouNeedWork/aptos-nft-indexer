use anyhow::Result;
use async_trait::async_trait;
use async_channel::Receiver;
use log::{trace,info};
use tokio::{runtime::Handle, task::JoinHandle};

use crate::models::current_collection_datas::CurrentCollectionDataQuery;

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
}

impl WorkerService {
    pub fn new(rx: Receiver<Worker>) -> Self {
        Self { rx }
    }
}

#[async_trait]
impl WorkerTrait for WorkerService {
    async fn run(&mut self, runtime_handle: &Handle) -> JoinHandle<Result<()>> {
        let rx = self.rx.clone();
        runtime_handle.spawn(async move {
            loop {
                tokio::select! {
                    new_worker = rx.recv() => {
			match new_worker {
			    Ok(Worker::COLLECTION(c)) => {
				trace!("Got new collection_id {} ",c.collection_data_id_hash);
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
