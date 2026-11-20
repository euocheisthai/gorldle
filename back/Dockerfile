ARG RUST_VERSION=1.87.0
FROM rust:${RUST_VERSION}-slim-bookworm AS build
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY front/Cargo.toml front/Cargo.toml
COPY front/src/main.rs front/src/main.rs
COPY back/Cargo.toml back/profile_1.json back/
COPY back/src/ back/src/
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --locked --release -p back && cp target/release/back /bin/gorldle-back

FROM debian:bookworm-slim
COPY --from=build /bin/gorldle-back /bin/gorldle-back
ENV APP_LISTEN_ADDR=0.0.0.0:8080
EXPOSE 8080
CMD ["/bin/gorldle-back"]
