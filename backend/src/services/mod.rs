/* external crates */

/* external uses */
use actix_web::{
    get,
    Responder,
    HttpResponse,
};

/* internal crates */
pub mod users;

/* internal uses */

#[get("/")]
pub async fn index() -> impl Responder {
    return HttpResponse::Ok()
        .content_type("application/json")
        .json("Hello, from Luo");
}
