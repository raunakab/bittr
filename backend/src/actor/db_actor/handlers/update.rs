/* external crates */

/* external uses */
use actix::Handler;
use diesel::prelude::*;
// use uuid::Uuid;

/* internal crates */

/* internal uses */
use super::super::DbActor;
use crate::models::queryable_user::QueryableUser;
use crate::schema::users;
use crate::messages::update::Update;

impl Handler<Update> for DbActor {
    type Result = QueryResult<QueryableUser>;

    fn handle(&mut self, msg: Update, _: &mut Self::Context) -> Self::Result {
        let conn = self.get_connection();

        return diesel::update(users::dsl::users)
            .filter(users::dsl::id.eq(msg.id))
            .set((
                users::dsl::username.eq(msg.first_name),
                users::dsl::passwd.eq(msg.last_name),
            ))
            .get_result::<QueryableUser>(&conn);
    }
}
