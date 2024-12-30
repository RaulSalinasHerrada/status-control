use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};

use crate::model::Status;
use crate::model::TableDb;
use crate::model::TablePost;

#[get("/")]
pub async fn get_tables(tables: Data<TableDb>) -> impl Responder {
    let table_db = tables.lock().unwrap();
    HttpResponse::Ok().json(&*table_db)
}

#[get("/{table_name}")]
pub async fn get_table_status(table: Path<String>, tables: Data<TableDb>) -> impl Responder {
    let table_db = tables.lock().unwrap();
    let table = table.into_inner();

    let status = match table_db.get(&table) {
        Some(x) => *x,
        None => Status::NonStarted,
    };

    HttpResponse::Ok().json(status)
}

#[post("/add/")]
pub async fn add_table_status(table: Json<TablePost>, tables: Data<TableDb>) -> impl Responder {
    let mut table_db = tables.lock().unwrap();

    let table = table.into_inner();

    let name = table.table;

    let status = match table.status {
        Some(x) => x,
        None => Status::Ok,
    };

    table_db.insert(name.clone(), status);
    HttpResponse::Created()
}
