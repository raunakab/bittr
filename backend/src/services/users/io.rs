/* external crates */

/* external uses */

/* internal crates */

/* internal uses */

pub mod new_user {
    /* external crates */

    /* external uses */
    use serde::Deserialize;

    /* internal crates */

    /* internal uses */

    #[derive(Deserialize)]
    pub struct NewUser {
        pub(in super::super) username: String,
        pub(in super::super) passwd: String,
    }

    #[allow(unused)]
    impl NewUser {
        pub fn new(username: String, passwd: String) -> Self {
            return NewUser { username, passwd };
        }
    }
}

pub mod user_id {
    /* external crates */

    /* external uses */
    use serde::Deserialize;
    use uuid::Uuid;

    /* internal crates */

    /* internal uses */

    #[derive(Deserialize)]
    pub struct UserId {
        pub(in super::super) id: Uuid,
    }

    #[allow(unused)]
    impl UserId {
        pub fn new(id: Uuid) -> Self {
            return UserId { id, };
        }
    }
}

pub mod authorization_response {
    /* external crates */

    /* external uses */
    use serde::Serialize;
    use uuid::Uuid;

    /* internal crates */

    /* internal uses */
    use crate::models::queryable_user::QueryableUser;

    #[derive(Serialize)]
    pub struct AuthorizationResponse {
        authorization_key: Uuid,
        queryable_user: QueryableUser,
    }

    #[allow(unused)]
    impl AuthorizationResponse {
        pub fn new(queryable_user: QueryableUser) -> Self {
            return AuthorizationResponse {
                authorization_key: Uuid::new_v4(),
                queryable_user,
            };
        }
    }
}
