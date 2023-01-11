use uuid::Uuid;
use validator_derive::Validate;
use super::*;
use crate::schema::userss;
use diesel::{prelude::*, Insertable, Queryable, AsChangeset};
use serde::{Serialize, Deserialize};
use std::hash::{Hash, Hasher};
use chrono::NaiveDateTime;
use validator::Validate;

#[derive(Serialize,Deserialize,AsChangeset, Insertable, Hash,Queryable)]
#[table_name = "userss"]
pub struct User{
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub full_name: String,
    #[serde(skip_serializing)]
    pub created_at: NaiveDateTime,
    #[serde(skip_serializing)]
    pub updated_at: NaiveDateTime,

}

#[derive(Serialize,Deserialize,AsChangeset, Insertable, Hash,Queryable, Validate)]
#[table_name = "userss"]
pub struct NewUser{
    #[validate(length(min = 3))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 3))]
    pub password_hash: String,
    pub full_name: String,
}
