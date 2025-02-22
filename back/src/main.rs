use std::{fs, sync::Arc};
use rand::Rng;

use serde;
use serde_json::Value;
use assert_json_diff::{assert_json_include, assert_json_eq};

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Html,
    routing::{get, post},
    Json, Router,
};
use tokio;
use tokio::sync::RwLock;
mod dota;
use dota::DotaEntry;

mod profile;
use profile::EntryId;

type SharedState = Arc<RwLock<Json<Value>>>;

#[tokio::main(flavor = "multi_thread", worker_threads = 5)]
async fn main() {
    tracing_subscriber::fmt::init();

    let profile_data: Json<Value> = load_profile(1).await;
    let shared_state: SharedState = Arc::new(RwLock::new(profile_data));

    let app = Router::new()
        .route("/api/ping", get(healthcheck))
        .route("/api/load_profile", get(load_profile_handler))
        .route("/api/profile_item", get(get_profile_item)).with_state(shared_state)
        .route("/api/guess_item", get(guess_profile_item))
        .route("/api/randomize", get(randomize_answer));
        // /api/refresh - to reload these 4 numbers

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn healthcheck() -> (StatusCode, Json<String>) {
    return (StatusCode::OK, Json("okayeg".to_string()));
}

async fn load_profile_handler(State(shared_state): State<SharedState>) -> Json<Value> {
    let new_profile: Json<Value> = load_profile(1).await;

    let mut profile_lock: tokio::sync::RwLockWriteGuard<'_, Json<Value>> = shared_state.write().await;
    *profile_lock = new_profile.clone();

    return new_profile;
}

async fn load_profile(profile_id: u8) -> Json<Value> {
    let profile_path: &str = &format!("profile_{}.json", profile_id);
    let profile: String =
        fs::read_to_string(profile_path).expect("Did you move the required config somewhere?");
    let current_profile: Value = serde_json::from_str(&profile).expect("That's no JSON");

    if let Value::Array(items) = &current_profile["items"] {
        for item in items {
            match serde_json::from_value::<DotaEntry>(item.clone()) {
                Ok(dota_entry) => println!("Loaded: {:?}", dota_entry),
                Err(e) => eprintln!("Failed to parse entry: {}", e),
            }
        }
    }

    return axum::Json(current_profile);
}

// /api/profile_item/?id=2
async fn get_profile_item(
    entry_id: Query<profile::EntryId>,
    State(shared_state): State<SharedState>,
) -> Json<Value> {
    let profile_lock: tokio::sync::RwLockReadGuard<'_, Json<Value>> = shared_state.read().await;

    if let Value::Array(items) = &profile_lock["items"] {
        if let Some(entry) = items.iter().find(|e: &&Value| e["id"] == entry_id.id) {
            return Json(entry.clone());
        }
    }
    Json(serde_json::json!({"error": "Entry not found"}))
}

// /api/guess_item?id=2
async fn guess_profile_item(entry_id: Query<profile::EntryId>) {
    // received entry id
    // call randomize_answer 
    // call get_profile_item to get info about player item id
    // call get_profile_item to get info about the randomized id 
    // compate the 2 items
}

// should only be called once on start, but ill handle that later
// as of now its randomized on every attempt lol
async fn randomize_answer() -> Json<Value> {
    let mut rng = rand::rng();
    let random_num: u8 = rng.random_range(0..3);
    let data = serde_json::json!({
        "id": format!("{}", random_num)
    });
    let answer: Value = serde_json::from_value(data).unwrap();
    return axum::Json(answer)
}