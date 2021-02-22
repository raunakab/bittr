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
// use r2d2::Error;

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

        // match users::dsl::users.filter(users::dsl::id.eq(msg.id)).load::<User>(&conn) {
        //     Ok(vec) => {
        //         if vec.len() == 1usize {
        //             return vec.get(0);
        //         }
        //     },
        //     Err(err) => {},
        // };

        // todo!();

        match users::dsl::users.filter(users::dsl::id.eq(msg.id)).load::<User>(&conn) {
            Ok(vec) => {},
            Err(err) => {},
        };

        todo!();
    }
}

impl Handler<UpdateMessage> for DbActor {
    type Result = QueryResult<User>;

    fn handle(&mut self, msg: UpdateMessage, _: &mut Self::Context) -> Self::Result {
        let conn = self.get_connection();

        return diesel::update(users::dsl::users)
            .filter(users::dsl::id.eq(msg.id))
            .set((
                users::dsl::first_name.eq(msg.first_name),
                users::dsl::last_name.eq(msg.last_name),
            ))
            .get_result::<User>(&conn);
    }
}
