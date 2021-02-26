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
pub struct UpdateWithUuid {
    pub id: Uuid,
    pub new_username: String,
    pub new_passwd: String,
}

#[allow(unused)]
impl UpdateWithUuid {
    pub fn new(id: Uuid, new_username: String, new_passwd: String) -> Self {
        return UpdateWithUuid { id, new_username, new_passwd, };
    }
}
