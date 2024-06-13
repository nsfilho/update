use axum::{
    routing::{get, post},
    Router,
};
use tracing::info;
use tracing_subscriber::FmtSubscriber;

mod config;
mod controllers;
mod services;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let config = config::Config::load();
    let subscriber = FmtSubscriber::builder()
        .with_max_level(config.log_level())
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // build our application with a single route
    let app = Router::new()
        .route("/", get(controllers::echo::get_root))
        .route("/update", post(controllers::update::update_service));

    // run our app with hyper, listening globally on port 3000
    let server_addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&server_addr).await.unwrap();
    info!("Starting server: {}", server_addr);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
