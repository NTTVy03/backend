mod app;
mod models;
mod schema;
mod utils;
mod db;
mod error;

#[actix_web::main]
async fn main() {
    app::start().await;
}