/* external crates */

/* external uses */
use actix::Message;
use diesel::prelude::*;

/* internal crates */

/* internal uses */
use crate::models::queryable_user::QueryableUser;

#[derive(Message)]
#[rtype(result="QueryResult<QueryableUser>")]
pub struct Create {
    pub first_name: String,
    pub last_name: String,
}

impl Create {
    pub fn new(first_name: String, last_name: String) -> Self {
        return Create { first_name, last_name, };
    }
}
