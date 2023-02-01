use diesel::prelude::*;

use diesel::r2d2::{ConnectionManager, Pool};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
//pub type DB = PooledConnection<ConnectionManager<PgConnection>>;

pub fn get_connection_pool(url: &str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(url);

    Pool::builder()
        .max_size(20)
        .test_on_check_out(true)
        //.connection_customizer(Box::new())
        .build(manager)
        .expect("Could not build connection pool")
}
