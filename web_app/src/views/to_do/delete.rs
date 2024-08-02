use crate::diesel;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;

use crate::database::establish_connection;
use crate::json_serialization::{to_do_item::ToDoItem, to_do_items::ToDoItems};
use crate::jwt::JwToken;
use crate::models::item::item::Item;
use crate::schema::to_do;

pub async fn delete(to_do_item: web::Json<ToDoItem>, token: JwToken) -> HttpResponse {
    let mut connection = establish_connection();

    // Match task by title and user_id.
    let items = to_do::table
        .filter(to_do::columns::title.eq(&to_do_item.title))
        .filter(to_do::columns::user_id.eq(&token.user_id))
        .order(to_do::columns::id.asc())
        .load::<Item>(&mut connection)
        .unwrap();

    let _ = diesel::delete(&items[0]).execute(&mut connection);

    return HttpResponse::Ok().json(ToDoItems::get_state(token.user_id));
}
