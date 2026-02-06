set fallback := true
set unstable := true

default: run

run:
    clear && RUSTFLAGS="-Awarnings" cargo run --bin netero-dev --quiet -- -v -p ollama  "escribe un poema"

install:
    cargo install --path . --bin netero-dev
