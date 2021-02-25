/* external crates */

/* external uses */
use serde::{
    Deserialize,
};
use uuid::Uuid;

/* internal crates */

/* internal uses */

#[derive(Deserialize)]
pub struct NewUser {
    pub(super) username: String,
    pub(super) passwd: String,
}

#[derive(Deserialize)]
pub struct UserId {
    pub(super) id: Uuid,
}
