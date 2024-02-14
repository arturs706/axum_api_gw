mod mware;
mod user_routes;
use axum::{http::{header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}, HeaderValue, Method}, middleware, routing::{get, post}};
use dotenv::dotenv;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;



#[tokio::main]
async fn main() {
    dotenv().ok();
    let cors = CorsLayer::new()
    .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
    .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
    .allow_credentials(true)
    .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);


    let app = axum::Router::new()
        .route("/api/v1", get(user_routes::fetchusershandler))
        .route("/api/v1/login", post(user_routes::login_user))
        .layer(cors)
        .layer(CookieManagerLayer::new())
        .layer(middleware::from_fn(mware::add_token));
        
    let listener = tokio::net::TcpListener::bind("0.0.0.0:10000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


