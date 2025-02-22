
use gloo_net::http::Request;
use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen_futures::spawn_local;


#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/heroguess")]
    HeroGuess,
    #[at("/get_profile_item")]
    ProfileItem,
}


fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { < HomePage /> },
        Route::HeroGuess => html! { < HeroGuess />},
        Route::ProfileItem => html! { <ProfileItem /> },
    }
}

#[function_component(HomePage)]
fn render_home_page() -> Html {
    html! {
        <div>{"background of qop (should be present everywhere), game modes on top"}</div>
    }
}

#[function_component(HeroGuess)]
fn hero_guessing_game() -> Html {
    html! {
        <>
        <div>{"search bar here - includes a list of available guesses, clicking one sends a request to /api/guess_item  with the item id"}</div>
        <div>{"at the bottom is a list of guesses that are all of type ProfileItem"}</div>
        </>
    }
}


#[function_component(ProfileItem)]
fn profile_item() -> Html {
    html! {
        <div>{"Icon + abilities list, each coloured green/yellow/red"}</div>
    }
}




fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}