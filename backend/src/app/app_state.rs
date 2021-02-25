/* external crates */

/* external uses */
use actix::Addr;

/* internal crates */

/* internal uses */
use crate::actor::db_actor::DbActor;

pub struct AppState {
    pub db: Addr<DbActor>
}

impl AppState {
    pub fn new(db: Addr<DbActor>) -> Self {
        return AppState { db, };
    }
}
