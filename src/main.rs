mod error_passing;
mod model;
mod security;
mod tables;
mod tasks;

use actix_web::http::StatusCode;
use actix_web::middleware::{ErrorHandlers, Logger};
use actix_web::web::{Data, ServiceConfig};
use actix_web::{get, web::scope, HttpResponse, Responder};
use actix_web_httpauth::middleware::HttpAuthentication;
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::SecretStore;
use std::{collections::HashMap, sync::Mutex};

use error_passing::*;
use model::*;
use security::*;
use tables::*;
use tasks::*;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world from status!")
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let tables: Data<TableDb> = Data::new(Mutex::new(HashMap::new()));
    let tasks: Data<TaskDb> = Data::new(Mutex::new(HashMap::new()));

    secrets.into_iter().for_each(|(key, val)| {
        std::env::set_var(key, val);
    });

    let auth_middleware = HttpAuthentication::with_fn(validator);

    // set up our Actix web service and wrap it with logger and add the AppState as app data
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            scope("/v1")
                .wrap(Logger::default())
                .wrap(
                    ErrorHandlers::new()
                        .handler(StatusCode::INTERNAL_SERVER_ERROR, add_error_header),
                )
                .service(hello)
                .service(basic_auth)
                .service(
                    scope("/task")
                        .wrap(auth_middleware.clone())
                        .service(get_tasks)
                        .service(get_task_status)
                        .service(add_task_status),
                )
                .service(
                    scope("/table")
                        .wrap(auth_middleware.clone())
                        .service(get_tables)
                        .service(get_table_status)
                        .service(add_table_status),
                )
                .app_data(tables.clone())
                .app_data(tasks.clone())
                ,
        );
    };

    Ok(config.into())
}
