use crate::db::DbPool;
use crate::errors::app_error::AppError;
use crate::models::todo::TodoItem;
use crate::schema::todo_items::dsl::*;

use actix_web::web;
use diesel::{prelude::*, result};

pub async fn get_all_todos(
    pool: web::Data<DbPool>,
) -> Result<Vec<TodoItem>, AppError> {
    // let results = web::block(move || {
    //     let connection = pool.get().unwrap();
    //     todo_items.load::<TodoItem>(&connection)
    // })
    // .await?
    // .map_err(|e| {
    //     AppError {
    //         message: "Database error".to_string()
    //     }
    // });

    let results = vec![]; // load list of TodoItem from database
    
    Ok(results)
}
