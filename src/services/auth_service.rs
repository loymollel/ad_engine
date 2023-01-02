use actix_web::web;
use diesel::prelude::*;
use crate::models;
use chrono::Local;
use bcrypt::{DEFAULT_COST, hash, /* verify */};

type DbError = Box<dyn std::error::Error + Send + Sync>;


pub fn login (
    fields: web::Form<models::user::UserForm>,
    conn: &mut PgConnection
) -> Result<(), DbError> {
    use crate::schema::users::dsl::*;

    let field = fields.clone();
    let get_current_time = Local::now().naive_local();
    let hashed_password = hash(field.password, DEFAULT_COST)?;

    let new_user = models::user::NewUser {
        first_name: field.first_name,
        last_name: field.last_name,
        email: field.email,
        password: hashed_password,
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


pub fn register (
    fields: web::Form<models::user::UserForm>,
    conn: &mut PgConnection
) -> Result<(), DbError> {
    use crate::schema::users::dsl::*;

    let field = fields.clone();

    let user = users
        .filter(email.eq(&field.email))
        .get_result::<models::user::User>(conn);

    if user.is_err() {
        let get_current_time = Local::now().naive_local();
        let hashed_password = hash(field.password, DEFAULT_COST)?;

        let new_user = models::user::NewUser {
            first_name: field.first_name,
            last_name: field.last_name,
            email: field.email,
            password: hashed_password,
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
    } else {
        // Err(format!("Email '{}' is already registered", &field.email))
        Ok(())
    }


}