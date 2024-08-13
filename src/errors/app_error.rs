// use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AppError {
    pub message: String,
}

// impl ResponseError for AppError {
//     fn error_response(&self) -> HttpResponse {
//         HttpResponse::InternalServerError().json(self)
//     }
// }