use anyhow::Result;

mod config;

fn main() -> Result<()> {
    let cfg = config::IndexConfig::new()?;

    println!("{:?}", cfg);

    Ok(())
}
