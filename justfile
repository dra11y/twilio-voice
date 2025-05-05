test:
    cargo test --all-features

fix:
    cargo clippy --fix --all-features --allow-dirty
    cargo fmt --all -- --check

check:
    cargo check --all-features
