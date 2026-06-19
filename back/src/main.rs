use std::collections::HashSet;
use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use rand::Rng;
use serde_json::Value;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};

mod dota;
use dota::DotaEntry;
mod profile;
use profile::{Correctness, EntryId, FieldComparison, GuessResponse};

const EMBEDDED_PROFILE: &str = include_str!("../profile_1.json");

struct AppState {
    profile: RwLock<Value>,
    answer_id: RwLock<u8>,
}

type SharedState = Arc<AppState>;

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    tracing_subscriber::fmt::init();

    let profile_data: Value = parse_profile(EMBEDDED_PROFILE);
    let answer_id = pick_answer_id(&profile_data);

    let shared_state = Arc::new(AppState {
        profile: RwLock::new(profile_data),
        answer_id: RwLock::new(answer_id),
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/ping", get(healthcheck))
        .route("/api/items", get(list_items))
        .route("/api/load_profile", get(load_profile_handler))
        .route("/api/profile_item", get(get_profile_item))
        .route("/api/guess_item", get(guess_profile_item))
        .route("/api/randomize", get(randomize_answer))
        .with_state(shared_state)
        .layer(cors);

    let addr = "127.0.0.1:8080";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("listening on http://{}", addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

fn parse_profile(raw: &str) -> Value {
    let value: Value = serde_json::from_str(raw).expect("profile_1.json is not valid JSON");
    if let Value::Array(items) = &value["items"] {
        for item in items {
            if let Err(e) = serde_json::from_value::<DotaEntry>(item.clone()) {
                tracing::warn!("failed to parse entry as DotaEntry: {e}");
            }
        }
    }
    value
}

fn pick_answer_id(profile: &Value) -> u8 {
    let ids: Vec<u8> = profile["items"]
        .as_array()
        .map(|items| {
            items
                .iter()
                .filter_map(|item| item["id"].as_u64().map(|n| n as u8))
                .collect()
        })
        .unwrap_or_default();
    let idx = rand::rng().random_range(0..ids.len().max(1));
    ids.get(idx).copied().unwrap_or(1)
}

async fn healthcheck() -> (StatusCode, Json<&'static str>) {
    (StatusCode::OK, Json("okayeg"))
}

async fn list_items(State(state): State<SharedState>) -> Json<Value> {
    let profile = state.profile.read().await;
    Json(profile["items"].clone())
}

async fn load_profile_handler(State(state): State<SharedState>) -> Json<Value> {
    let new_profile = parse_profile(EMBEDDED_PROFILE);
    *state.profile.write().await = new_profile.clone();
    *state.answer_id.write().await = pick_answer_id(&new_profile);
    Json(new_profile)
}

async fn get_profile_item(
    Query(entry_id): Query<EntryId>,
    State(state): State<SharedState>,
) -> Json<Value> {
    let profile = state.profile.read().await;
    if let Value::Array(items) = &profile["items"] {
        if let Some(entry) = items.iter().find(|e| e["id"] == entry_id.id) {
            return Json(entry.clone());
        }
    }
    Json(serde_json::json!({ "error": "Entry not found" }))
}

async fn guess_profile_item(
    Query(entry_id): Query<EntryId>,
    State(state): State<SharedState>,
) -> Result<Json<GuessResponse>, (StatusCode, String)> {
    let profile = state.profile.read().await;
    let answer_id = *state.answer_id.read().await;

    let items = profile["items"]
        .as_array()
        .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "no items".into()))?;

    let lookup = |id: u64| -> Result<DotaEntry, (StatusCode, String)> {
        items
            .iter()
            .find(|e| e["id"] == id)
            .cloned()
            .ok_or((StatusCode::NOT_FOUND, format!("no entry for id {id}")))
            .and_then(|v| {
                serde_json::from_value::<DotaEntry>(v)
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
            })
    };

    let player_item = lookup(entry_id.id as u64)?;
    let answer_item = lookup(answer_id as u64)?;

    let mut fields = Vec::new();
    for (key, player_value) in &player_item {
        let correctness = match answer_item.get(key) {
            Some(answer_value) if player_value == answer_value => Correctness::Correct,
            Some(answer_value) if key == "position" => {
                match (&player_value, &answer_value) {
                    (Value::Array(p), Value::Array(a)) => check_partial_correctness(p, a),
                    _ => Correctness::Incorrect,
                }
            }
            _ => Correctness::Incorrect,
        };
        fields.push(FieldComparison {
            field: key.to_string(),
            value: player_value,
            correct: correctness,
        });
    }

    Ok(Json(GuessResponse {
        name: player_item.name.as_str().unwrap_or("").to_string(),
        fields,
    }))
}

fn check_partial_correctness(player_list: &[Value], answer_list: &[Value]) -> Correctness {
    let player_set: HashSet<_> = player_list.iter().collect();
    let answer_set: HashSet<_> = answer_list.iter().collect();
    if player_set == answer_set {
        Correctness::Correct
    } else if !player_set.is_disjoint(&answer_set) {
        Correctness::PartiallyCorrect
    } else {
        Correctness::Incorrect
    }
}

async fn randomize_answer(State(state): State<SharedState>) -> Json<Value> {
    let profile = state.profile.read().await;
    let new_id = pick_answer_id(&profile);
    *state.answer_id.write().await = new_id;
    Json(serde_json::json!({ "id": new_id }))
}
