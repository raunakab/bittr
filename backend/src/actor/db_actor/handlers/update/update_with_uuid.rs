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
use crate::messages::update::update_with_uuid::UpdateWithUuid;

impl Handler<UpdateWithUuid> for DbActor {
    type Result = QueryResult<QueryableUser>;

    fn handle(&mut self, msg: UpdateWithUuid, _: &mut Self::Context) -> Self::Result {
        let conn = self.get_connection();

        return diesel::update(users::dsl::users)
            .filter(users::dsl::id.eq(msg.id))
            .set((
                users::dsl::username.eq(msg.new_username),
                users::dsl::passwd.eq(msg.new_passwd),
            ))
            .get_result::<QueryableUser>(&conn);
    }
}
