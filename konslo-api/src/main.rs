mod error;
mod extractor;
mod handlers;
mod jwt;
mod requests;
mod responses;
mod router;

use crate::router::{AppState, get_router};
use axum::http::{header, HeaderValue, Method};
use dotenvy::dotenv;
use konslo_core::db::run_migrations;
use konslo_core::repositories::{PostgresCheckRepository, PostgresHabitRepository, PostgresUserRepository};
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

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:4200".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_headers([header::CONTENT_TYPE])
        .allow_credentials(true);

    // load env. variables
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL should be defined in the .env file");
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET should be defined in the .env file");

    // create the pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await
        .expect("failed to connect to database");

    // run migrations
    run_migrations(&pool).await.expect("failed to run migration");

    // wire dependencies
    let user_repo = Arc::new(PostgresUserRepository::new(pool.clone()));
    let habit_repo = Arc::new(PostgresHabitRepository::new(pool.clone()));
    let check_repo = Arc::new(PostgresCheckRepository::new(pool.clone()));

    let user_service = Arc::new(DefaultUserService::new(user_repo.clone()));
    let habit_service = Arc::new(DefaultHabitService::new(habit_repo.clone(), check_repo.clone()));

    let state = AppState {
        habit_service,
        user_service,
        jwt_secret,
    };

    // 4. Config routing
    let app = get_router(state).layer(cors);

    // 3. Définition de l'adresse (localhost:3000)
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {}", addr);

    // 4. Lancement du serveur
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
