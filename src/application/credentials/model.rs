
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::{schema::credentials, utils::passwords::hash};

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
impl NewCredential{
    pub fn hash_password(&mut self)-> &Self{
        match hash(&self.password){
            Some(hash) =>{
                self.password = hash;
                self
            },
            None=>self
        }
    }
}

#[derive(Serialize,Deserialize)]
pub struct Queries{

}