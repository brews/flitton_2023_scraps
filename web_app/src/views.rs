mod app;
mod auth;
mod to_do;
mod users;

use actix_web::web::ServiceConfig;
use app::app_views_factory;
use auth::auth_views_factory;
use to_do::to_do_views_factory;
use users::user_views_factory;

/// Register all services.
pub fn views_factory(app: &mut ServiceConfig) {
    app_views_factory(app);
    auth_views_factory(app);
    to_do_views_factory(app);
    user_views_factory(app);
}
