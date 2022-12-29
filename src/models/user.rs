use diesel::prelude::*;
use crate::schema::users;
use serde::{Deserialize, Serialize};

extern crate diesel;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub phone_number: String,
    pub address: String,
    pub account_number: String,
    pub disabled: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub phone_number: String,
    pub address: String,
    pub account_number: String,
    pub disabled: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct UserForm {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub phone_number: String,
    pub address: String,
    pub account_number: String,
    pub disabled: bool,
}
