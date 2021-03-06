use crate::{
    core::{
        credentials::{GetCredential, GetRegister, NewCredential},
        errors::Error,
        pagination::{Page, Paginator},
        tokens::{AuthPayload, HasSession, ToToken},
    },
    schema::users,
    config::{Role, LogModel}
};
use chrono::{Duration, Utc};
use http_api_problem::StatusCode;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    id: i32,
    name: String,
    lastname: String,
    credential_id: i32,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    //contact_id: i32,
}
impl User{
    pub fn fullname(&self)->String{
        format!("{},{}",self.lastname, self.name)
    }
    pub fn id(&self)->usize{
        self.id as usize
    }
}
impl Paginator for User {}

#[derive(Serialize, Deserialize)]
pub struct UserQueries {
    pub page: Option<i64>,
    pub take: Option<i64>,
    pub name: Option<String>,
    pub lastname: Option<String>,
    pub credential_id: Option<i32>,
}
impl Page for UserQueries {
    fn get_page(&self) -> (i64, i64) {
        (self.take.unwrap_or(5), self.page.unwrap_or(1))
    }
}
impl HasSession for User {
    fn get_auth(self, log_model: LogModel) -> Result<AuthPayload, Error> {
        match Utc::now().checked_add_signed(Duration::minutes(10)) {
            Some(time) => {
                let exp = time.timestamp();
                Ok(AuthPayload::get_auth::<User>(
                    self.id,
                    log_model,
                    Role::User,
                    exp,
                ))
            }
            None => Err(Error::Redaction(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error while trying to create a timestamp".to_owned(),
            )),
        }
    }
}
#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    name: String,
    lastname: String,
    credential_id: i32,
}

#[derive(Serialize, Deserialize, AsChangeset, Validate)]
#[table_name = "users"]
pub struct UpdateUser {
    #[validate(length(min = 2, max = 55))]
    name: String,
    #[validate(length(min = 2, max = 55))]
    lastname: String,
}

#[derive(Serialize, Deserialize, Debug, Validate, Clone)]
pub struct UserRegister {
    #[validate(length(min = 5, max = 255))]
    password: String,
    #[validate(length(min = 5, max = 40), email)]
    email: String,
    #[validate(length(min = 2, max = 55))]
    name: String,
    #[validate(length(min = 2, max = 55))]
    lastname: String,
    state: Option<bool>,
    log_model: LogModel,
}
impl UserRegister {
    pub fn email(&self) -> &String {
        &self.email
    }
}
impl GetCredential<UserRegister> for UserRegister {
    fn get_credential(&self) -> Result<NewCredential, Error> {
        let clone = (*self).clone();
        NewCredential::new(clone.password, clone.email, clone.log_model, clone.state)
    }
}

impl GetRegister<UserRegister, NewUser, users::table> for UserRegister {
    fn get_register(&self, credential_id: i32) -> NewUser {
        let clone = (*self).clone();

        NewUser {
            name: clone.name,
            lastname: clone.lastname,
            credential_id,
        }
    }
}
