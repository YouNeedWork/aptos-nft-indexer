use anyhow::Result;

use cargos_indexer::service::{aptos_collections_indexer, aptos_nfts_indexer, IndexerService};
use cargos_indexer::{aws, config, db, worker::Worker, worker::WorkerService};

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    let cfg = config::IndexConfig::new()?;

    run_all(cfg)?;
    Ok(())
}

fn run_all(cfg: config::IndexConfig) -> Result<()> {
    // init service
    let mut service: IndexerService = IndexerService::new(cfg.clone());

    // init db
    let indexer_db = db::get_connection_pool(&cfg.indexer_db_posgres);
    let market_db = db::get_connection_pool(&cfg.market_posgres);

    // init the nft_collection sender
    let (tx, rx) = async_channel::unbounded::<Worker>();

    //add service
    let collection = aptos_collections_indexer::AptosService::new(
        cfg.clone(),
        indexer_db.clone(),
        market_db.clone(),
        tx.clone(),
    );

    let nft = aptos_nfts_indexer::AptosService::new(
        cfg.clone(),
        indexer_db.clone(),
        market_db.clone(),
        tx.clone(),
    );

    let aws_cfg = cfg.clone();

    //init aws s3
    let client = service
        .runtime()
        .block_on(async move { aws::get_client(&aws_cfg, "ap-northeast-1").await.unwrap() });

    let worker = WorkerService::new(rx, market_db, indexer_db, client);

    //let handle = worker.run(service.runtime(),rx);
    //Add worker and started
    for _ in 0..cfg.work_number {
        service.add_worker(Box::new(worker.clone()));
    }

    service.add_server(Box::new(collection));
    service.add_server(Box::new(nft));
    service.run()
}
