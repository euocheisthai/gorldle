
use gloo_net::http::Request;
use yew::prelude::*;
use yew_router::prelude::*;


#[function_component(App)]
fn app() -> yew::Html {
    // react flashbacks
    let counter: UseStateHandle<i32> = use_state(|| 0);
    let onclick = {
        let counter: UseStateHandle<i32> = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<App>::new().render();
}




// use serde_json::Value;
// use axum::response::Html;

// #[function_component(App)]
// fn app() -> yew::Html {
//     html! {
//         <h1>{ "Hello World" }</h1>
//     }
// }

// pub fn main() {
//     yew::Renderer::<App>::new().render();
// }


// pub fn render_profile_page(profile_json: &Value) -> Html<String> {
//     let mut html_content = String::from("<h1>Dota Profile</h1><ul>");
    
//     if let Some(items) = profile_json["items"].as_array() {
//         for item in items {
//             html_content.push_str(&format!("<li>{:?}</li>", item));
//         }
//     }

//     html_content.push_str("</ul>");
//     Html(html_content)
// }
