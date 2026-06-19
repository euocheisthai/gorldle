use gloo_net::http::Request;
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Correctness {
    Correct,
    PartiallyCorrect,
    Incorrect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldComparison {
    pub field: String,
    pub value: serde_json::Value,
    pub correct: Correctness,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuessResponse {
    pub name: String,
    pub fields: Vec<FieldComparison>,
}

#[derive(Copy, Clone)]
struct GameSignals {
    items: RwSignal<Vec<Item>>,
    input: RwSignal<String>,
    guesses: RwSignal<Vec<GuessResponse>>,
    won: RwSignal<bool>,
    error: RwSignal<Option<String>>,
}

fn submit_guess(s: GameSignals, raw_name: String) {
    let name = raw_name.trim().to_string();
    if name.is_empty() {
        return;
    }
    let target = s.items.with(|list| {
        list.iter()
            .find(|i| i.name.eq_ignore_ascii_case(&name))
            .cloned()
    });
    let Some(target) = target else {
        s.error.set(Some(format!("no character named '{name}'")));
        return;
    };
    let already_guessed = s.guesses.with(|gs| {
        gs.iter()
            .any(|g| g.name.eq_ignore_ascii_case(&target.name))
    });
    if already_guessed {
        return;
    }
    s.error.set(None);
    s.input.set(String::new());
    spawn_local(async move {
        let url = format!("/api/guess_item?id={}", target.id);
        match Request::get(&url).send().await {
            Ok(resp) => match resp.json::<GuessResponse>().await {
                Ok(g) => {
                    let all_correct = g
                        .fields
                        .iter()
                        .filter(|f| f.field != "name")
                        .all(|f| f.correct == Correctness::Correct);
                    s.guesses.update(|gs| gs.push(g));
                    if all_correct {
                        s.won.set(true);
                    }
                }
                Err(e) => s.error.set(Some(format!("parse error: {e}"))),
            },
            Err(e) => s.error.set(Some(format!("fetch error: {e}"))),
        }
    });
}

#[component]
pub fn App() -> impl IntoView {
    let s = GameSignals {
        items: RwSignal::new(Vec::new()),
        input: RwSignal::new(String::new()),
        guesses: RwSignal::new(Vec::new()),
        won: RwSignal::new(false),
        error: RwSignal::new(None),
    };

    spawn_local(async move {
        match Request::get("/api/items").send().await {
            Ok(resp) => match resp.json::<Vec<Item>>().await {
                Ok(list) => s.items.set(list),
                Err(e) => s.error.set(Some(format!("parse error: {e}"))),
            },
            Err(e) => s.error.set(Some(format!("fetch error: {e}"))),
        }
    });

    let reset = move |_| {
        spawn_local(async move {
            if let Err(e) = Request::get("/api/randomize").send().await {
                log::error!("randomize failed: {e}");
            }
            s.guesses.set(Vec::new());
            s.won.set(false);
            s.error.set(None);
            s.input.set(String::new());
        });
    };

    let suggestions = move || {
        let q = s.input.get().to_lowercase();
        if q.is_empty() {
            return Vec::<Item>::new();
        }
        let already: std::collections::HashSet<String> =
            s.guesses.with(|gs| gs.iter().map(|g| g.name.to_lowercase()).collect());
        s.items.with(|list| {
            list.iter()
                .filter(|i| {
                    let name_lc = i.name.to_lowercase();
                    name_lc.contains(&q) && !already.contains(&name_lc)
                })
                .take(8)
                .cloned()
                .collect::<Vec<_>>()
        })
    };

    view! {
        <main>
            <h1>"gorldle"</h1>
            <p class="tagline">"guess the dota character of the day LMAO"</p>

            <Show when=move || !s.won.get() fallback=|| ()>
                <div class="input-row">
                    <input
                        type="text"
                        placeholder="type a character name..."
                        autocomplete="off"
                        prop:value=move || s.input.get()
                        on:input=move |ev| s.input.set(event_target_value(&ev))
                        on:keydown=move |ev: leptos::ev::KeyboardEvent| {
                            if ev.key() == "Enter" {
                                submit_guess(s, s.input.get_untracked());
                            }
                        }
                    />
                    <button on:click=move |_| submit_guess(s, s.input.get_untracked())>
                        "Guess"
                    </button>
                    {move || {
                        let list = suggestions();
                        (!list.is_empty()).then(|| view! {
                            <div class="suggestions">
                                {list.into_iter().map(|item| {
                                    let name = item.name.clone();
                                    let click_name = name.clone();
                                    view! {
                                        <div on:click=move |_| submit_guess(s, click_name.clone())>
                                            {name}
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                        })
                    }}
                </div>
            </Show>

            {move || s.error.get().map(|e| view! { <div class="error">{e}</div> })}

            <Show when=move || !s.guesses.with(Vec::is_empty) fallback=|| ()>
                <div class="header-row">
                    <div>"name"</div>
                    <div>"attribute"</div>
                    <div>"position"</div>
                    <div>"attack type"</div>
                    <div>"release year"</div>
                </div>
            </Show>

            <div class="guesses">
                {move || s.guesses.get().into_iter().rev().map(render_guess).collect_view()}
            </div>

            <Show when=move || s.won.get() fallback=|| ()>
                <div class="victory">"Waow you got it. Press 'New round' to play again."</div>
            </Show>

            <div class="controls">
                <button on:click=reset>"New round"</button>
            </div>
        </main>
    }
}

fn render_guess(guess: GuessResponse) -> impl IntoView {
    let name = guess.name.clone();
    let squares = guess
        .fields
        .into_iter()
        .filter(|f| f.field != "name")
        .map(|f| {
            let class = format!("square {}", correctness_class(&f.correct));
            let display = render_value(&f.value);
            view! { <div class=class>{display}</div> }
        })
        .collect_view();
    view! {
        <div class="guess-row">
            <div class="square name">{name}</div>
            {squares}
        </div>
    }
}

fn correctness_class(c: &Correctness) -> &'static str {
    match c {
        Correctness::Correct => "correct",
        Correctness::PartiallyCorrect => "partiallycorrect",
        Correctness::Incorrect => "incorrect",
    }
}

fn render_value(v: &serde_json::Value) -> String {
    match v {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Array(arr) => arr
            .iter()
            .map(|x| match x {
                serde_json::Value::String(s) => s.clone(),
                other => other.to_string(),
            })
            .collect::<Vec<_>>()
            .join(", "),
        _ => v.to_string(),
    }
}
