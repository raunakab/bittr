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
use crate::db_actor::DbActor;

pub struct AppState {
    pub db: Addr<DbActor>
}

impl AppState {
    pub fn new(db: Addr<DbActor>) -> Self {
        return AppState { db, };
    }
}
