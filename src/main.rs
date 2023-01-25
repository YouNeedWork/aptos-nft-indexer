use anyhow::Result;

mod config;
mod service;
mod worker;

use service::IndexerService;


fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    let cfg = config::IndexConfig::new()?;

    run_all(cfg)?;
    Ok(())
}

fn run_all(cfg: config::IndexConfig) -> Result<()> {
    let mut service:IndexerService = IndexerService::new(cfg);
    service.run()
}

#[test]
fn test_run_main() {
    let cfg = config::IndexConfig::default();
    todo!();
}
