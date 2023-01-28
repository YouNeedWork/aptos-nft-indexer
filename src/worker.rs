use tokio::task::JoinHandle;

pub enum Worker {
    NFTS,
    METADATAS,
    OWNER,
}

impl Worker {
    pub fn new() -> Self {
        Self::NFTS
    }
}

pub struct WorkerService {
    workers: Vec<JoinHandle<Worker>>,
}

impl WorkerService {
    pub fn new() -> Self {
        Self { workers: vec![] }
    }
}
