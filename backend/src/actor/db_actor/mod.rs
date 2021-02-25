/* external crates */

/* external uses */
use actix::{
    Actor,
    SyncContext,
};
use diesel::{
    r2d2::{
        PooledConnection,
        ConnectionManager,
        Pool,
    },
    PgConnection,
};

/* internal crates */
pub mod handlers;

/* internal uses */

pub struct DbActor {
    conn: Pool<ConnectionManager<PgConnection>>,
}

impl DbActor {
    pub fn new(conn: Pool<ConnectionManager<PgConnection>>) -> Self {
        return DbActor { conn, };
    }

    pub fn get_connection(self: &mut Self) -> PooledConnection<ConnectionManager<PgConnection>> {
        return self.conn
            .get()
            .expect("Connection to database failed");
    }
}

impl Actor for DbActor {
    type Context = SyncContext<Self>;
}
