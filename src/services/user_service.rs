use actix_web::web;
use diesel::prelude::*;
use crate::models;

use chrono::Local;

type DbError = Box<dyn std::error::Error + Send + Sync>;


pub fn insert (
    fields: web::Form<models::user::UserForm>,
    conn: &mut PgConnection
) -> Result<(), DbError> {
    use crate::schema::users::dsl::*;

    let field = fields.clone();
    let get_current_time = Local::now().naive_local();

    let new_user = models::user::NewUser {
        first_name: field.first_name,
        last_name: field.last_name,
        email: field.email,
        password: field.password,
        phone_number: field.phone_number,
        address: field.address,
        account_number: field.account_number,
        disabled: field.disabled,
        created_at: get_current_time,
        updated_at: get_current_time,
    };

    diesel::insert_into(users)
        .values(new_user)
        .execute(conn)?;

    Ok(())
}


pub fn find_all (
    conn: &mut PgConnection
) -> Result<Vec<models::user::User>, DbError > {
    use crate::schema::users::dsl::*;

    let all_users = users
            .load::<models::user::User>(conn)?;

    Ok(all_users)
}


pub fn find_by_id (
    user_id: i32,
    conn: &mut PgConnection
) -> Result<Option<models::user::User>, DbError> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(id.eq(user_id))
        .first::<models::user::User>(conn)
        .optional()?;

    Ok(user)
}


pub fn update (
    user_id: i32,
    fields: web::Form<models::user::UserForm>,
    conn: &mut PgConnection
) -> Result<(), DbError> {
    use crate::schema::users::dsl::*;

    let field = fields.clone();
    let current_time = Local::now().naive_local();

    // Get old values
    let old_user_values = users
        .filter(id.eq(&user_id))
        .first::<models::user::User>(conn)
        .optional()?;

    // compare values
    match old_user_values {
        None => (),
        Some(values) => {

            if field.first_name != values.first_name {
                diesel::update(users.filter(id.eq(&user_id)))
                    .set((first_name.eq(&field.first_name),
                          updated_at.eq(&current_time)))
                    .execute(conn)?;
            }

            if field.last_name != values.last_name {
                diesel::update(users.filter(id.eq(&user_id)))
                    .set((last_name.eq(&field.last_name),
                          updated_at.eq(&current_time)))
                    .execute(conn)?;
            }

            if field.email != values.email {
                diesel::update(users.filter(id.eq(&user_id)))
                    .set((email.eq(&field.email),
                          updated_at.eq(&current_time)))
                    .execute(conn)?;
            }

            if field.password != values.password {
                diesel::update(users.filter(id.eq(&user_id)))
                    .set((password.eq(&field.password),
                          updated_at.eq(&current_time)))
                    .execute(conn)?;
            }

            if field.phone_number != values.phone_number {
                diesel::update(users.filter(id.eq(&user_id)))
                    .set((phone_number.eq(&field.phone_number),
                          updated_at.eq(&current_time)))
                    .execute(conn)?;
            }

            if field.address != values.address {
                diesel::update(users.filter(id.eq(&user_id)))
                    .set((address.eq(&field.address),
                          updated_at.eq(&current_time)))
                    .execute(conn)?;
            }

            if field.account_number != values.account_number {
                diesel::update(users.filter(id.eq(&user_id)))
                    .set((account_number.eq(&field.account_number),
                          updated_at.eq(&current_time)))
                    .execute(conn)?;
            }

            if field.disabled != values.disabled {
                diesel::update(users.filter(id.eq(&user_id)))
                    .set((disabled.eq(&field.disabled),
                          updated_at.eq(&current_time)))
                    .execute(conn)?;
            }

        }
    }

    Ok(())
}


pub fn delete (
    user_id: i32,
    conn: &mut PgConnection
) -> Result<(), DbError> {
    use crate::schema::users::dsl::*;

    diesel::delete(users.filter(id.eq(user_id)))
        .execute(conn)
        .expect("Error deleting user");

    Ok(())
}