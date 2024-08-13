mod todos;

use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route("/todos", web::get().to(todos::get_todos))
        .route("/todos", web::post().to(todos::add_todo))
        .route("/todos/{id}", web::put().to(todos::update_todo))
        .route("/todos/{id}", web::delete().to(todos::delete_todo));
}
