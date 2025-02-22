wip

* `curl -v localhost:8080/ping` - healthcheck
* `curl -v localhost:8080/api/load_profile` - to load/refresh profile (todo - make profile id a query parameter, id 1 is hardcoded atm)
* `curl -v "localhost:8080/api/profile_item?id=2"` - retrieve info about a specific profile entry
* `curl -v "localhost:8080/api/guess_item?id=2"` - check if your guess id equals to the current correct answer id
* `curl -v "localhost:8080/api/randomize"` - regenerate the answer id for a new game 


to consider:
* https://docs.rs/aide/0.10.0/aide/axum/index.html
