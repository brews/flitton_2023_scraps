use crate::diesel;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use std::collections::HashMap;

use crate::database::establish_connection;
use crate::json_serialization::login::Login;
use crate::jwt::JwToken;
use crate::models::user::user::User;
use crate::schema::users;

pub async fn login(credentials: web::Json<Login>) -> HttpResponse {
    let password = credentials.password.clone();
    let mut conn = establish_connection();
    let users = users::table
        .filter(users::columns::username.eq(credentials.username.clone()))
        .load::<User>(&mut conn)
        .unwrap();

    // Check that we have one and only one user.
    if users.len() == 0 {
        return HttpResponse::NotFound().await.unwrap();
    } else if users.len() > 1 {
        return HttpResponse::Conflict().await.unwrap();
    }

    match users[0].verify(password) {
        true => {
            // If password matches return token from user ID in header of response.
            let token = JwToken::new(users[0].id);
            let raw_token = token.encode();

            // NOTE This is not a best practice. Returning in the body of a POST response
            // because the book says it keeps things simple when we go on to develop
            // the front end.
            let mut body = HashMap::new();
            body.insert("token", raw_token.clone());
            HttpResponse::Ok()
                .append_header(("token", raw_token))
                .json(body)
        }
        false => HttpResponse::Unauthorized().await.unwrap(),
    }
}
