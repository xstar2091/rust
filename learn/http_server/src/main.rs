use axum::{
    routing::{get},
    Router,
};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use http_server::handler::health::health;
use http_server::handler::common::AppState;
use http_server::handler::order::{get_order, get_order_list};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "http_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let state = AppState::new("./data.toml");
    let app = Router::new()
        .route("/health", get(health))
        // /orders/{id}?limit={limit}&offset={offset}
        .route("/orders/{id}", get(get_order))
        // /orders?limit={limit}&offset={offset}
        .route("/orders", get(get_order_list))
        .with_state(state)
        .layer(TraceLayer::new_for_http());
    let listener = match tokio::net::TcpListener::bind("0.0.0.0:7878").await {
        Ok(listener) => listener,
        Err(err) => {
            tracing::error!(
                error = %err,
                "http server listen 7878 failed"
            );
            std::process::exit(1);
        },
    };
    tracing::info!("listening on 0.0.0.0:7878 started");
    match axum::serve(listener, app).await {
        Err(err) => {
            tracing::error!(
                error = %err,
                "http server listener failed to start"
            );
            std::process::exit(1);
        },
        Ok(_) => {
            tracing::info!("listening on 0.0.0.0:7878 stopped");
        },
    }
}
