use crate::api::*;
use crate::config::*;

use actix_web::web;
use actix_web_httpauth::{middleware::HttpAuthentication};

pub fn config_services(cfg: &mut web::ServiceConfig){
    cfg.service(
      web::scope("api")
          .service(
              web::scope("/user")
                  .wrap(HttpAuthentication::bearer(auth::bearer_token_validator))
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
          .service(
              web::scope("/home")
                  .service(
                      web::resource("")
                      .route(web::get().to(auth_controller::home))
                  )
          )
          .service(
              web::scope("/register")
                  .service(
                      web::resource("")
                          .route(web::get().to(auth_controller::register))
                  )
          )
          .service(
              web::scope("/login")
                  // .wrap(HttpAuthentication::basic(auth::basic_token_validator))
                  .service(
                      web::resource("")
                          .route(web::post().to(auth_controller::login))
                  )
          )
    );
}
