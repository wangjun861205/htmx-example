use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub(crate) struct Todo {
    pub(crate) id: i64,
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) start_time: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct TodoCreate {
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) start_time: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct TodoUpdate {
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) start_time: String,
}
