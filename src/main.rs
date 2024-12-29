mod model;
mod security;
mod tables;
mod tasks;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use actix_web::web::Data;
use actix_web::{get, web::scope, HttpResponse, Responder};

use actix_web::middleware::Logger;

use actix_web::web::ServiceConfig;
use shuttle_actix_web::ShuttleActixWeb;

use actix_web_httpauth::{extractors::bearer::BearerAuth, middleware::HttpAuthentication};

use model::*;
use security::validator;
use tables::*;
use tasks::*;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world from status!")
}

// #[actix_web::main]
#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let tables: TableDb = Arc::new(Mutex::new(HashMap::new()));
    let tasks: TaskDb = Arc::new(Mutex::new(HashMap::new()));

    let auth = HttpAuthentication::with_fn(validator);

    // set up our Actix web service and wrap it with logger and add the AppState as app data
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            scope("/v1")
                .wrap(auth)
                .wrap(Logger::default())
                .service(hello)
                .service(
                    scope("/task")
                        .service(get_tasks)
                        .service(get_task_status)
                        .service(add_task_status),
                )
                .service(
                    scope("/table")
                        .service(get_tables)
                        .service(get_table_status)
                        .service(add_table_status),
                )
                .app_data(Data::new(tables.clone()))
                .app_data(Data::new(tasks.clone())),
        );
    };

    Ok(config.into())
}
