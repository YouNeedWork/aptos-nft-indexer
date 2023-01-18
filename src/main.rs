use anyhow::Result;

mod config;
mod service;
mod worker;

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    let cfg = config::IndexConfig::new()?;

    service::IndexerService::new(cfg).run()
}

#[test]
fn test_run_main() {
    let cfg = config::IndexConfig::default();
    todo!();
}
