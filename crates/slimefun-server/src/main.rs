use axum::{
    extract::Json,
    routing::get,
    Router,
};
use slimefun_addons::SlimefunAddonsEngine;
use slimefun_core::{BlockStorageEngine, SlimefunItemRegistry};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::info;

struct AppState {
    storage: BlockStorageEngine,
    items: SlimefunItemRegistry,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = Arc::new(AppState {
        storage: BlockStorageEngine::new(),
        items: SlimefunItemRegistry::new(),
    });

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/slimefun/stats", get(get_slimefun_stats))
        .route("/api/slimefun/addons", get(get_active_addons))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8085));
    info!("⚡ Slimefun Unified Engine Server (Rust) corriendo en http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "OK - Slimefun Rust Monorepo Engine Fully Loaded"
}

async fn get_slimefun_stats(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "total_stored_blocks": state.storage.count(),
        "registered_items": state.items.count(),
        "engine": "Rust 2021 Workspace Monorepo"
    }))
}

async fn get_active_addons() -> Json<serde_json::Value> {
    let addons = SlimefunAddonsEngine::get_active_addons();
    Json(serde_json::json!({
        "addon_count": addons.len(),
        "addons": addons
    }))
}
