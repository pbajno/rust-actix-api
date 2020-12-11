use crate::models::Status;
use crate::queries;
use deadpool_postgres::{Pool, Client};
use actix_web::{web, Responder, HttpResponse};

pub async fn slash_status() -> impl Responder {
    web::HttpResponse::Ok()
        .json(Status { status : "OK".to_string()})
}

pub async fn slash_todos(db_pool: web::Data<Pool>) -> impl Responder {
    
    let client: Client = db_pool.get().await.expect("Error connecting to the database");

    let result = queries::get_todos(&client).await;
    
    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().into()
    }

}

pub async fn slash_items(db_pool: web::Data<Pool>, path: web::Path<(i32,)>) -> impl Responder {
    
    let client: Client = db_pool.get().await.expect("Error connecting to the database");

    let result = queries::get_items(&client, path.0).await;
    
    match result {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().into()
    }

}