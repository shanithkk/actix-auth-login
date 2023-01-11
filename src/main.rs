#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
use std::sync::Arc;

use color_eyre::Result;
use actix_web::{HttpServer,App,middleware::{self, Logger}};
use tracing::info;

mod config;
use crate::config::{*, crypto::CryptoService};
mod handler;
use handler::*;
mod models;
use models::*;
mod schema;
mod db;
use db::*;
mod errors;
use actix_web::web::Data;

#[actix_web::main]
async fn main() -> Result<()> {
    
    let config = Config::from_env().expect("Server");
    let pool = config.db_pool().expect("Database Configuration");
    let crypto_service = config.crypto_service();
    info!("Server run at http://{}:{}", config.host, config.port);
    
    HttpServer::new(move || {
        
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(crypto_service.clone()))
            .configure(app_config)
    })
    .bind(format!("{}:{}",config.host,config.port))?
    .run()
    .await?;

    Ok(())
}
