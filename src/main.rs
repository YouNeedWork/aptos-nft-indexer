use anyhow::Result;

mod config;
mod db;
mod models;
mod schema;
mod service;
mod worker;

use service::aptos_indexer::AptosNFTService;
use service::IndexerService;

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    let cfg = config::IndexConfig::new()?;

    run_all(cfg)?;
    Ok(())
}

fn run_all(cfg: config::IndexConfig) -> Result<()> {
    let mut service: IndexerService = IndexerService::new(cfg.clone());

    let indexer_db = db::get_connection_pool(&cfg.indexer_db_posgres);

    let nft = AptosNFTService::new(cfg.clone(), indexer_db);

    service.add_server(Box::new(nft));
    service.run()
}
