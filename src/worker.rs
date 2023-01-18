use tokio::task::JoinHandle;

pub enum Worker {
    Default,
}

impl Worker {
    pub fn new() -> Self {
        Self::Default
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

#[test]
fn new_worker_service() {
    todo!();
}
