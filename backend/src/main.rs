use std::collections::HashMap;

use backlib::establish_connection;
use backlib::models::User;
use backlib::schema::users::dsl::*;
use diesel::RunQueryDsl;
use warp::Filter;

#[tokio::main]
async fn main() {
    let get_users = warp::get()
        .and(warp::path("api"))
        .and(warp::path("users"))
        .and(warp::path::end())
        .and_then(get_users);

    warp::serve(get_users).run(([127, 0, 0, 1], 3030)).await;
}

async fn get_users() -> Result<impl warp::Reply, warp::Rejection> {
    let mut result = HashMap::new();

    let connection = establish_connection();
    let results = users
        .load::<User>(&connection)
        .expect("Error loading posts");

    for user in results {
        result.insert(user.email, user.username);
    }

    Ok(warp::reply::json(&result))
}
