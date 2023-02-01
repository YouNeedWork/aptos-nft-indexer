pub mod config;
pub mod db;
pub mod models;
pub mod schema;
pub mod service;
pub mod worker;

use num_enum::IntoPrimitive;

#[derive(Debug,IntoPrimitive)]
#[repr(u8)]
pub enum ChainID {
    Aptos,
}

