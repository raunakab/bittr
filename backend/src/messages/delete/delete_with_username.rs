/* external crates */

/* external uses */
use actix::Message;
use diesel::prelude::*;

/* internal crates */

/* internal uses */
use crate::models::queryable_user::QueryableUser;

#[derive(Message)]
#[rtype(result="QueryResult<QueryableUser>")]
pub struct DeleteWithUsername {
    pub username: String,
}

#[allow(unused)]
impl DeleteWithUsername {
    pub fn new(username: String) -> Self {
        return DeleteWithUsername { username, };
    }
}
