// external uses
use actix_web::{
    get,
    put,
    delete,
    web::{
        self,
        Data,
        Json,
        Path,
        Query,
    },
    App,
    HttpServer,
    Responder,
    HttpResponse,
};
use actix_cors::Cors;
use serde::{
    Serialize,
    Deserialize,
};
use uuid::Uuid;

// internal uses
use crate::UPPair;
use crate::db_actor;
use crate::messages::*;
use crate::app_state::AppState;

#[put("/users")]
pub async fn create_user(app_state: Data<AppState>) -> impl Responder {
    // let create_message = CreateMessage::new(info.username, info.passwd);
    // let db = &app_state.as_ref().db;

    // match db.send(create_message).await {
    //     Ok(Ok(user)) => { return HttpResponse::Ok().json(user); },
    //     Ok(Err(err)) => { return HttpResponse::NotFound().json("Unable to create user"); },
    //     Err(_) => { return HttpResponse::InternalServerError().json("Something went wrong"); }
    // };
    // return HttpResponse::Ok()
    //         .content_type("application/json")
    //         .json(format!("Hello, {} {}", info.username, info.passwd));
    return HttpResponse::Ok();
}

// #[get("/users/{id}")]
// pub async fn retrieve_user(Path(id): Path<Uuid>, app_state: Data<AppState>) -> impl Responder {

// }

// #[delete("/users")]
// pub async fn delete_user() -> impl Responder {

// }
