use axum::{
    extract::State,
    http::{HeaderValue, StatusCode},
    routing::post,
    Json, Router,
};
use serde_json::{json, Value};
use sim_app::{Action, App};
use tower_http::cors::CorsLayer;

async fn api(State(app): State<App>, Json(action): Json<Action>) -> (StatusCode, Json<Value>) {
    match tokio::task::spawn_blocking(move || app.execute(action)).await {
        Ok(Ok(value)) => (StatusCode::OK, Json(value)),
        Ok(Err(error)) => (StatusCode::BAD_REQUEST, Json(json!({"error":error}))),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error":error.to_string()})),
        ),
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_env_filter("info").init();
    let app = App::start(std::env::current_dir()?.join("artifacts/saves"))?;
    let cors = CorsLayer::new()
        .allow_origin([
            HeaderValue::from_static("http://localhost:1420"),
            HeaderValue::from_static("http://127.0.0.1:1420"),
        ])
        .allow_methods([axum::http::Method::POST])
        .allow_headers([axum::http::header::CONTENT_TYPE]);
    let router = Router::new()
        .route("/api", post(api))
        .with_state(app)
        .layer(cors);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:1421").await?;
    tracing::info!("development_server_started: http://127.0.0.1:1421");
    axum::serve(listener, router).await?;
    Ok(())
}
