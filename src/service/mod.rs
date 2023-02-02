use anyhow::Result;
use async_trait::async_trait;
use tokio::{
    runtime::{Builder, Handle, Runtime},
    task::JoinHandle,
};

use crate::config::IndexConfig;
use crate::worker::WorkerTrait;

pub mod aptos_collections_indexer;
pub mod aptos_nfts_indexer;

#[async_trait]
pub trait Service {
    async fn run(&self, runtime_handle: &Handle) -> JoinHandle<Result<()>>;
}

pub struct IndexerService {
    cfg: IndexConfig,
    rt: Runtime,
    servers: Vec<Box<dyn Service>>,
    workers: Vec<Box<dyn WorkerTrait>>,
}

impl IndexerService {
    pub fn new(cfg: IndexConfig) -> Self {
        log::info!("IndexService init");

        let rt = Builder::new_multi_thread()
            .worker_threads(cfg.work_number as usize)
            .thread_name("Tokio-Runtime")
            .thread_stack_size(3 * 1024 * 1024)
            .enable_time()
            .enable_io()
            .build()
            .unwrap();

        Self {
            cfg,
            rt,
            servers: vec![],
            workers: vec![],
        }
    }

    pub fn run(&mut self) -> Result<()> {
        log::info!("IndexService Runing");

        let services = self
            .servers
            .iter()
            .map(|service| service.run(self.rt.handle()))
            .collect::<Vec<_>>();

        // let workers = self
        //     .workers
        //     .iter()
        //     .map(|worker| worker.run(self.rt.handle()))
        //     .collect::<Vec<_>>();

        let mut workers = vec![];
        for worker in &mut self.workers {
            workers.push(worker.run(self.rt.handle()));
        }

        self.rt.block_on(async move {
            for s in services {
                s.await;
            }

            for worker in workers {
                worker.await;
            }

            loop {}
        });

        log::info!("IndexService Ended");
        Ok(())
    }

    pub fn add_server(&mut self, s: Box<dyn Service>) {
        self.servers.push(s)
    }

    pub fn add_worker(&mut self, s: Box<dyn WorkerTrait>) {
        self.workers.push(s)
    }

    pub fn runtime(&self) -> &Handle {
        self.rt.handle()
    }
}
