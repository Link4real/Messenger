use crate::schema::*;
use serde::Serialize;
use serde::Deserialize;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub username: &'a str,
    pub pwhash: &'a str,
}

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub pwhash: String,
}

#[derive(Queryable)]

pub struct Message {
    pub id: i32,
    pub author: String,
    pub body: String,
    pub stamp: chrono::NaiveDateTime,
}

#[derive(Insertable, Serialize)]
#[table_name = "messages"]
pub struct NewMessage<'a> {
    pub author: &'a str,
    pub body: &'a str,
    pub stamp: &'a chrono::NaiveDateTime,
}
