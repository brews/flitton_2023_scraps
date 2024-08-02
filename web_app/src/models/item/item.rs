use crate::models::user::user::User;
use crate::schema::to_do;
use chrono::NaiveDateTime;

/// To Do Item recieved from backend database.
#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(User))]
#[diesel(table_name = to_do)]
pub struct Item {
    pub id: i32,
    pub title: String,
    pub status: String,
    pub date: NaiveDateTime,
    pub user_id: i32,
}
