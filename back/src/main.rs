use std::{fs, sync::Arc};
use std::collections::HashSet;
use rand::Rng;

use serde;
use serde_json::Value;
use assert_json_diff::{assert_json_include, assert_json_eq};

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use tokio;
use tokio::sync::RwLock;

use reqwest;

mod dota;
use dota::DotaEntry;
mod profile;
use profile::{EntryId, FieldComparison, Correctness};

struct AppState {
    profile: RwLock<Json<Value>>,
    random_id: RwLock<u8>,
    client: reqwest::Client,
    base_url: String,
}

type SharedState = Arc<AppState>;

#[tokio::main(flavor = "multi_thread", worker_threads = 5)]
async fn main() {
    tracing_subscriber::fmt::init();

    let profile_data: Json<Value> = load_profile(1).await;
    let mut rng = rand::rng();
    let random_id: u8 = rng.random_range(0..3);

    let shared_state = Arc::new(AppState {
        profile: RwLock::new(profile_data),
        random_id: RwLock::new(random_id),
        client: reqwest::Client::new(),
        base_url: "http://localhost:8080/api/profile_item".to_string(),
    });

    let app = Router::new()
        .route("/api/ping", get(healthcheck))
        .route("/api/load_profile", get(load_profile_handler))
        .route("/api/profile_item", get(get_profile_item))
        .route("/api/guess_item", get(guess_profile_item))
        .route("/api/randomize", get(randomize_answer))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn healthcheck() -> (StatusCode, Json<String>) {
    return (StatusCode::OK, Json("okayeg".to_string()));
}

async fn load_profile_handler(State(shared_state): State<SharedState>) -> Json<Value> {
    let new_profile: Json<Value> = load_profile(1).await;

    let mut profile_lock: tokio::sync::RwLockWriteGuard<'_, Json<Value>> = shared_state.profile.write().await;
    *profile_lock = new_profile.clone();

    return new_profile;
}

async fn load_profile(profile_id: u8) -> Json<Value> {
    let profile_path: &str = &format!("profile_{}.json", profile_id);
    let profile: String = fs::read_to_string(profile_path).expect("Did you move the required config somewhere?");
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
    let profile_lock: tokio::sync::RwLockReadGuard<'_, Json<Value>> = shared_state.profile.read().await;

    if let Value::Array(items) = &profile_lock["items"] {
        if let Some(entry) = items.iter().find(|e: &&Value| e["id"] == entry_id.id) {
            return Json(entry.clone());
        }
    }
    
    return Json(serde_json::json!({"error": "Entry not found"}))
}

// /api/guess_item?id=2
async fn guess_profile_item(
    entry_id: Query<profile::EntryId>, 
    State(shared_state): State<SharedState>
) {
    let client: &reqwest::Client = &shared_state.client;
    let base_url: &String = &shared_state.base_url;

    let player_url: String = format!("{}?id={}", base_url, entry_id.id);
    let player_item: DotaEntry = client.get(&player_url).send().await.unwrap().json::<DotaEntry>().await.unwrap();

    let answer_id: tokio::sync::RwLockReadGuard<'_, u8> = shared_state.random_id.read().await;
    let answer_url = format!("{}?id={}", base_url, answer_id);
    let answer_item: DotaEntry = client.get(&answer_url).send().await.unwrap().json::<DotaEntry>().await.unwrap();
    
    let mut fields: Vec<FieldComparison> = Vec::new();

    if let (player_map, answer_map) = (player_item, answer_item) {
        for (key, player_value) in player_map {
            let answer_value = answer_map.get(key);
            let correctness = match answer_value {
                Some(answer_value) if player_value == answer_value => Correctness::Correct,
                Some(Value::Array(player_list)) if key == "position" => {
                    if let Some(Value::Array(answer_list)) = answer_value {
                        check_partial_correctness(&player_list, &answer_list)
                    } else {
                        Correctness::Incorrect
                    }
                }
                Some(_) => Correctness::PartiallyCorrect,
                None => Correctness::Incorrect,
            };

            fields.push(FieldComparison {
                field: key.clone(),
                value: player_value.to_string(),
                correct: correctness,
            });
        }
    }

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

// /api/randomize
#[axum::debug_handler]
async fn randomize_answer(State(shared_state): State<SharedState>) -> Json<Value> {
    let random_num = rand::rng().random_range(0..3);
    let mut random_id_lock: tokio::sync::RwLockWriteGuard<'_, u8> = shared_state.random_id.write().await;
    *random_id_lock = random_num;

    Json(serde_json::json!({ "id": random_num }))
}