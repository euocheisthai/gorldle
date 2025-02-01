use std::fs;
use serde_json::Value;

use tokio;
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};

mod dota;
use dota::DotaEntry;


async fn root() -> &'static str {
    "TBA!"
}

async fn healthcheck() -> (StatusCode, Json<String>) {
    let ok_response: String = String::from("okayeg");

    return (StatusCode::OK, Json(ok_response))
}

async fn load_profile(profile_path: &str) -> Json<Value> {
    let profile: String = fs::read_to_string(profile_path)
        .expect("Did you move the config somewhere?");

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

    return Json(current_profile)
}

#[tokio::main(flavor = "multi_thread", worker_threads = 5)]
async fn main() {

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/ping", get(healthcheck))
        .route("/profile", get(|| async { load_profile("profile_1.json").await }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}
