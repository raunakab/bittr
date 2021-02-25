/* external crates */

/* external uses */
use serde::{
    Serialize,
    Deserialize
};
use diesel::Insertable;
use uuid::Uuid;

/* internal crates */

/* internal uses */
use crate::schema::users;

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[table_name="users"]
pub struct InsertableUser {
    id: Uuid,
    username: String,
    passwd: String
}

impl InsertableUser {
    pub fn new(username: String, passwd: String) -> Self {
        return InsertableUser {
            id: Uuid::new_v4(),
            username,
            passwd,
        };
    }
}

