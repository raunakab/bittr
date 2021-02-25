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
pub struct Delete {
    pub id: Uuid,
}

#[allow(unused)]
impl Delete {
    pub fn new(id: Uuid) -> Self {
        return Delete { id, };
    }
}
