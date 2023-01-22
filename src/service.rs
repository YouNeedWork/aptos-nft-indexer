use anyhow::Result;
use tokio::{runtime::Handle, task::JoinHandle};

use crate::config::IndexConfig;

#[derive(Debug, Clone)]
pub struct Service {}

impl Service {
    pub async fn run(&self, runtime_handle: &Handle) -> Result<()> {
        runtime_handle.spawn(async move {});
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct IndexerService<'a> {
    cfg: IndexConfig<'a>,
    servers: Vec<Service>,
}

impl<'a> IndexerService<'a> {
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
            .build()
            .unwrap();

        //rt.block_on(async move { self.start_indexer().await });
        for service in self.servers {
            service.run(rt.handle());
        }

        log::info!("IndexService Ended");
        Ok(())
    }

    async fn start_indexer(&self) -> Result<()> {
        // start all indexer include fetch nft collection,metadata,owner,rolay,creater address, more table.
        // let servers = self.servers.clone();

        // let services_joinhandle = servers
        //     .iter()
        //     .map(|e| tokio::spawn(async move { e.run().await }))
        //     .collect::<Vec<JoinHandle<Result<()>>>>();

        Ok(())
    }
}
