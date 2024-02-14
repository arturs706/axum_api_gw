mod mware;
mod user_routes;
use axum::{http::{header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}, HeaderValue, Method}, middleware, routing::{get, post}, Extension};
use dotenv::dotenv;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;
use deadpool_redis::{Pool, Runtime, Config};
use std::time::Duration;


type DeadpoolPool = Pool;


// const PREFIX: &str = "with_deadpool";
// const TTL: usize = 60 * 5;
const MAX_POOL_SIZE: usize = 50;
const WAIT_TIMEOUT: Option<Duration> = Some(Duration::from_secs(10));

pub fn create_pool() -> Result<DeadpoolPool, String> {
    dotenv().ok();
    let redis_url: String = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
    let config = Config::from_url(redis_url);
    config
        .builder()
        .map(|b| {
            b.max_size(MAX_POOL_SIZE)
                .wait_timeout(WAIT_TIMEOUT) // TODO needs create_timeout/recycle timeout?
                .runtime(Runtime::Tokio1)
                .build()
                .unwrap() // TODO don't panic. flat_map can't be used???
        })
        .map_err(|e| e.to_string())
}


#[tokio::main]
async fn main() {
    dotenv().ok();
    let cors = CorsLayer::new()
    .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
    .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
    .allow_credentials(true)
    .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);
    let redis_pool_data = create_pool().unwrap();

    let app = axum::Router::new()
        .route("/api/v1", get(user_routes::fetchusershandler))
        .route("/api/v1/login", post(user_routes::login_user))
        .layer(cors)
        .layer(CookieManagerLayer::new())
        .layer(middleware::from_fn(mware::add_token))
        .layer(Extension(redis_pool_data));
        
    let listener = tokio::net::TcpListener::bind("0.0.0.0:10000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


