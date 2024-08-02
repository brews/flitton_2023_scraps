use crate::diesel;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;

use crate::database::establish_connection;
use crate::json_serialization::{to_do_item::ToDoItem, to_do_items::ToDoItems};
use crate::jwt::JwToken;
use crate::schema::to_do;

pub async fn edit(to_do_item: web::Json<ToDoItem>, token: JwToken) -> HttpResponse {
    let mut connection = establish_connection();

    // Match requested task by title AND user_id.
    let results = to_do::table
        .filter(to_do::columns::title.eq(&to_do_item.title))
        .filter(to_do::columns::user_id.eq(&token.user_id));

    let _ = diesel::update(results)
        .set(to_do::columns::status.eq("DONE"))
        .execute(&mut connection);

    return HttpResponse::Ok().json(ToDoItems::get_state(token.user_id));
}
