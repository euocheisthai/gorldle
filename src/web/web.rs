// use yew::prelude::*;
// use yew_router::prelude::*;
// use serde_json::Value;
// use axum::response::Html;

use yew::prelude::*;
use serde_json::Value;
use axum::response::Html;

#[function_component(App)]
fn app() -> yew::Html {
    html! {
        <h1>{ "Hello World" }</h1>
    }
}

pub fn web_main_render() {
    yew::Renderer::<App>::new().render();
}


pub fn render_profile_page(profile_json: &Value) -> Html<String> {
    let mut html_content = String::from("<h1>Dota Profile</h1><ul>");
    
    if let Some(items) = profile_json["items"].as_array() {
        for item in items {
            html_content.push_str(&format!("<li>{:?}</li>", item));
        }
    }

    html_content.push_str("</ul>");
    Html(html_content)
}
