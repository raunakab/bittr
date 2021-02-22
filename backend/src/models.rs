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
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String
}
