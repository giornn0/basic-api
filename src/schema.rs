
table! {
    use diesel::{sql_types::{Nullable,Bool,Text, Timestamptz}, types::{ Int4, Varchar}};
    use crate::config::LogModelMapping;
    credentials (id) {
        id -> Int4,
        password -> Text,
        email -> Varchar,
        state -> Nullable<Bool>,
        log_model -> LogModelMapping,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    organizations (id) {
        id -> Int4,
        name -> Varchar,
        active -> Nullable<Bool>,
        logo -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    tokens (id) {
        id -> Int4,
        model -> Varchar,
        token -> Text,
        expiration_date -> Nullable<Timestamptz>,
        state -> Nullable<Bool>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Int4,
        lastname -> Varchar,
        name -> Varchar,
        credential_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

joinable!(users -> credentials (credential_id));

allow_tables_to_appear_in_same_query!(
    credentials,
    organizations,
    tokens,
    users,
);
