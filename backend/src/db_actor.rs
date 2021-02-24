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
        PooledConnection,
        ConnectionManager,
        Pool,
        Error,
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
use crate::messages::*;

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

impl Handler<CreateMessage> for DbActor {
    type Result = QueryResult<User>;

    fn handle(&mut self, msg: CreateMessage, _: &mut Self::Context) -> Self::Result {
        let conn = self.get_connection();

        let new_user = NewUser::new(msg.first_name, msg.last_name);

        return diesel::insert_into(users::dsl::users)
            .values(new_user)
            .get_result::<User>(&conn);
    }
}

impl Handler<RetrieveMessage> for DbActor {
    type Result = QueryResult<User>;

    fn handle(&mut self, msg: RetrieveMessage, _: &mut Self::Context) -> Self::Result {
        let conn = self.get_connection();

        match users::dsl::users.filter(users::dsl::id.eq(msg.id)).load::<User>(&conn) {
            Ok(vec) => {
                if vec.len() > 1 { panic!(); }

                match vec.get(0) {
                    Some(user) => { return Ok(user.clone()); },
                    None => { return Err(diesel::result::Error::NotFound); },
                }
            },
            Err(err) => { return Err(err); },
        };
    }
}

impl Handler<UpdateMessage> for DbActor {
    type Result = QueryResult<User>;

    fn handle(&mut self, msg: UpdateMessage, _: &mut Self::Context) -> Self::Result {
        let conn = self.get_connection();

        return diesel::update(users::dsl::users)
            .filter(users::dsl::id.eq(msg.id))
            .set((
                users::dsl::username.eq(msg.first_name),
                users::dsl::passwd.eq(msg.last_name),
            ))
            .get_result::<User>(&conn);
    }
}

impl Handler<DeleteMessage> for DbActor {
    type Result = QueryResult<User>;

    fn handle(&mut self, msg: DeleteMessage, _: &mut Self::Context) -> Self::Result {
        let conn = self.get_connection();

        return diesel::delete(users::dsl::users)
            .filter(users::dsl::id.eq(msg.id))
            .get_result::<User>(&conn);
    }
}
