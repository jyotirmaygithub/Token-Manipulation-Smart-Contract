
cargo build --release --target wasm32-unknown-unknown --package token-flow-backend

# notes_keeper_backend.wasm ---->>  the name of the wasm file should use underscroes not hifens.
candid-extractor target/wasm32-unknown-unknown/release/token_flow_backend.wasm > src/token-flow-backend/token-flow-backend.did

dfx deploy token-flow-backend
