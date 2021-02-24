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

impl CreateMessage {
    pub fn new(first_name: String, last_name: String) -> Self {
        return CreateMessage { first_name, last_name, };
    }
}

#[derive(Message)]
#[rtype(result="QueryResult<User>")]
pub struct RetrieveMessage {
    pub id: Uuid,
}

impl RetrieveMessage {
    pub fn new(id: Uuid) -> Self {
        return RetrieveMessage { id, };
    }
}

#[derive(Message)]
#[rtype(result="QueryResult<User>")]
pub struct UpdateMessage {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
}

impl UpdateMessage {
    pub fn new(id: Uuid, first_name: String, last_name: String) -> Self {
        return UpdateMessage { id, first_name, last_name, };
    }
}

#[derive(Message)]
#[rtype(result="QueryResult<User>")]
pub struct DeleteMessage {
    pub id: Uuid,
}

impl DeleteMessage {
    pub fn new(id: Uuid) -> Self {
        return DeleteMessage { id, };
    }
}
