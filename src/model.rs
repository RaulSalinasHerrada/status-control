use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum Status {
    Failure,
    NonStarted,
    Progress,
    Ok,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TablePost {
    pub table_name: String,
    pub status: Option<Status>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TaskPost {
    pub task_hash: String,
    pub status: Option<Status>,
}

pub type TableDb = Arc<Mutex<HashMap<String, Status>>>;
pub type TaskDb = Arc<Mutex<HashMap<String, Status>>>;
