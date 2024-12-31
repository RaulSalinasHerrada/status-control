use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum Status {
    Failure,
    NonStarted,
    Progress,
    Ok,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TablePost {
    pub table: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TaskPost {
    pub task: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

pub type TableDb = Mutex<HashMap<String, Status>>;
pub type TaskDb = Mutex<HashMap<String, Status>>;
