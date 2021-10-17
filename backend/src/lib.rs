#[macro_use]
extern crate diesel;
extern crate dotenv;
pub mod auth;
pub mod error;
pub mod models;
pub mod schema;
use crate::models::User;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use sha256::digest;
use std::env;
use warp::Rejection;

pub type Result<T> = std::result::Result<T, error::Error>;
pub type WebResult<T> = std::result::Result<T, Rejection>;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

use self::models::NewUser;

pub fn create_user<'a>(
    conn: &PgConnection,
    email: &'a str,
    username: &'a str,
    password: &'a str,
) -> User {
    let result = digest(password);

    let new_user = NewUser {
        email: email,
        username: username,
        pwhash: &result,
    };

    diesel::insert_into(schema::users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new user")
}
