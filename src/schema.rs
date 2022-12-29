// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        phone_number -> Varchar,
        address -> Varchar,
        account_number -> Varchar,
        disabled -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
