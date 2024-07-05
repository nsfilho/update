//! # Objectives
//!
//! * This application is a simple api for updating services in your docker swarm.
//! * When the update find some service using the same image, it will update for a new tag.
//!
//! # Configure
//!
//! For configure the application, please follow the instructions in [config::Config](config/struct.Config.html).
//!
//! # Development
//!
//! You need convert the unix socket from docker to http. You can use the [socat](http://www.dest-unreach.org/socat/) for this.
//!
//! ```bash
//! socat TCP-LISTEN:8080,bind=127.0.0.1,reuseaddr,fork,range=127.0.0.0/8 UNIX-CLIENT:/var/run/docker.sock
//! ```
//!
use std::{sync::Arc, time::Duration};

use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use tokio::signal;
use tower_http::{
    cors::{AllowOrigin, CorsLayer},
    limit::RequestBodyLimitLayer,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing::info;
use tracing_subscriber::FmtSubscriber;

mod config;
mod controllers;
mod services;

struct AppState {
    docker: services::docker::Docker,
}

/// Main entrypoint for the application
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let config = config::Config::load();

    // config log_level
    let subscriber = FmtSubscriber::builder()
        .with_max_level(config.log_level())
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // create our application state
    let app_state = Arc::new(AppState {
        docker: services::docker::DockerBuilder::builder()
            .with_http_url(&config.docker_url)
            .build(),
    });

    // build our application
    let app = Router::new()
        .route("/", get(controllers::echo::get_root))
        .route("/update", post(controllers::update::update_service))
        .layer(TimeoutLayer::new(Duration::from_secs(
            config.http_request_timeout,
        )))
        .layer(RequestBodyLimitLayer::new(config.http_body_limit))
        .layer((
            TraceLayer::new_for_http(),
            TimeoutLayer::new(Duration::from_secs(config.graceful_shutdown_timeout)),
        ))
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::any())
                .allow_methods([Method::GET, Method::POST, Method::OPTIONS]),
        )
        .with_state(app_state);

    // run our app with hyper, listening globally on port 3000
    let server_addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&server_addr).await.unwrap();
    info!("Starting server: {}", server_addr);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
