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
    pub mode: models::api::NodeMode,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = db::init().await.unwrap();

    let app = App::new().app_data(NodeData {
        pool: pool.clone(),
        mode: Default::default(),
    });

    // route GET methods
    let app = app
        .service(get::get_balance)
        .service(get::get_transaction)
        .service(get::get_transactions)
        .service(get::get_block_by_hash)
        .service(get::get_block_by_id)
        .service(get::get_proof)
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
        .service(test::verify_proof);

    HttpServer::new(move || app)
        .bind(env::var("SERVER_URL"))?
        .run()
        .await
}