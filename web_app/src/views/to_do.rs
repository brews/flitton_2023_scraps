mod create;
mod delete;
mod edit;
mod get;

use actix_web::web::{get, post, scope, ServiceConfig};

/// Register To Do services.
pub fn to_do_views_factory(app: &mut ServiceConfig) {
    // Ew. See https://stackoverflow.com/questions/12142652/what-is-the-usefulness-of-put-and-delete-http-request-methods
    app.service(
        scope("v1/item")
            .route("create/{title}", post().to(create::create))
            .route("delete", post().to(delete::delete))
            .route("get", get().to(get::get))
            .route("edit", post().to(edit::edit)),
    );
}
