mod app;
mod models;

#[actix_web::main]
async fn main() {
    app::start().await;
}