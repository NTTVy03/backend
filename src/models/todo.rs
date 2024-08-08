use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct TodoItem {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
    pub create_at: DateTime<Utc>,
}