mod todo;

use actix_web::{web, App, HttpServer};
use std::sync::Mutex;
use actix_cors::Cors;

use crate::models::TodoItem;

pub struct AppState {
    todo_list: Mutex<Vec<TodoItem>>,
}

pub async fn start() {
    let app_state = web::Data::new(AppState {
        todo_list: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        let cors = Cors::default().
            allow_any_origin().
            allow_any_method().
            allow_any_header().
            max_age(3600);

        App::new()
            .app_data(app_state.clone())
            .wrap(cors)
            .configure(routes)

    })
    .bind("127.0.0.1:8080")
    .unwrap_or_else(|_| panic!("Can not bind server to address"))
    .run()
    .await
    .unwrap_or_else(|_| panic!("Server run failed"));
}

fn routes(app: &mut web::ServiceConfig) {
    app
        .route("/todos", web::get().to(todo::get_todos))
        .route("/todos", web::post().to(todo::add_todo))
        .route("/todos/{id}", web::put().to(todo::update_todo))
        .route("/todos/{id}", web::delete().to(todo::delete_todo));
}