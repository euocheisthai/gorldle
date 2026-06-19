# gorldle

wippppp

### api

* `GET /api/ping` — healthcheck
* `GET /api/randomize` — regenerate the answer id for a new game
* `GET /api/load_profile` — to load/refresh profile (todo - make profile id a query parameter, id 1 is hardcoded atm) and re-pick the answer
* `GET /api/items` — full character list
* `GET /api/profile_item?id=N` — retrieve info about a specific profile entry
* `GET /api/guess_item?id=N` — check if your guess id equals to the current correct answer id

### layout

* back - axum api on `:8080` serving `profile_1.json`
* front - leptos CSR compiled with trunk on `:3000`; trunk proxies `/api/*` to the backend

### requirements

```sh
rustup target add wasm32-unknown-unknown
cargo install trunk
cargo install wasm-bindgen-cli --version 0.2.100
```

### running locally

backend:
```sh
cargo run -p back
```

front:
```sh
cd front && trunk serve
```

then open localhost:3000

### todo

* deploy in [lab](https://github.com/euocheisthai/lab)
* add an actually pretty ui
* [über important] proper ci
* neighbor admin app for managing/building profiles