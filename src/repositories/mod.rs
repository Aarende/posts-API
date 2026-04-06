use std::time::Duration;
use diesel_async::{
    AsyncPgConnection,
    pooled_connection::{
        AsyncDieselConnectionManager,
        deadpool::{Object, Pool}
    }
};
use crate::constants::{MAX_DBPOOL_SIZE, DBPOOL_TIMEOUT_IN_SECS};

pub mod users_repositories;
pub mod posts_repositories;

pub type PgConnection = Object<AsyncPgConnection>;
pub type PgPool = Pool<AsyncPgConnection>;

pub fn create_pool(database_url: &str) -> PgPool {
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);

    Pool::builder(manager)
        .max_size(MAX_DBPOOL_SIZE)
        .wait_timeout(Some(Duration::from_secs(DBPOOL_TIMEOUT_IN_SECS)))
        .runtime(deadpool::Runtime::Tokio1)
        .build()
        .expect("Failed to create pool of connections for database")
}
