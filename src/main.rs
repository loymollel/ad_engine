extern crate diesel;
extern crate dotenv;

use actix_web::{middleware, web, App, HttpServer};
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager}
};
use std::{env};

mod models;
mod services;
mod api;
mod config;
mod schema;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let app_host = env::var("APP_HOST").expect("APP_HOST not found.");
    let app_port = env::var("APP_PORT").expect("APP_PORT not found.");
    let app_url = format!("{}:{}", &app_host, &app_port);
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found.");

    // Create connection pool
    let manager = ConnectionManager::<PgConnection>::new(&db_url);

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create a pool.");

    // Start HTTP Server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .configure(config::app::config_services)
    })
        .bind(&app_url)?
        .run()
        .await
}
