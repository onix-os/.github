set shell := ["sh", "-cu"]

build:
    cargo run --release

check:
    cargo fmt --check
    cargo clippy --all-targets -- -D warnings
    cargo test

serve: build
    miniserve dist --index index.html
