# Loro WIT Demo with Wasmtime

This example demonstrates using Loro CRDTs through the WebAssembly Component Model with [Wasmtime](https://wasmtime.dev/) as the host runtime.

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

## How it works

1. The `loro-wit` crate defines a WIT (WebAssembly Interface Types) interface for Loro CRDTs
2. `cargo component build` compiles the Rust code to a WebAssembly Component
3. The Wasmtime host uses `wasmtime::component::bindgen!` to generate Rust bindings from the WIT
4. The host instantiates and calls the component to work with Loro documents

## Code Structure

The demo showcases:

- Creating a Loro document
- Working with text containers (insert, delete, read)
- Working with map containers (insert typed values, read)
- Working with list containers (push, read)
- Exporting and importing document snapshots
- Forking documents

## Key Wasmtime APIs Used

- `Component::from_file` - Load a WebAssembly Component
- `wasmtime::component::bindgen!` - Generate host bindings from WIT
- `Linker` - Link WASI and other imports
- `Store` - Manage WebAssembly state
- Resource handles for Loro containers
