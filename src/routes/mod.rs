pub mod user;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    user::init_user_routes(cfg);
}
