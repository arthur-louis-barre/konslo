use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // 1. Initialisation des logs (pour voir ce qui se passe)
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 2. Construction du routeur
    let app = Router::new()
        .route("/", get(|| async { "Hello Konslo" }));

    // 3. Définition de l'adresse (localhost:3000)
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);

    // 4. Lancement du serveur
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}