#![allow(unused)]

use actix_web::{
    get,
    web,
    App,
    HttpServer,
    Responder,
    web::Query,
    HttpResponse,
};
use actix_cors::Cors;
use serde::{
    Serialize,
    Deserialize,
};
use uuid::Uuid;

#[derive(Serialize)]
pub struct AuthKey {
    authkey: Uuid,
}
impl AuthKey {
    pub fn new() -> Self {
        return AuthKey {
            authkey: Uuid::new_v4(),
        };
    }
}

#[derive(Deserialize)]
pub struct UPPair {
    username: String,
    password: String,
}


fn in_users(uppair: UPPair) -> bool {
    let user1: UPPair = UPPair {
        username: String::from("pbhagat"),
        password: String::from("prasad2205"),
    };
    let users: Vec<UPPair> = vec![user1];

    for pair in users.iter() {
        if pair.username == uppair.username && pair.password == uppair.password {
            return true;
        }
    }
    return false;
}

#[get("/users")]
async fn index(web::Query(info): web::Query<UPPair>) -> impl Responder {
    if in_users(info) {
        return HttpResponse::Ok()
            .content_type("application/json")
            .json(AuthKey::new());
    } else {
        return HttpResponse::Unauthorized()
            .content_type("application/json")
            .json("something went wrong");
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    return HttpServer::new(|| {
        return App::new()
            .wrap(Cors::permissive())
            .service(index);
    })
        .bind("127.0.0.1:5000")?
        .run()
        .await;
}
