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
    let (tx,rx) = tokio::sync::mpsc::channel::<Worker>(1000);
    //add service
    let nft_collect = AptosNFTService::new(cfg.clone(), indexer_db,market_db,tx.clone());
    let worker = WorkerService::new();
    //let handle = worker.run(service.runtime(),rx);
    service.add_worker(Box::new(worker));
    //Add worker and started
    //service.add_worker(worker.subscript());
    service.add_server(Box::new(nft_collect));
    service.run()
}
