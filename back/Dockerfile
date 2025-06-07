ARG RUST_VERSION=1.87.0
ARG APP_NAME=gorldle-back

FROM rust:${RUST_VERSION}-slim-bullseye AS build
ARG APP_NAME
WORKDIR /app
RUN apt-get update
RUN apt-get install --no-install-recommends -y pkg-config openssl libssl-dev tree

#--mount=type=bind,source=Cargo.lock,target=Cargo.lock \
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
cargo build --release
echo "searching for da binary"
tree .
cp ./target/release/$APP_NAME /bin/$APP_NAME
EOF

FROM debian:bullseye-slim AS final
ARG APP_NAME
COPY --from=build /bin/$APP_NAME /bin/
COPY ./profile_1.json /bin/profile_1.json
EXPOSE 8080
CMD ["/bin/gorldle-back"]