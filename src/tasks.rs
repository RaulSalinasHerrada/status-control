use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};

use crate::model::Status;
use crate::model::TaskDb;
use crate::model::TaskPost;

#[get("/")]
pub async fn get_tasks(tasks: Data<TaskDb>) -> impl Responder {
    let db = tasks.lock().unwrap();
    HttpResponse::Ok().json(&*db)
}

#[get("/{task_hash}")]
pub async fn get_task_status(task_hash: Path<String>, tasks: Data<TaskDb>) -> impl Responder {
    let task_db = tasks.lock().unwrap();
    let task_hash = task_hash.into_inner();

    let status = match task_db.get(&task_hash) {
        Some(x) => *x,
        None => Status::NonStarted,
    };

    HttpResponse::Ok().json(status)
}

#[post("/add/")]
pub async fn add_task_status(task: Json<TaskPost>, tasks: Data<TaskDb>) -> impl Responder {
    let mut task_db = tasks.lock().unwrap();
    let task = task.into_inner();

    let task_hash = task.task_hash;

    let status = match task.status {
        Some(x) => x,
        None => Status::Ok,
    };

    task_db.insert(task_hash.clone(), status);

    HttpResponse::Created()
}
