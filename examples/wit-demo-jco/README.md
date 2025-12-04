# Loro WIT Collaborative Editing Demo with JCO

This example demonstrates **collaborative editing** using Loro CRDTs through the WebAssembly Component Model with [JCO](https://github.com/bytecodealliance/jco) (JavaScript Component Tools).

The demo simulates two peers (Alice and Bob) making concurrent edits to a shared document and syncing their changes, demonstrating CRDT convergence.

## Prerequisites

- Node.js 18+
- Rust with `wasm32-wasip1` target
- `cargo-component` (`cargo install cargo-component`)
- `jco` (`npm install -g @bytecodealliance/jco`)

## Setup

1. Install dependencies:
   ```bash
   npm install
   ```

2. Build the component and generate JavaScript bindings:
   ```bash
   npm run build
   ```

3. Run the demo:
   ```bash
   npm run demo
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
3. `jco transpile` generates JavaScript/TypeScript bindings from the component
4. The demo script creates two LoroDoc instances representing different peers
5. Peers exchange updates via `exportUpdates()` and `importBytes()` to sync

## Available Operations

### LoroDoc
- `new()` - Create a new document
- `peerId()` - Get the document's peer ID
- `setPeerId(id)` - Set the peer ID
- `getText(name)` - Get a text container
- `getMap(name)` - Get a map container
- `getList(name)` - Get a list container
- `commit()` - Commit pending changes
- `exportSnapshot()` - Export document as snapshot
- `exportUpdates()` - Export all updates
- `importBytes(data)` - Import data from another peer
- `fork()` - Create a fork with new peer ID

### LoroText
- `insert(pos, text)` - Insert text
- `delete(pos, len)` - Delete text
- `toString()` - Get content as string

### LoroMap
- `insertString(key, value)` - Insert string
- `insertI64(key, value)` - Insert integer
- `delete(key)` - Delete key
- `keys()` - Get all keys

### LoroList
- `pushString(value)` - Push string to end
- `pushI64(value)` - Push integer to end
- `delete(pos, len)` - Delete items
- `len()` - Get length
