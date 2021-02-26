/* external crates */

/* external uses */
use actix::Message;
use diesel::prelude::*;

/* internal crates */

/* internal uses */
use crate::models::queryable_user::QueryableUser;

#[derive(Message)]
#[rtype(result="QueryResult<QueryableUser>")]
pub struct RetrieveWithUsername {
    pub username: String,
}

impl RetrieveWithUsername {
    pub fn new(username: String) -> Self {
        return RetrieveWithUsername { username, };
    }
}
