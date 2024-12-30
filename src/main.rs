mod model;
mod security;
mod tables;
mod tasks;

use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::web::Path;
use actix_web::web::ServiceConfig;
use actix_web::{get, web::scope, HttpResponse, Responder};
use actix_web_httpauth::middleware::HttpAuthentication;
use ferris_says::say;
use model::*;
use security::*;
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::SecretStore;
use std::io::BufWriter;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tables::*;
use tasks::*;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world from status!")
}

#[get("/join/{name}")]
async fn join(name: Path<String>) -> impl Responder {
    let out = format!("Join us {name}!");
    let width = 256;

    let mut buf = BufWriter::new(Vec::new());
    say(out.as_str(), width, &mut buf).unwrap();

    let bytes = buf.into_inner().unwrap();
    let msg = String::from_utf8(bytes).unwrap();

    HttpResponse::Ok().body(msg)
}

// #[actix_web::main]
#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let tables: TableDb = Arc::new(Mutex::new(HashMap::new()));
    let tasks: TaskDb = Arc::new(Mutex::new(HashMap::new()));

    secrets.into_iter().for_each(|(key, val)| {
        std::env::set_var(key, val);
    });

    let auth_middleware = HttpAuthentication::with_fn(validator);

    // set up our Actix web service and wrap it with logger and add the AppState as app data
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            scope("/v1")
                .wrap(Logger::default())
                .service(hello)
                .service(join)
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
                .app_data(Data::new(tables.clone()))
                .app_data(Data::new(tasks.clone())),
        );
    };

    Ok(config.into())
}
