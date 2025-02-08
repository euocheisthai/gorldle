wip

* `curl -v localhost:8080/ping` - healthcheck
* `curl -v localhost:8080/api/load_profile` - to load/refresh profile (todo - make profile id a query parameter, id 1 is hardcoded atm)
* `curl -v "localhost:8080/api/profile?id=2"` - retrieve info about a specific profile entry


to consider:
* https://docs.rs/aide/0.10.0/aide/axum/index.html
