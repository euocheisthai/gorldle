use std::fs;
use serde;
use serde_json::Value;

use tokio;
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
    response::Html,
    extract::{Query, State},
};
mod dota;
use dota::DotaEntry;


#[tokio::main(flavor = "multi_thread", worker_threads = 5)]
async fn main() {

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/ping", get(healthcheck))
        .route("/api/load_profile", get(|| async { load_profile(1).await }))
        .route("/api/profile", get(get_profile));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}


async fn healthcheck() -> (StatusCode, Json<String>) {
    let ok_response: String = String::from("okayeg");

    return (StatusCode::OK, Json(ok_response))
}

async fn load_profile(profile_id: i8) -> Json<Value> {

    let profile_path: &str = &format!("profile_{}.json", profile_id);
    
    let profile: String = fs::read_to_string(profile_path)
        .expect("Did you move the required config somewhere?");

    let current_profile: Value = serde_json::from_str(&profile)
        .expect("That's no JSON");

    let _profile_id: &Value = &current_profile["profile_id"];

    if let Value::Array(items) = &current_profile["items"] {
        for item in items {
            match serde_json::from_value::<DotaEntry>(item.clone()) {
                Ok(dota_entry) => println!("Loaded: {:?}", dota_entry),
                Err(e) => eprintln!("Failed to parse entry: {}", e),
            }
        }
    }

    return axum::Json(current_profile)
}

#[derive(serde::Deserialize)]
struct Entry_ID {
    id: i16,
}

// /api/profile/?id=2
async fn get_profile(entry_id: Query<Entry_ID>) {
    let entry_id: Entry_ID = entry_id.0;
    // return the whole profile entry ig...

}
