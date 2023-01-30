use tokio::{task::JoinHandle,runtime::Handle};
use async_trait::async_trait;
use anyhow::Result;

use crate::models::current_collection_datas::CurrentCollectionDataQuery;


#[async_trait]
pub trait WorkerTrait {
    async fn run(&self, runtime_handle: &Handle) -> JoinHandle<Result<()>>;
}


#[derive(Debug)]
pub enum Worker {
    COLLECTION(CurrentCollectionDataQuery),
    NFTS,//TODO all type here are holder DB origin type.like(DB)
    OWNER,
}

impl From<CurrentCollectionDataQuery>  for Worker{
    fn from(value: CurrentCollectionDataQuery) -> Self {
	Self::COLLECTION(value)
    }
}

impl Worker {
    pub fn new() -> Self {
        Self::NFTS
    }
}


// pub struct WorkerService {
//     workers: Vec<JoinHandle<Worker>>,
// }
// impl WorkerService {
//     pub fn new() -> Self {
//         Self { workers: vec![] }
//     }
// }
