#![allow(unused)]

use actix_web::{
    get,
    web,
    App,
    HttpServer,
    Responder,
    web::Query,
};
use actix_cors::Cors;
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct UPPair {
    username: String,
    password: String,
}

#[get("/")]
async fn index(web::Query(info): web::Query<UPPair>) -> impl Responder {
    return format!("Hello! {}, {}", info.username, info.password);
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
