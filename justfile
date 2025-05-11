set positional-arguments

test *args:
    bacon test
    # cargo test --all-features -- --exact twiml::responses::tests::test_say_with_ssml

# watch-test *args:
#     cargo test --all-features -- --exact "$@"

fix:
    cargo clippy --fix --all-features --allow-dirty
    cargo fmt --all -- --check

check:
    cargo check --all-features
