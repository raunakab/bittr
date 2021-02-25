/* external crates */

/* external uses */
use actix::Message;
use diesel::prelude::*;
use uuid::Uuid;

/* internal crates */

/* internal uses */
use crate::models::queryable_user::QueryableUser;

#[derive(Message)]
#[rtype(result="QueryResult<QueryableUser>")]
pub struct Update {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
}

#[allow(unused)]
impl Update {
    pub fn new(id: Uuid, first_name: String, last_name: String) -> Self {
        return Update { id, first_name, last_name, };
    }
}
