use crate::diesel;
use diesel::prelude::*;

use crate::database::establish_connection;
use crate::models::item::item::Item;
use crate::schema::to_do;

use actix_web::{body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder};
use serde::Serialize;

use crate::to_do::structs::base::Base;
use crate::to_do::ItemTypes;
use crate::to_do::{enums::TaskStatus, to_do_factory};

#[derive(Serialize)]
pub struct ToDoItems {
    pub pending_items: Vec<Base>,
    pub done_items: Vec<Base>,
    pub pending_item_count: i8,
    pub done_item_count: i8,
}

impl ToDoItems {
    pub fn new(input_items: Vec<ItemTypes>) -> ToDoItems {
        let mut pending_array_buffer = Vec::new();
        let mut done_array_buffer = Vec::new();

        for item in input_items {
            match item {
                ItemTypes::Pending(packed) => pending_array_buffer.push(packed.super_struct),
                ItemTypes::Done(packed) => done_array_buffer.push(packed.super_struct),
            }
        }

        let done_count: i8 = done_array_buffer.len() as i8;
        let pending_count: i8 = pending_array_buffer.len() as i8;

        return ToDoItems {
            pending_items: pending_array_buffer,
            done_item_count: done_count,
            pending_item_count: pending_count,
            done_items: done_array_buffer,
        };
    }

    /// Get ToDoItems state from storage.
    pub fn get_state(user_id: i32) -> ToDoItems {
        let mut connection = establish_connection();

        let items: Vec<Item> = to_do::table
            .filter(to_do::columns::user_id.eq(&user_id))
            .order(to_do::columns::id.asc())
            .load::<Item>(&mut connection)
            .unwrap();

        let mut array_buffer = Vec::new();

        for item in items {
            let status = TaskStatus::from_string(item.status);
            let item = to_do_factory(&item.title, status);
            array_buffer.push(item);
        }

        return ToDoItems::new(array_buffer);
    }
}

impl Responder for ToDoItems {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
