use crate::{
    schema::organizations, core::pagination::{Paginator, Page},
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
impl Paginator for Organization {}
#[derive(Serialize, Deserialize)]
pub struct OrganizationQueries {
    pub page: Option<i64>,
    pub take: Option<i64>,
    pub name: Option<String>,
    pub lastname: Option<String>,
    pub credential_id: Option<i32>,
}
impl Page for OrganizationQueries {
    fn get_page(&self) -> (i64, i64) {
        (self.take.unwrap_or(5), self.page.unwrap_or(1))
    }
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



