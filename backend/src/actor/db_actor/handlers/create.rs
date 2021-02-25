/* external crates */

/* external uses */
use actix::Handler;
use diesel::prelude::*;
// use uuid::Uuid;

/* internal crates */

/* internal uses */
use super::super::DbActor;
use crate::models::{
    queryable_user::QueryableUser,
    insertable_user::InsertableUser,
};
use crate::schema::users;
use crate::messages::create::Create;

impl Handler<Create> for DbActor {
    type Result = QueryResult<QueryableUser>;

    fn handle(&mut self, msg: Create, _: &mut Self::Context) -> Self::Result {
        let conn = self.get_connection();

        let new_user = InsertableUser::new(msg.first_name, msg.last_name);

        return diesel::insert_into(users::dsl::users)
            .values(new_user)
            .get_result::<QueryableUser>(&conn);
    }
}
