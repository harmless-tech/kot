default:
    just -l

fmt:
    cargo +nightly fmt

check:
    cargo +nightly fmt --check
    cargo +nightly clippy --all-targets --all-features --workspace --locked -- -D warnings
    #cargo +nightly clippy --all-targets --all-features --workspace --release --locked -- -D warnings
    #cargo +nightly deny check

build:
    cargo +nightly build --locked

buildr:
    cargo +nightly build --locked --release

build-msvc:
    cargo +nightly xwin build --locked --target aarch64-pc-windows-msvc --no-default-features --features default-native

buildr-msvc:
    cargo +nightly xwin build --locked --release --target aarch64-pc-windows-msvc --no-default-features --features default-native

run +ARGS:
    cargo +nightly run --locked -- {{ARGS}}

runr +ARGS:
    cargo +nightly run --locked --release -- {{ARGS}}
