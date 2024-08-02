use crate::diesel;
use actix_web::{HttpRequest, HttpResponse};
use diesel::prelude::*;

use crate::database::establish_connection;
use crate::json_serialization::to_do_items::ToDoItems;
use crate::jwt::JwToken;
use crate::models::item::item::Item;
use crate::models::item::new_item::NewItem;
use crate::schema::to_do;

pub async fn create(token: JwToken, req: HttpRequest) -> HttpResponse {
    let title: String = req.match_info().get("title").unwrap().to_string();

    let mut connection = establish_connection();

    // Requires title to be universally unique across user_id?!?
    let items = to_do::table
        .filter(to_do::columns::title.eq(&title.as_str()))
        .order(to_do::columns::id.asc())
        .load::<Item>(&mut connection)
        .unwrap();

    // Create new item for user_id, but only if it doesn't already exist.
    if items.len() == 0 {
        let new_post = NewItem::new(title, token.user_id);
        let _ = diesel::insert_into(to_do::table)
            .values(&new_post)
            .execute(&mut connection);
    }

    // process_input(item, "create".to_string(), &state);
    return HttpResponse::Ok().json(ToDoItems::get_state(token.user_id));
}
