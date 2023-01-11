pub(crate) mod crypto;
use crypto::*;
use std::{sync::Arc, time::Duration};

use color_eyre::Result;
use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};
use dotenv::dotenv;
use eyre::WrapErr;
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use tracing_subscriber::EnvFilter;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub host: String,
    pub port: i32,
    pub database_url: String,
    pub secret_key: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Result<Config> {
        dotenv().ok();

        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();

        info!("Loading Configuration");
        let mut c = config::Config::new();
        c.merge(config::Environment::default())?;

        c.try_into()
            .context("Loading configuration from environmet")
    }

    // pub fn db_pool(&self) -> Result<Pool>{
    //     let manager = ConnectionManager::<PgConnection>::new(self.database_url.clone());
    //     Ok(Pool::new(manager).expect("Failed to create db pool"))

    // }
    pub fn db_pool(&self) -> Result<Pool> {
        info!("Creating database connection pool.");
        let manager = ConnectionManager::<PgConnection>::new(self.database_url.clone());
        Pool::builder()
            .connection_timeout(Duration::from_secs(30))
            .build(manager)
            .context("created database connection pool")
    }

    pub fn crypto_service(&self) -> CryptoService {
        CryptoService {
            key: Arc::new(self.secret_key.clone()),
            jwt_secret: Arc::new(self.jwt_secret.clone()),
        }
    }

    // pub fn connection(&self) -> DbConnection {
    //     POOL.get().expect("Failed to connect db")
    // }
    // pub fn init(&self) {
    //     lazy_static::initialize(&POOL);
    //     let conn = self.connection();
    //     embedded_migrations::run(&conn).unwrap();
    // }
}

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

// embed_migrations!();
// use lazy_static::lazy_static;
// use std::env;
// pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

// lazy_static! {
//     static ref POOL: Pool = {
//         let db_url = env::var("DATABASE_URL").expect("Database url not set");
//         let manager = ConnectionManager::<PgConnection>::new(db_url);
//         Pool::new(manager).expect("Failed to create db pool")
//     };
// }
