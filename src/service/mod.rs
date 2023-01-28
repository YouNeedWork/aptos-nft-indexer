use anyhow::Result;
use async_trait::async_trait;
use tokio::{runtime::Handle, task::JoinHandle};

use crate::config::IndexConfig;

pub mod aptos_indexer;

#[async_trait]
pub trait Service {
    async fn run(&self, runtime_handle: &Handle) -> JoinHandle<Result<()>>;
}

pub struct IndexerService {
    cfg: IndexConfig,
    servers: Vec<Box<dyn Service>>,
}

impl IndexerService {
    pub fn new(cfg: IndexConfig) -> Self {
        log::info!("IndexService init");

        Self {
            cfg,
            servers: vec![],
        }
    }

    pub fn run(&self) -> Result<()> {
        log::info!("IndexService Runing");
        use tokio::runtime::Builder;

        let rt = Builder::new_multi_thread()
            .worker_threads(self.cfg.work_number as usize)
            .thread_name("Tokio-Runtime")
            .thread_stack_size(3 * 1024 * 1024)
            .enable_time()
            .build()
            .unwrap();

        let services = self
            .servers
            .iter()
            .map(|service| service.run(rt.handle()))
            .collect::<Vec<_>>();

        rt.block_on(async move {
            for s in services {
                s.await;
            }

            loop {}
        });

        log::info!("IndexService Ended");
        Ok(())
    }

    pub fn add_server(&mut self, s: Box<dyn Service>) {
        self.servers.push(s)
    }
}
