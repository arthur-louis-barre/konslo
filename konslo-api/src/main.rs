mod error;
mod handlers;
mod router;

use dotenvy::dotenv;
use konslo_core::db::habits::PostgresHabitRepository;
use konslo_core::service::habit::HabitService;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use crate::router::get_router;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // 1. Load env. variables
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("Database must be defined in the .env file");

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
    let habit_repo = Arc::new(PostgresHabitRepository::new(pool));
    let habit_service = HabitService::new(habit_repo);

    // 4. Config routing
    let app = get_router(habit_service).layer(cors);

    // 3. Définition de l'adresse (localhost:3000)
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);

    // 4. Lancement du serveur
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
