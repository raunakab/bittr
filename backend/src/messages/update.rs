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
    pub username: String,
    pub passwd: String,
}

#[allow(unused)]
impl Update {
    pub fn new(id: Uuid, username: String, passwd: String) -> Self {
        return Update { id, username, passwd, };
    }
}
