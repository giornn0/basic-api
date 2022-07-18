
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::schema::credentials;

#[derive(Queryable,Serialize,Deserialize)]
pub struct Credential{
    id: i32,
    password: String,
    email: String,
    state: Option<bool>,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    //contact_id: i32,
}
#[derive(Serialize, Deserialize, Debug, Insertable, AsChangeset, Validate)]
#[table_name = "credentials"]
pub struct NewCredential {
    #[validate(length(min = 2, max = 55))]
    password: String,
    #[validate(email)]
    email: String,
    state: Option<bool>,
}

#[derive(Serialize,Deserialize)]
pub struct Queries{

}