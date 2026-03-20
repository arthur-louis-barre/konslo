mod error;
mod handlers;
mod router;
mod requests;
mod responses;

use crate::router::{get_router, AppState};
use dotenvy::dotenv;
use konslo_core::repositories::habit::PostgresHabitRepository;
use konslo_core::services::habit::DefaultHabitService;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use konslo_core::repositories::check::PostgresCheckRepository;
use konslo_core::services::check::DefaultCheckService;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // 1. Load env. variables
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Database must be defined in the .env file");

    // 1. Initialisation des logs (pour voir ce qui se passe)
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 2. Create the pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // 3. Instantiate the repository
    let habit_repo = Arc::new(PostgresHabitRepository::new(pool.clone()));
    let habit_service = Arc::new(DefaultHabitService::new(habit_repo.clone()));
    let check_repo = Arc::new(PostgresCheckRepository::new(pool));
    let check_service = Arc::new(DefaultCheckService::new(check_repo, habit_repo));
    let state = AppState { habit_service, check_service };

    // 4. Config routing
    let app = get_router(state).layer(cors);

    // 3. Définition de l'adresse (localhost:3000)
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {}", addr);

    // 4. Lancement du serveur
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
