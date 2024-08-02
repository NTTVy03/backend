use actix_web::{web, get, post, App, HttpResponse, HttpServer, Responder};

struct AppState {
    app_name: String,
}

#[get("/")]
async fn hello(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    // HttpResponse::Ok().body("Hello {}!", app_name)
    format!("Hello {}!", app_name)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/hey")]
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(
                AppState{
                    app_name: String::from("ZyZy web")
                }
            ))
            .service(hello)
            .service(echo)
            .service(manual_hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}