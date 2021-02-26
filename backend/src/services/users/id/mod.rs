/* external crates */

/* external uses */
use actix_web::{
    get,
    put,
    delete,
    web::{
        Path,
        Json,
        Data,
    },
    Responder,
    HttpResponse,
};

/* internal crates */
pub(in self) mod input {
    /* external crates */

    /* external uses */

    /* internal crates */
    pub(in super) mod username {
        /* external crates */

        /* external uses */
        use serde::Deserialize;

        /* internal crates */

        /* internal uses */

        #[derive(Clone, Deserialize)]
        pub struct Username {
            pub(in super::super) username: String,
        }
    }

    pub(in super) mod new_user_info {
        /* external crates */

        /* external uses */
        use serde::Deserialize;

        /* internal crates */

        /* internal uses */

        #[derive(Deserialize)]
        pub struct NewUserInfo {
            pub(in super::super) new_username: String,
            pub(in super::super) new_passwd: String,
        }
    }

    /* internal uses */
}
pub(in self) mod output {
    /* external crates */

    /* external uses */

    /* internal crates */

    /* internal uses */
}

/* internal uses */
use crate::messages::{
    retrieve::retrieve_with_username::RetrieveWithUsername,
    update::update_with_username::UpdateWithUsername,
    delete::delete_with_username::DeleteWithUsername,
};
use crate::app::app_state::AppState;
use input::{
    username::Username,
    new_user_info::NewUserInfo,
};

#[get("/users/{username}")]
pub async fn get_user(Path(username): Path<Username>, app_state: Data<AppState>) -> impl Responder {
    let retrieve_message = RetrieveWithUsername::new(username.username);
    let db = &app_state.as_ref().db;

    match db.send(retrieve_message).await {
        Ok(Ok(user)) => {
            return HttpResponse::Ok()
                .content_type("application/json")
                .json(user);
        },
        Ok(Err(err)) => {
            return HttpResponse::NotFound()
                .content_type("application/json")
                .json(format!("{}", err));
        },
        Err(err) => {
            return HttpResponse::InternalServerError()
                .content_type("application/json")
                .json(format!("{}", err));
        }
    }
}

#[put("/users/{username}")]
pub async fn put_user(Path(username): Path<Username>, json: Json<NewUserInfo>, app_state: Data<AppState>) -> impl Responder {
    let new_user_info = json.into_inner();
    let update_message = UpdateWithUsername::new(username.username, new_user_info.new_username, new_user_info.new_passwd);
    let db = &app_state.as_ref().db;

    match db.send(update_message).await {
        Ok(Ok(user)) => {
            return HttpResponse::Ok()
                .content_type("application/json")
                .json(user);
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
        },
    };
}

#[delete("/users/{username}")]
pub async fn delete_user(Path(username): Path<Username>, app_state: Data<AppState>) -> impl Responder {
    let delete_message = DeleteWithUsername::new(username.username);
    let db = &app_state.as_ref().db;

    match db.send(delete_message).await {
        Ok(Ok(user)) => {
            return HttpResponse::Ok()
                .content_type("application/json")
                .json(format!("deleted user: {}", user.username));
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
    }
}
