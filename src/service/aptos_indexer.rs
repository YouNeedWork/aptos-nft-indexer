use async_trait::async_trait;


#[derive(Debug, Clone)]
pub struct AptosNFTService {
    // Database conn.
    // Dataquery function.
}

#[async_trait]
impl Service for AptosNFTService {
    pub async fn run(&self, runtime_handle: &Handle) -> JoinHandle<Result<()>> {
        runtime_handle.spawn(async move {
	    loop{
		println!("Spawn: Hello");
		use tokio::time::Duration;
		tokio::time::sleep(Duration::from_millis(100)).await;
	    }
	})
    }
}
