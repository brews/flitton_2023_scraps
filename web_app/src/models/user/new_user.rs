use bcrypt::{hash, DEFAULT_COST};
use diesel::Insertable;
use uuid::Uuid;

use crate::schema::users;

/// NewUser going into backend database. Password is hashed.
#[derive(Insertable, Clone)]
#[diesel(table_name=users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub unique_id: String,
}

impl NewUser {
    /// Create NewUser with hashed password.
    pub fn new(username: String, email: String, password: String) -> NewUser {
        let hashed_password: String = hash(password.as_str(), DEFAULT_COST).unwrap();
        let uuid = Uuid::new_v4().to_string();
        return NewUser {
            username,
            email,
            password: hashed_password,
            unique_id: uuid,
        };
    }
}
