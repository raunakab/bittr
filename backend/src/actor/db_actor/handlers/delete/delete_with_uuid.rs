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
use crate::messages::delete::delete_with_uuid::DeleteWithUuid;

impl Handler<DeleteWithUuid> for DbActor {
    type Result = QueryResult<QueryableUser>;

    fn handle(&mut self, msg: DeleteWithUuid, _: &mut Self::Context) -> Self::Result {
        let conn = self.get_connection();

        return diesel::delete(users::dsl::users)
            .filter(users::dsl::id.eq(msg.id))
            .get_result::<QueryableUser>(&conn);
    }
}
