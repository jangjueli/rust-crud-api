use actix_web::web;
use crate::handlers::user::{list_users, get_user, create_user, delete_user, update_user};

pub fn init_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::get().to(list_users))
            .route("{id}",web::get().to(get_user))
            .route("", web::post().to(create_user))            
            .route("{id}", web::delete().to(delete_user))            
            .route("{id}", web::put().to(update_user))      
    );
}
