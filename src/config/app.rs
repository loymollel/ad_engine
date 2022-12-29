use crate::api::*;
use actix_web::web;

pub fn config_services(cfg: &mut web::ServiceConfig){
    cfg.service(
      web::scope("api")
          .service(
              web::scope("/user")
                  .service(
                    web::resource("")
                        .route(web::post().to(user_controller::insert))
                        .route(web::get().to(user_controller::find_all))
                    // .route(web::get().to(user_controller::index))
                  )
                  .service(
                    web::resource("/id/{id}")
                        .route(web::get().to(user_controller::find_by_id))
                        .route(web::put().to(user_controller::update))
                        .route(web::delete().to(user_controller::delete))
                  )
          )
    );
}
