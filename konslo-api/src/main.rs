mod auth;
mod error;
mod handlers;
mod requests;
mod responses;
mod router;

use crate::router::{AppState, get_router};
use axum::http::{HeaderValue, Method, header};
use dotenvy::dotenv;
use konslo_core::db::run_migrations;
use konslo_core::repositories::check::PostgresCheckRepository;
use konslo_core::repositories::demo_users::PostgresDemoUserRepository;
use konslo_core::repositories::habit::PostgresHabitRepository;
use konslo_core::repositories::user::PostgresUserRepository;
use konslo_core::services::demo_user::DefaultDemoUserService;
use konslo_core::services::habit::DefaultHabitService;
use konslo_core::services::user::DefaultUserService;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // instantiate the logger
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    // load environment variables
    dotenv().ok();
    let allowed_origin = env::var("ALLOWED_ORIGIN").expect("ALLOWED_ORIGIN must be set");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let cors = CorsLayer::new()
        .allow_origin(
            allowed_origin
                .parse::<HeaderValue>()
                .expect("ALLOWED_ORIGIN is not a valid header value"),
        )
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_headers([header::CONTENT_TYPE])
        .allow_credentials(true);

    // create the pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await
        .expect("failed to connect to database");

    // run migrations
    run_migrations(&pool)
        .await
        .expect("failed to run migration");

    // wire dependencies
    let demo_user_repo = Arc::new(PostgresDemoUserRepository::new(pool.clone()));
    let user_repo = Arc::new(PostgresUserRepository::new(pool.clone()));
    let habit_repo = Arc::new(PostgresHabitRepository::new(pool.clone()));
    let check_repo = Arc::new(PostgresCheckRepository::new(pool.clone()));

    let demo_user_service = Arc::new(DefaultDemoUserService::new(
        demo_user_repo.clone(),
        user_repo.clone(),
        habit_repo.clone(),
        check_repo.clone(),
    ));
    let user_service = Arc::new(DefaultUserService::new(user_repo.clone()));
    let habit_service = Arc::new(DefaultHabitService::new(
        habit_repo.clone(),
        check_repo.clone(),
    ));

    let state = AppState {
        demo_user_service,
        user_service,
        habit_service,
        jwt_secret,
    };

    // config routing
    let app = get_router(state).layer(cors);

    // run server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
