use anyhow::Result;

use cargos_indexer::service::{aptos_indexer::AptosNFTService, IndexerService};
use cargos_indexer::{config, db,worker::Worker,worker::WorkerService};

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    let cfg = config::IndexConfig::new()?;
    
    run_all(cfg)?;
    Ok(())
}

fn run_all(cfg: config::IndexConfig) -> Result<()> {
    //init service
    let mut service: IndexerService = IndexerService::new(cfg.clone());

    //init db
    let indexer_db = db::get_connection_pool(&cfg.indexer_db_posgres);
    let market_db = db::get_connection_pool(&cfg.market_posgres);
    
    //init the nft_collection sender
    let (tx,rx) = async_channel::unbounded::<Worker>();
    
    //add service
    let nft_collect = AptosNFTService::new(cfg.clone(), indexer_db,market_db,tx.clone());
    
    let worker = WorkerService::new(rx);
    //let handle = worker.run(service.runtime(),rx);
    
    //Add worker and started    
    for _ in 0 ..cfg.work_number {
	service.add_worker(Box::new(worker.clone()));
    }

    //service.add_worker(worker.subscript());
    service.add_server(Box::new(nft_collect));
    service.run()
}
