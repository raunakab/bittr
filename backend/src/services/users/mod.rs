/* external crates */

/* external uses */
use actix_web::{
    get,
    put,
    web::{
        Data,
        Json,
        Query,
    },
    Responder,
    HttpResponse,
};

/* internal crates */
mod io;

/* internal uses */
use crate::messages::{
    create::Create,
    retrieve::Retrieve,
};
use crate::app::app_state::AppState;
use crate::services::users::io::{
    new_user::NewUser,
    user_id::UserId,
    authorization_response::AuthorizationResponse,
};

#[put("/users")]
pub async fn create_user(json: Json<NewUser>, app_state: Data<AppState>) -> impl Responder {
    let new_user = json.into_inner();
    let create_message = Create::new(new_user.username, new_user.passwd);
    let db = &app_state.as_ref().db;

    match db.send(create_message).await {
        Ok(Ok(user)) => { return HttpResponse::Ok().json(user); },
        Ok(Err(err)) => { return HttpResponse::NotFound().json(format!("{}", err)); },
        Err(_) => { return HttpResponse::InternalServerError().json("Something went wrong"); }
    };
}

#[get("/users")]
pub async fn get_user(Query(user_id): Query<UserId>, app_state: Data<AppState>) -> impl Responder {
    let retrieve_message = Retrieve::new(user_id.id);
    let db = &app_state.as_ref().db;

    match db.send(retrieve_message).await {
        Ok(Ok(user)) => { return HttpResponse::Ok().json(AuthorizationResponse::new(user)); },
        Ok(Err(err)) => { return HttpResponse::NotFound().json(format!("{}", err)); },
        Err(_) => { return HttpResponse::InternalServerError().json("Something went wrong"); }
    }
}

// #[get("/users/{id}")]
// pub async fn retrieve_user(Path(id): Path<Uuid>, app_state: Data<AppState>) -> impl Responder {

// }

// #[delete("/users")]
// pub async fn delete_user() -> impl Responder {

// }
