default:
    @just --list

# Run pre-commit hooks on all files, including autoformatting
pre-commit-all:
    pre-commit run --all-files

# Run 'cargo run' on the project
run *ARGS:
    cargo run {{ARGS}}

# Run 'bacon' to run the project (auto-recompiles)
watch *ARGS:
	bacon --job run -- -- {{ ARGS }}

build-sdk-kotlin:
    cd packages/sdk
    cargo build --package sdk --release
    cargo run --bin uniffi-bindgen generate --library target/release/libsdk.so --language kotlin --out-dir ../../target