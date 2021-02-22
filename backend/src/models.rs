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
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[table_name="users"]
pub struct NewUser {
    id: Uuid,
    first_name: String,
    last_name: String
}

impl NewUser {
    pub fn new(first_name: String, last_name: String) -> Self {
        return NewUser {
            id: Uuid::new_v4(),
            first_name: first_name,
            last_name: last_name,
        };
    }
}
