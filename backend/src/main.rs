use backlib::auth;
use backlib::auth::LoginRequest;
use backlib::auth::LoginResponse;
use backlib::error;
use backlib::error::Error::WrongCredentialsError;
use backlib::establish_connection;
use backlib::models::User;
use backlib::schema::users::dsl::*;
use backlib::WebResult;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use futures_util::{FutureExt, StreamExt};
use sha256::digest;
use warp::reject;
use warp::reply;
use warp::Filter;
use warp::Reply;

use warp::{
    filters::header::headers_cloned,
    http::header::{HeaderMap, HeaderValue},
    Rejection,
};

pub fn with_auth() -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    headers_cloned()
        .map(move |headers: HeaderMap<HeaderValue>| (headers))
        .and_then(backlib::auth::authorize)
}

#[tokio::main]
async fn main() {
    let login_route = warp::path!("login")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(login_handler);

    // Test route, get your user ID of your JWT token
    let user_route = warp::path!("user").and(with_auth()).and_then(user_handler);

    let chat_route = warp::path("chat")
        // The `ws()` filter will prepare the Websocket handshake.
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            // And then our closure will be called when it completes...
            ws.on_upgrade(|websocket| {
                
                // Just echo all messages back...
                let (tx, rx) = websocket.split();
                rx.forward(tx).map(|result| {
                    if let Err(e) = result {
                        eprintln!("websocket error: {:?}", e);
                    }
                })
            })
        });

    let routes = login_route
        .or(chat_route)
        .or(user_route)
        .recover(error::handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn login_handler(body: LoginRequest) -> WebResult<impl Reply> {
    let connection = establish_connection();
    let user = users
        .filter(email.eq(&body.email))
        .limit(1)
        .load::<User>(&connection)
        .unwrap();

    if user.is_empty() {
        return Err(reject::custom(WrongCredentialsError));
    } else {
        let user = user.get(0).unwrap();
        if user.email == body.email && user.pwhash == digest(body.pw) {
            let token = auth::create_jwt(&user.id.to_string()).map_err(|e| reject::custom(e))?;
            return Ok(reply::json(&LoginResponse { token }));
        } else {
            return Err(reject::custom(WrongCredentialsError));
        }
    }
}

pub async fn user_handler(uid: String) -> WebResult<impl Reply> {
    Ok(format!("Hello User {}", uid))
}
