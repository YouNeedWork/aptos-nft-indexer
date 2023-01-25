use async_trait::async_trait;
use tokio::task::JoinHandle;
use tokio::runtime::Handle;
use crate::service::Service;
use anyhow::Result;



#[derive(Debug, Clone)]
pub struct AptosNFTService {
    // Database conn.
    // Dataquery function.
}

impl AptosNFTService {
    pub fn new() -> Self {
	// Create a new channel
	// Do query database in runtime. and privade Vec<NFT> to insert databasse work
	
	//let (tx,tx) = tokio::sync::mpsc::new();
	
        Self {}
    }
}

#[async_trait]
impl Service for AptosNFTService {
    async fn run(&self, runtime_handle: &Handle) -> JoinHandle<Result<()>> {
        runtime_handle.spawn(async move {
            loop {
                use tokio::time::Duration;
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        })
    }
}
