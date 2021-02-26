/* external crates */

/* external uses */
use actix::Message;
use diesel::prelude::*;

/* internal crates */

/* internal uses */
use crate::models::queryable_user::QueryableUser;

#[derive(Message)]
#[rtype(result="QueryResult<QueryableUser>")]
pub struct UpdateWithUsername {
    pub username: String,
    pub new_username: String,
    pub new_passwd: String,
}

#[allow(unused)]
impl UpdateWithUsername {
    pub fn new(username: String, new_username: String, new_passwd: String) -> Self {
        return UpdateWithUsername { username, new_username, new_passwd, };
    }
}
