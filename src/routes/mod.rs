use actix_web::web;

mod users;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(users::create_user)
        .service(users::get_users)
        .service(users::get_user_by_id)
        .service(users::update_user_by_id)
        .service(users::delete_user_by_id);
}
