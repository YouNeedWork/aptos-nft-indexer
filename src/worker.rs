use tokio::{task::JoinHandle,runtime::Handle,sync::mpsc::Receiver};
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



pub struct WorkerService {
    rx:Receiver<Worker>
}

impl WorkerService {
    pub fn new( rx:Receiver<Worker>) -> Self {
        //Self { rx }
	Self{rx}
    }
}

#[async_trait]
impl WorkerTrait for WorkerService {
    async fn run(&self,runtime_handle: &Handle) -> JoinHandle<Result<()>> {
	
	//let rx = self.rx;
	runtime_handle.spawn(async move {
	    loop {
		tokio::select!{
		    work = self.rx.recv() => {
			dbg!(&work);
		    }
		}
	    }
	})
    }
}
