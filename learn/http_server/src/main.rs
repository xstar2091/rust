use axum::{
    http::HeaderName,
    routing::{get},
    Router,
};
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
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

    let request_id_header = HeaderName::from_static("x-request-id");
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(|request: &axum::http::Request<_>| {
            // 从请求扩展中取出 SetRequestIdLayer 生成的 ID
            let request_id = request
                .extensions()
                .get::<tower_http::request_id::RequestId>()
                .map(|id| id.header_value().to_str().unwrap_or("unknown"))
                .unwrap_or("unknown");

            // 创建一个包含 request_id 的 span，后续日志都会自动带上它
            tracing::info_span!(
            "request",
            method = %request.method(),
            uri = %request.uri(),
            version = ?request.version(),
            request_id = %request_id, // 关键：唯一标识
        )
    });
    let state = AppState::new("./data.toml");
    let app = Router::new()
        .route("/health", get(health))
        // /orders/{id}?limit={limit}&offset={offset}
        .route("/orders/{id}", get(get_order))
        // /orders?limit={limit}&offset={offset}
        .route("/orders", get(get_order_list))
        .with_state(state)
        .layer(trace_layer)
        .layer(PropagateRequestIdLayer::new(request_id_header.clone()))
        .layer(SetRequestIdLayer::new(request_id_header, MakeRequestUuid));
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
