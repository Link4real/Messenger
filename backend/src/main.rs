use backlib::establish_connection;
use backlib::models::User;
use backlib::schema::users::dsl::*;
use diesel::{QueryDsl, RunQueryDsl};

#[tokio::main]
async fn main() {
    let connection = establish_connection();
    let results = users
        .load::<User>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{}:{}:{}", user.email, user.username, user.pwhash);
    }
}
