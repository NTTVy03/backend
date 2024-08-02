use actix_web::{web, get, App, HttpServer};
use std::sync::Mutex;

struct AppState {
    counter: Mutex<i32>,
}

#[get("/")]
async fn count(data: web::Data<AppState>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("Number {}!", counter)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(
        AppState{
            counter: Mutex::new(0)
        }
    );

    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .service(count)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}