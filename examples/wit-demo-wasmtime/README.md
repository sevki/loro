# Loro WIT Collaborative Editing Demo with Wasmtime

This example demonstrates **collaborative editing** using Loro CRDTs through the WebAssembly Component Model with [Wasmtime](https://wasmtime.dev/) as the host runtime.

The demo simulates two peers (Alice and Bob) making concurrent edits to a shared document and syncing their changes, demonstrating CRDT convergence.

## Prerequisites

- Rust with `wasm32-wasip1` target
- `cargo-component` (`cargo install cargo-component`)

## Setup

1. Build the Loro WIT component:
   ```bash
   cd ../../crates/loro-wit
   cargo component build --release
   ```

2. Build and run the demo:
   ```bash
   cargo run
   ```

   Or specify a custom component path:
   ```bash
   cargo run -- /path/to/loro_wit.wasm
   ```

## What the Demo Shows

1. **Two peers created**: Alice (peer 1) and Bob (peer 2)
2. **Initial sync**: Alice creates a document, syncs to Bob
3. **Concurrent edits**: Both peers edit simultaneously without seeing each other's changes
4. **Bidirectional sync**: Updates are exchanged between peers
5. **Convergence verification**: Both documents end up with the same state

## How it works

1. The `loro-wit` crate defines a WIT (WebAssembly Interface Types) interface for Loro CRDTs
2. `cargo component build` compiles the Rust code to a WebAssembly Component
3. The Wasmtime host uses `wasmtime::component::bindgen!` to generate Rust bindings from the WIT
4. The host creates two LoroDoc instances representing different peers
5. Peers exchange updates via `export_updates()` and `import_bytes()` to sync

## Key Wasmtime APIs Used

- `Component::from_file` - Load a WebAssembly Component
- `wasmtime::component::bindgen!` - Generate host bindings from WIT
- `Linker` - Link WASI and other imports
- `Store` - Manage WebAssembly state
- Resource handles for Loro containers
