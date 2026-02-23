use axum::{extract::State, response::Json, routing::{get, post, delete}, Router};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

struct AppState { start_time: Instant, stats: Mutex<Stats> }
struct Stats { total_pushes: u64, total_purges: u64, total_queries: u64, bytes_served: u64 }

#[derive(Serialize)]
struct Health { status: String, version: String, uptime_secs: u64, total_ops: u64 }

#[derive(Deserialize)]
struct PushRequest { path: String, content_type: Option<String>, ttl_secs: Option<u64>, regions: Option<Vec<String>> }
#[derive(Serialize)]
struct PushResponse { asset_id: String, path: String, status: String, regions_deployed: Vec<String>, ttl_secs: u64, edge_url: String }

#[derive(Deserialize)]
struct PurgeRequest { paths: Vec<String>, regions: Option<Vec<String>> }
#[derive(Serialize)]
struct PurgeResponse { status: String, paths_purged: usize, regions_affected: Vec<String> }

#[derive(Deserialize)]
struct LatencyRequest { origin: String, destinations: Option<Vec<String>> }
#[derive(Serialize)]
struct LatencyResponse { origin: String, measurements: Vec<LatencyMeasurement> }
#[derive(Serialize)]
struct LatencyMeasurement { region: String, latency_ms: f64, vivaldi_coords: [f64; 3] }

#[derive(Serialize)]
struct EdgeNode { region: String, location: String, capacity_gbps: f64, current_load_pct: f64, vivaldi_coords: [f64; 3] }

#[derive(Serialize)]
struct StatsResponse { total_pushes: u64, total_purges: u64, total_queries: u64, bytes_served: u64, cache_hit_rate: f64 }

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_env_filter(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "cdn_engine=info".into())).init();
    let state = Arc::new(AppState { start_time: Instant::now(), stats: Mutex::new(Stats { total_pushes: 0, total_purges: 0, total_queries: 0, bytes_served: 0 }) });
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);
    let app = Router::new()
        .route("/health", get(health))
        .route("/api/v1/cdn/push", post(push))
        .route("/api/v1/cdn/purge", delete(purge))
        .route("/api/v1/cdn/latency", post(latency))
        .route("/api/v1/cdn/edges", get(edges))
        .route("/api/v1/cdn/stats", get(stats))
        .layer(cors).layer(TraceLayer::new_for_http()).with_state(state);
    let addr = std::env::var("CDN_ADDR").unwrap_or_else(|_| "0.0.0.0:8081".into());
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("CDN Engine on {addr}");
    axum::serve(listener, app).await.unwrap();
}

async fn health(State(s): State<Arc<AppState>>) -> Json<Health> {
    let st = s.stats.lock().unwrap();
    Json(Health { status: "ok".into(), version: env!("CARGO_PKG_VERSION").into(), uptime_secs: s.start_time.elapsed().as_secs(), total_ops: st.total_pushes + st.total_purges + st.total_queries })
}

async fn push(State(s): State<Arc<AppState>>, Json(req): Json<PushRequest>) -> Json<PushResponse> {
    let regions = req.regions.unwrap_or_else(|| vec!["us-east".into(), "eu-west".into(), "ap-northeast".into()]);
    let ttl = req.ttl_secs.unwrap_or(86400);
    s.stats.lock().unwrap().total_pushes += 1;
    Json(PushResponse {
        asset_id: uuid::Uuid::new_v4().to_string(), path: req.path.clone(), status: "deployed".into(),
        regions_deployed: regions, ttl_secs: ttl,
        edge_url: format!("https://cdn.alicelaw.net{}", req.path),
    })
}

async fn purge(State(s): State<Arc<AppState>>, Json(req): Json<PurgeRequest>) -> Json<PurgeResponse> {
    let count = req.paths.len();
    let regions = req.regions.unwrap_or_else(|| vec!["us-east".into(), "eu-west".into(), "ap-northeast".into()]);
    s.stats.lock().unwrap().total_purges += 1;
    Json(PurgeResponse { status: "purged".into(), paths_purged: count, regions_affected: regions })
}

async fn latency(State(s): State<Arc<AppState>>, Json(req): Json<LatencyRequest>) -> Json<LatencyResponse> {
    let dests = req.destinations.unwrap_or_else(|| vec!["us-east".into(), "eu-west".into(), "ap-northeast".into(), "ap-southeast".into()]);
    let h = fnv1a(req.origin.as_bytes());
    let measurements: Vec<LatencyMeasurement> = dests.iter().enumerate().map(|(i, r)| {
        let base = ((h.wrapping_add(i as u64 * 37)) % 200) as f64;
        LatencyMeasurement { region: r.clone(), latency_ms: base + 5.0, vivaldi_coords: [base * 0.1, (i as f64) * 0.3, 0.5] }
    }).collect();
    s.stats.lock().unwrap().total_queries += 1;
    Json(LatencyResponse { origin: req.origin, measurements })
}

async fn edges() -> Json<Vec<EdgeNode>> {
    Json(vec![
        EdgeNode { region: "us-east".into(), location: "Virginia, US".into(), capacity_gbps: 100.0, current_load_pct: 42.0, vivaldi_coords: [1.2, 0.5, 0.3] },
        EdgeNode { region: "eu-west".into(), location: "Frankfurt, DE".into(), capacity_gbps: 80.0, current_load_pct: 35.0, vivaldi_coords: [0.8, 1.1, 0.2] },
        EdgeNode { region: "ap-northeast".into(), location: "Tokyo, JP".into(), capacity_gbps: 60.0, current_load_pct: 55.0, vivaldi_coords: [2.0, 1.8, 0.4] },
        EdgeNode { region: "ap-southeast".into(), location: "Singapore".into(), capacity_gbps: 40.0, current_load_pct: 28.0, vivaldi_coords: [1.5, 1.5, 0.6] },
    ])
}

async fn stats(State(s): State<Arc<AppState>>) -> Json<StatsResponse> {
    let st = s.stats.lock().unwrap();
    let total = st.total_queries + st.total_pushes;
    Json(StatsResponse { total_pushes: st.total_pushes, total_purges: st.total_purges, total_queries: st.total_queries, bytes_served: st.bytes_served, cache_hit_rate: if total > 0 { 0.92 } else { 0.0 } })
}

fn fnv1a(data: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &b in data { h ^= b as u64; h = h.wrapping_mul(0x0100_0000_01b3); }
    h
}
