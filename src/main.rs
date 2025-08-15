use actix_web::{web, App, HttpServer};
//use dotenvy::dotenv;
use std::env;
use dotenvy::from_filename;

mod db;
mod models;
mod handlers;
mod routes;

use db::init_db;
use routes::init_routes;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    
    // .env
    //dotenv().ok();
    from_filename(".env.dev").ok();

    // database
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = init_db(&database_url).await.expect("Failed to connect to DB");

    // http
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .configure(init_routes)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
