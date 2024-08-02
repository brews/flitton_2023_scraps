extern crate bcrypt;

use bcrypt::verify;
use diesel::{Identifiable, Queryable};

use crate::schema::users;

/// User read from the backend database.
#[derive(Queryable, Clone, Identifiable)]
#[diesel(table_name=users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub unique_id: String,
}

impl User {
    pub fn verify(&self, password: String) -> bool {
        verify(password.as_str(), &self.password).unwrap()
    }
}
