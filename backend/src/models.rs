// external uses
use actix::Addr;
use serde::{
    Serialize,
    Deserialize
};
use diesel::{
    Queryable,
    Insertable
};
use uuid::Uuid;

// internal uses
use crate::schema::users;
// use crate::actors::db::DbActor;

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub passwd: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[table_name="users"]
pub struct NewUser {
    id: Uuid,
    username: String,
    passwd: String
}

impl NewUser {
    pub fn new(username: String, passwd: String) -> Self {
        return NewUser {
            id: Uuid::new_v4(),
            username,
            passwd,
        };
    }
}
