
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::schema::users;

#[derive(Queryable,Serialize,Deserialize)]
pub struct User{
    id: i32,
    name: String,
    lastname: String,
    credential_id: i32,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    //contact_id: i32,
}
#[derive(Serialize, Deserialize, Debug, Insertable, AsChangeset, Validate)]
#[table_name = "users"]
pub struct NewUser {
    #[validate(length(min = 2, max = 55))]
    name: String,
    #[validate(length(min = 2, max = 55))]
    lastname: String,
    credential_id: i32,
}

#[derive(Serialize,Deserialize)]
pub struct Queries{

}