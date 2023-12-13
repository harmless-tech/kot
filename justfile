default:
    just -l

pwd := `pwd`

run +ARGS:
    cargo run -- {{ARGS}}

runr +ARGS:
    cargo run --release -- {{ARGS}}

fmt:
    cargo +nightly fmt

lint:
    cargo clippy --all-targets --locked --workspace -- -D warnings

check:
    cargo +nightly fmt --check
    just lint
    cargo clippy --all-targets --locked --workspace --release -- -D warnings


docker:
    docker run -it --rm --pull=always \
    -e CARGO_TARGET_DIR=/ptarget \
    --mount type=bind,source={{pwd}},target=/project \
    --mount type=bind,source=$HOME/.cargo/registry,target=/usr/local/cargo/registry \
    -w /project \
    rust:latest \
    bash

docker-alpine:
    docker run -it --rm --pull=always \
    -e CARGO_TARGET_DIR=/ptarget \
    --mount type=bind,source={{pwd}},target=/project \
    --mount type=bind,source=$HOME/.cargo/registry,target=/usr/local/cargo/registry \
    -w /project \
    rust:alpine \
    sh

doco:
    cargo doc --open --document-private-items

doc:
    cargo doc --document-private-items

hack:
    docker run -t --rm --pull=always \
    -e CARGO_TARGET_DIR=/ptarget \
    --mount type=bind,source={{pwd}},target=/project \
    --mount type=bind,source=$HOME/.cargo/registry,target=/usr/local/cargo/registry \
    -w /project \
    rust:latest \
    bash -c "curl --proto '=https' --tlsv1.2 -sSf \
    https://raw.githubusercontent.com/cargo-prebuilt/cargo-prebuilt/main/scripts/install-cargo-prebuilt.sh | bash \
    && cargo prebuilt cargo-hack --ci \
    && cargo hack check --each-feature --no-dev-deps --verbose --workspace \
    && cargo hack check --feature-powerset --no-dev-deps --verbose --workspace"

msrv:
    docker run -t --rm --pull=always \
    -e CARGO_TARGET_DIR=/ptarget \
    --mount type=bind,source={{pwd}},target=/project \
    --mount type=bind,source=$HOME/.cargo/registry,target=/usr/local/cargo/registry \
    -w /project \
    rust:latest \
    bash -c 'cargo install cargo-msrv --version 0.15.1 --profile=dev && cargo msrv -- cargo check --verbose --locked'
