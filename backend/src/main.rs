/* external crates */
extern crate actix;
#[macro_use]
extern crate diesel;
#[allow(unused)]
#[macro_use]
extern crate diesel_migrations;

/* external uses */
use std::env;
use actix::SyncArbiter;
use actix_web::{
    HttpServer,
    App,
};
use actix_cors::Cors;
use dotenv::dotenv;
use diesel::{
    r2d2::{
        self,
        ConnectionManager,
    },
    prelude::*,
};

/* internal crates */
mod actor;
mod app;
mod messages;
mod models;
mod services;
mod schema;

/* internal uses */
use crate::actor::db_actor::DbActor;
use crate::app::app_state::AppState;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

// fn in_users(uppair: UPPair) -> bool {
//     let user1: UPPair = UPPair {
//         username: String::from("pbhagat"),
//         passwd: String::from("prasad2205"),
//     };
//     let users: Vec<UPPair> = vec![user1];

//     for pair in users.iter() {
//         if pair.username == uppair.username && pair.passwd == uppair.passwd {
//             return true;
//         }
//     }
//     return false;
// }

// #[get("/users")]
// async fn index(web::Query(info): web::Query<UPPair>) -> impl Responder {
//     // if in_users(info) {
//     //     return HttpResponse::Ok()
//     //         .content_type("application/json")
//     //         .json(AuthKey::new());
//     // } else {
//     //     return HttpResponse::Unauthorized()
//     //         .content_type("application/json")
//     //         .json("something went wrong");
//     // }
//     return HttpResponse::Ok()
//             .content_type("application/json")
//             .json(AuthKey::new());
// }

// #[derive(Deserialize)]
// pub struct Test {
//     username: String,
//     passwd: String,
// }

// #[put("/users")]
// async fn index(test: Json<Test>) -> impl Responder {
//     // if in_users(info) {
//     //     return HttpResponse::Ok()
//     //         .content_type("application/json")
//     //         .json(AuthKey::new());
//     // } else {
//     //     return HttpResponse::Unauthorized()
//     //         .content_type("application/json")
//     //         .json("something went wrong");
//     // }
//     // return HttpResponse::Ok()
//     //         .content_type("application/json")
//     //         .json(AuthKey::new());
//     return HttpResponse::Ok()
//         .content_type("application/json")
//         .json(format!("test-stuff-stuff: {}, {}", test.username, test.passwd));
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let db_addr = SyncArbiter::start(5, move || DbActor::new(pool.clone()));

    return HttpServer::new(move || {
        return App::new()
            .wrap(Cors::permissive())
            .service(services::users::create_user)
            .service(services::users::get_user)
            .data(AppState::new(db_addr.clone()));
    })
        .bind("127.0.0.1:5000")?
        .run()
        .await;
}
