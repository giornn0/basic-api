use crate::{
    schema::organizations,
};
use serde::{Deserialize, Serialize};
use validator::Validate;


#[derive(Queryable, Serialize, Deserialize)]
pub struct Organization {
    id: i32,
    name: String,
    active: Option<bool>,
    logo: String,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    //contact_id: i32,
}
#[derive(Serialize, Deserialize, Insertable, Validate)]
#[table_name = "organizations"]
pub struct NewOrganization {
    #[validate(length(min = 2, max = 55))]
    name: String,
    active: Option<bool>,
    logo: String,
}

#[derive(Serialize, Deserialize, AsChangeset, Validate)]
#[table_name = "organizations"]
pub struct UpdateOrganization {
    #[validate(length(min = 2, max = 55))]
    name: String,
    active: Option<bool>,
    logo: String,
}

#[derive(Serialize, Deserialize)]
pub struct Queries {}

