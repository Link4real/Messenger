pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use sha256::digest;
use std::env;

use crate::models::User;

#[tokio::main]
async fn main() {
    use schema::users::dsl::*;

    let connection = establish_connection();
    let results = users
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{}", user.email);
        println!("{}", user.username);
        println!("\n");
    }
}

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
