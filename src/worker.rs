use tokio::{task::JoinHandle,runtime::Handle,sync::mpsc::Receiver};
use async_trait::async_trait;
use anyhow::Result;
use log::info;


use crate::models::current_collection_datas::CurrentCollectionDataQuery;


#[async_trait]
pub trait WorkerTrait {
    async fn run(&mut self,runtime_handle: &Handle)-> JoinHandle<Result<()>>;
}

#[derive(Debug)]
pub enum Worker {
    COLLECTION(CurrentCollectionDataQuery),
    NEW_NFTS_OR_OWNER_CHANGED,//TODO all type here are holder DB origin type.like(DB)
}

impl From<CurrentCollectionDataQuery>  for Worker{
    fn from(value: CurrentCollectionDataQuery) -> Self {
	Self::COLLECTION(value)
    }
}


pub struct WorkerService {
    rx:Option<Receiver<Worker>>
}

impl WorkerService {
    pub fn new( rx:Option<Receiver<Worker>>) -> Self {
	Self{rx}
    }
}

#[async_trait]
impl WorkerTrait for WorkerService {
    async fn run(&mut self,runtime_handle: &Handle) -> JoinHandle<Result<()>> {
	let rx = self.rx.take();
	
	match rx {
	    Some(mut rx) => {
		runtime_handle.spawn(async move {
		    loop {
			tokio::select!{
			    Some(work) = rx.recv() => {
				match work {
				    Worker::COLLECTION(collection) => {
					info!("Got new Collection.");
					dbg!(&collection);
				    }
				    _=> {
					dbg!(&work);
				    }
				}
			    }
			}
		    }
		})
	    },
	    None => {
		unreachable!("");
	    }
	}
    }
}
