use crate::{
    core::credentials::{LogModel, NewCredential, GetRegister, GetCredential},
    schema::users,
};
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

#[derive(Serialize, Deserialize)]
pub struct Queries {}

#[derive(Serialize, Deserialize, Debug, Validate,Clone)]
pub struct UserRegister {
    #[validate(length(min = 2, max = 255))]
    password: String,
    #[validate(length(min = 2, max = 40), email)]
    email: String,
    #[validate(length(min = 2, max = 55))]
    name: String,
    #[validate(length(min = 2, max = 55))]
    lastname: String,
    state: Option<bool>,
    log_model: LogModel,
}
impl UserRegister{
    pub fn email(&self)->&String{
        &self.email
    }
}
impl GetCredential<UserRegister> for UserRegister {
    fn get_credential(&self) -> NewCredential {
        let clone = (*self).clone();
        NewCredential::new(
            clone.password,
            clone.email,
            clone.log_model,
            clone.state
        )
    }
}

impl GetRegister<UserRegister, NewUser, users::table> for UserRegister {
    fn get_register(&self, credential_id: i32) -> NewUser {
        let clone = (*self).clone();

        NewUser{
            name:clone.name,
            lastname:clone.lastname,
            credential_id
        }
    }
}
