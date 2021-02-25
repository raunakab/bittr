/* external crates */

/* external uses */
use serde::{
    Serialize,
    Deserialize
};
use diesel::Queryable;
use uuid::Uuid;

/* internal crates */

/* internal uses */

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct QueryableUser {
    pub id: Uuid,
    pub username: String,
    pub passwd: String,
}
