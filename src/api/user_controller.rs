use actix_web::{web, Error, HttpResponse};

use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager}
};

use crate::{
    models::user::UserForm,
    services::user_service
};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;


// POST api/user
pub async fn insert(
    params: web::Form<UserForm>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {

    web::block(move || {
        let mut conn = pool.get()?;
        user_service::insert(params, &mut conn)
    }).await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("User was added successfully!")))
}


// GET api/user
pub async fn find_all(
    pool: web::Data<DbPool>
) -> Result<HttpResponse, Error> {

    // use web::block to offload blocking Diesel code without blocking server thread
    let users = web::block(move || {
        let mut conn = pool.get()?;
        user_service::find_all(&mut conn)
    }).await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_users = serde_json::to_string(&users)?;
    let res = HttpResponse::Ok().body(all_users);
    Ok(res)
}


// GET api/user/id/{id}
pub async fn find_by_id(
    id: web::Path<i32>,
    pool: web::Data<DbPool>
) -> Result<HttpResponse, Error> {

    let user_id = id.into_inner();

    let user = web::block(move || {
        let mut conn = pool.get()?;
        user_service::find_by_id(user_id, &mut conn)
    }).await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound().body(format!("No user found with the id: {user_id}"));
        Ok(res)
    }
}


// PUT api/user/id/{id}
pub async fn update(
    id: web::Path<i32>,
    params: web::Form<UserForm>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {

    web::block(move || {
        let mut conn = pool.get()?;
        user_service::update(id.into_inner(), params, &mut conn)
    }).await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("User was updated successfully!")))
}


// DELETE api/user/id/{id}
pub async fn delete(
    id: web::Path<i32>,
    pool: web::Data<DbPool>
) -> Result<HttpResponse, Error> {

    web::block(move || {
        let mut conn = pool.get()?;
        user_service::delete(id.into_inner(), &mut conn)
    }).await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("User was deleted successfully!")))
}


// pub async fn index() -> Result<HttpResponse, Error> {
//     Ok(HttpResponse::Ok()
//         .content_type("text/html; charset=utf-8")
//         .body(include_str!("../../static/register.html")))
// }