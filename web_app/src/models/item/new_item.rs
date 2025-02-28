use crate::schema::to_do;
use chrono::{NaiveDateTime, Utc};

/// New To Do Item to insert into database.
#[derive(Insertable)]
#[diesel(table_name = to_do)]
pub struct NewItem {
    pub title: String,
    pub status: String,
    pub date: NaiveDateTime,
    pub user_id: i32,
}

impl NewItem {
    pub fn new(title: String, user_id: i32) -> NewItem {
        let now = Utc::now().naive_local();
        return NewItem {
            title,
            status: String::from("PENDING"),
            date: now,
            user_id,
        };
    }
}
