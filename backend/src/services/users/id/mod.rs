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
    pub(in super) mod id {
        /* external crates */

        /* external uses */
        use serde::Deserialize;
        use uuid::Uuid;

        /* internal crates */

        /* internal uses */

        #[derive(Deserialize)]
        pub struct Id {
            pub(in super::super) id: Uuid,
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
    retrieve::Retrieve,
    update::Update,
    delete::Delete,
};
use crate::app::app_state::AppState;
use input::{
    id::Id,
    new_user_info::NewUserInfo,
};

#[get("/users/{id}")]
pub async fn get_user(Path(user_id): Path<Id>, app_state: Data<AppState>) -> impl Responder {
    let retrieve_message = Retrieve::new(user_id.id);
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

#[put("/users/{id}")]
pub async fn put_user(Path(id): Path<Id>, json: Json<NewUserInfo>, app_state: Data<AppState>) -> impl Responder {
    let new_user_info = json.into_inner();
    let update_message = Update::new(id.id, new_user_info.new_username, new_user_info.new_passwd);
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

#[delete("/users/{id}")]
pub async fn delete_user(Path(id): Path<Id>, app_state: Data<AppState>) -> impl Responder {
    let delete_message = Delete::new(id.id);
    let db = &app_state.as_ref().db;

    match db.send(delete_message).await {
        Ok(Ok(_)) => {
            return HttpResponse::Ok()
                .content_type("application/json")
                .json(format!("deleted user: {}", id.id));
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
