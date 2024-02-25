use axum::{
    routing::get,
    Router,
};
use crate::routes::user::get_user_handler;

mod routes {
    pub mod user;
}


#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" })).route("/user", get(get_user_handler));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
