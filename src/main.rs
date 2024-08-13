mod db;
mod errors;
mod models;
mod routes;
mod services;
mod config;
mod schema;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use config::Config;

#[actix_web::main]
async fn main() {
    let config = Config::from_env();
    let pool = db::establish_connection(&config.database_url);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(pool.clone())
            .wrap(cors)
            .configure(routes::init)

    })
    .bind(config.server_addr)
    .unwrap_or_else(|_| panic!("Can not bind server to address"))
    .run()
    .await
    .unwrap_or_else(|_| panic!("Server run failed"));
}