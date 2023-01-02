use actix_web::{web, Error, HttpResponse};

use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager}
};

use crate::{
    models::user::UserForm,
    services::auth_service
};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;


// GET api/home
pub async fn login_page() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../static/login.html")))
}

// GET api/register_page
pub async fn register_page() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../static/register.html")))
}


// POST api/register
pub async fn register(
    params: web::Form<UserForm>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {

    web::block(move || {
        let mut conn = pool.get()?;
        auth_service::register(params, &mut conn)
    }).await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("User was added successfully!")))

}


// POST api/login
pub async fn login(
    params: web::Form<UserForm>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {

    web::block(move || {
        let mut conn = pool.get()?;
        auth_service::login(params, &mut conn)
    }).await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("User was logged in successfully!")))
}