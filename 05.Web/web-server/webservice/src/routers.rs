use super::handlers::*;
use actix_web::web;

// 配置 route
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/courses"))
        .route("/", web::post().to(new_course))
        .route("/{user_id}", web::get().to(get_courses_from_teacher))
        .route("/{user_id}/{course_id}", web::get().to(get_course_detail));
}
