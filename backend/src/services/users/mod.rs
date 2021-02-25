/* external crates */

/* external uses */
use actix_web::{
    post,
    web::{
        Data,
        Json,
    },
    Responder,
    HttpResponse,
};

/* internal crates */
pub(in self) mod input {
    /* external crates */

    /* external uses */

    /* internal crates */
    pub(in super) mod user_info {
        /* external crates */

        /* external uses */
        use serde::Deserialize;

        /* internal crates */

        /* internal uses */

        #[derive(Deserialize)]
        pub struct UserInfo {
            pub(in super::super) username: String,
            pub(in super::super) passwd: String,
        }
    }

    /* internal uses */
}
pub(in self) mod output {
    /* external crates */

    /* external uses */

    /* internal crates */

    pub mod authorization_response {
        /* external crates */

        /* external uses */
        use serde::Serialize;
        use uuid::Uuid;

        /* internal crates */

        /* internal uses */
        use crate::models::queryable_user::QueryableUser;

        #[derive(Serialize)]
        pub struct AuthorizationResponse {
            authorization_key: Uuid,
            queryable_user: QueryableUser,
        }

        #[allow(unused)]
        impl AuthorizationResponse {
            pub fn new(queryable_user: QueryableUser) -> Self {
                return AuthorizationResponse {
                    authorization_key: Uuid::new_v4(),
                    queryable_user,
                };
            }
        }
    }

    /* internal uses */
}
pub mod id;

/* internal uses */
use crate::messages::create::Create;
use crate::app::app_state::AppState;
use input::user_info::UserInfo;
use output::authorization_response::AuthorizationResponse;

#[post("/users")]
pub async fn create_user(json: Json<UserInfo>, app_state: Data<AppState>) -> impl Responder {
    let user_info = json.into_inner();
    let create_message = Create::new(user_info.username, user_info.passwd);
    let db = &app_state.as_ref().db;

    match db.send(create_message).await {
        Ok(Ok(user)) => {
            return HttpResponse::Ok()
                .content_type("application/json")
                .json(AuthorizationResponse::new(user));
        },
        Ok(Err(err)) => {
            return HttpResponse::InternalServerError()
                .content_type("application/json")
                .json(format!("{}", err));
        },
        Err(err) => {
            return HttpResponse::InternalServerError()
                .content_type("application/json")
                .json(format!("{}", err));
        }
    };
}

// #[get("/users")]
// pub async fn get_user(Query(user_id): Query<UserId>, app_state: Data<AppState>) -> impl Responder {
//     let retrieve_message = Retrieve::new(user_id.id);
//     let db = &app_state.as_ref().db;

//     match db.send(retrieve_message).await {
//         Ok(Ok(user)) => { return HttpResponse::Ok().json(AuthorizationResponse::new(user)); },
//         Ok(Err(err)) => { return HttpResponse::NotFound().json(format!("{}", err)); },
//         Err(_) => { return HttpResponse::InternalServerError().json("Something went wrong"); }
//     }
// }

// #[get("/users/{id}")]
// pub async fn retrieve_user(Path(id): Path<Uuid>, app_state: Data<AppState>) -> impl Responder {

// }

// #[delete("/users")]
// pub async fn delete_user() -> impl Responder {

// }
