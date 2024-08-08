use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use diesel::prelude::*;

#[derive(Serialize, Deserialize, Clone)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::todo_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TodoItem {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
    pub create_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct CreateTodoItem {
    pub title: String,
    pub completed: bool,
}

#[derive(Deserialize)]
pub struct UpdateTodoItem {
    pub title: Option<String>,
    pub completed: Option<bool>,
}
