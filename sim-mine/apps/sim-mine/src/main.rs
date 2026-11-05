use axum::{
    http::HeaderName,
    routing::get,
    Router,
};
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use sim_mine_core::config::Config;
use sim_mine_core::http::{
    health_handler::health,
    http_common::HttpState,
    project_handler::*,
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Arc::new(Config::load());

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,tower_http=debug".into()),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_file(true)
                .with_line_number(true),
        )
        .init();

    let request_id_header = HeaderName::from_static("http-request-id");
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
                http_request_id = %request_id, // 关键：唯一标识
            )
        });

    let state = HttpState::load(Arc::clone(&config)).await?;

    let app = Router::new()
        .route("/v1/health", get(health))
        .route("/v1/{user_id}/project/list", get(project_list_get))
        .route("/v1/param", get(project_param_default_get))
        .route("/v1/carparam", get(project_car_param_default_get))
        .layer(trace_layer)
        .layer(PropagateRequestIdLayer::new(request_id_header.clone()))
        .layer(SetRequestIdLayer::new(request_id_header, MakeRequestUuid))
        .with_state(state);
    let listen_address = format!("0.0.0.0:{}", config.server().port());
    let listener = match tokio::net::TcpListener::bind(&listen_address).await {
        Ok(listener) => listener,
        Err(err) => {
            tracing::error!(
                address = %listen_address,
                error = %err,
                "http server failed to listen"
            );
            return Err(err.into());
        }
    };
    tracing::info!("server starting on port {}", config.server().port());
    if let Err(err) = axum::serve(listener, app).await {
        tracing::error!(error = %err, "http server failed");
        return Err(err.into());
    }
    tracing::info!("server stopped");
    Ok(())
}
