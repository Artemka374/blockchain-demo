extern crate core;

use actix_web::web::Data;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;

pub mod backend;

use backend::{get, post, test};
pub mod crypto;
pub mod db;
pub mod models;

pub struct NodeData {
    pub pool: PgPool,
    pub config: models::config::Config,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();

    let pool = db::init().await.expect("Failed to connect to database");
    let config = models::config::Config::parse();
    let server_url = config.server_url.clone();

    HttpServer::new(move || {
        let app = App::new().app_data(Data::new(NodeData {
            pool: pool.clone(),
            config: config.clone(),
        }));

        // route GET methods
        let app = app
            .service(get::get_balance)
            .service(get::get_transaction)
            .service(get::get_transactions)
            .service(get::get_block_by_hash)
            .service(get::get_block_by_id)
            .service(get::get_proof)
            .service(get::get_nonce)
            .service(get::get_target)
            .service(get::block_height);

        // route POST methods
        let app = app
            .service(post::transfer)
            .service(post::try_mine)
            .service(post::set_target)
            .service(post::mint);

        // route TEST methods
        let app = app
            .service(test::get_mode)
            .service(test::set_mode)
            .service(test::generate_sig)
            .service(test::verify_sig)
            .service(test::get_pub_key)
            .service(test::verify_proof)
            .service(test::hash_message);
        app
    })
    .bind(server_url)?
    .run()
    .await
}
