use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use actix_web::{
    get, post,
    web::{scope, Data, Json, Path},
    HttpResponse, Responder,
};

use serde::{Deserialize, Serialize};

use actix_web::middleware::Logger;

use actix_web::web::ServiceConfig;
use shuttle_actix_web::ShuttleActixWeb;

#[derive(Serialize, Deserialize, Clone, Copy)]
enum Status {
    Failure,
    NonStarted,
    Progress,
    Ok,
}

#[derive(Serialize, Deserialize, Clone)]
struct TablePost {
    table_name: String,
    status: Option<Status>,
}

#[derive(Serialize, Deserialize, Clone)]
struct TaskPost {
    task_hash: String,
    status: Option<Status>,
}

type TableDb = Arc<Mutex<HashMap<String, Status>>>;
type TaskDb = Arc<Mutex<HashMap<String, Status>>>;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world from status!")
}

#[get("/tables")]
async fn get_tables(tables: Data<TableDb>) -> impl Responder {
    let table_db = tables.lock().unwrap();
    HttpResponse::Ok().json(&*table_db)
}

#[get("/tasks")]
async fn get_tasks(tasks: Data<TaskDb>) -> impl Responder {
    let db = tasks.lock().unwrap();
    HttpResponse::Ok().json(&*db)
}

#[get("/task/{task_hash}")]
async fn get_task_status(task_hash: Path<String>, tasks: Data<TaskDb>) -> impl Responder {
    let task_db = tasks.lock().unwrap();
    let task_hash = task_hash.into_inner();

    let status = match task_db.get(&task_hash) {
        Some(x) => *x,
        None => Status::NonStarted,
    };

    HttpResponse::Ok().json(status)
}

#[get("/table/{table_name}")]
async fn get_table_status(table_name: Path<String>, tables: Data<TableDb>) -> impl Responder {
    let table_db = tables.lock().unwrap();
    let table_name = table_name.into_inner();

    let status = match table_db.get(&table_name) {
        Some(x) => *x,
        None => Status::NonStarted,
    };

    HttpResponse::Ok().json(status)
}

#[post("/table/add/")]
async fn add_table_status(table: Json<TablePost>, tables: Data<TableDb>) -> impl Responder {
    let mut table_db = tables.lock().unwrap();

    let table = table.into_inner();

    let table_name = table.table_name;

    let status = match table.status {
        Some(x) => x,
        None => Status::Ok,
    };

    table_db.insert(table_name.clone(), status);
    HttpResponse::Created()
}

#[post("/task/add/")]
async fn add_task_status(task: Json<TaskPost>, tasks: Data<TaskDb>) -> impl Responder {
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

// #[actix_web::main]
#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let tables: TableDb = Arc::new(Mutex::new(HashMap::new()));
    let tasks: TaskDb = Arc::new(Mutex::new(HashMap::new()));

    // set up our Actix web service and wrap it with logger and add the AppState as app data
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            scope("/v1")
                .wrap(Logger::default())
                .service(hello)
                .service(get_tasks)
                .service(get_tables)
                .service(get_table_status)
                .service(get_task_status)
                .service(add_table_status)
                .service(add_task_status)
                .app_data(Data::new(tables.clone()))
                .app_data(Data::new(tasks.clone())),
        );
    };

    Ok(config.into())
}