/* external crates */

/* external uses */
use actix::Handler;
use diesel::prelude::*;
// use uuid::Uuid;

/* internal crates */

/* internal uses */
use super::super::super::DbActor;
use crate::models::queryable_user::QueryableUser;
use crate::schema::users;
use crate::messages::retrieve::retrieve_with_username::RetrieveWithUsername;

impl Handler<RetrieveWithUsername> for DbActor {
    type Result = QueryResult<QueryableUser>;

    fn handle(&mut self, msg: RetrieveWithUsername, _: &mut Self::Context) -> Self::Result {
        let conn = self.get_connection();

        match users::dsl::users.filter(users::dsl::username.eq(msg.username)).load::<QueryableUser>(&conn) {
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
