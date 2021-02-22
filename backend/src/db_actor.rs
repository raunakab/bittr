// external uses
use actix::{
    Actor,
    Handler,
    Message,
    SyncContext,
};
use diesel::{
    prelude::*,
    r2d2::{
        ConnectionManager,
        Pool
    },
    PgConnection,
};
use uuid::Uuid;

// internal uses
use crate::models::{
    User,
    NewUser,
};
use crate::schema::users;

#[derive(Message)]
#[rtype(result="QueryResult<User>")]
pub struct CreateMessage {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Message)]
#[rtype(result="QueryResult<User>")]
pub struct RetrieveMessage {
    pub id: Uuid,
}

#[derive(Message)]
#[rtype(result="QueryResult<User>")]
pub struct UpdateMessage {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Message)]
#[rtype(result="QueryResult<User>")]
pub struct DeleteMessage {
    pub id: Uuid,
}
