# Loro WIT Demo with JCO

This example demonstrates using Loro CRDTs through the WebAssembly Component Model with [JCO](https://github.com/bytecodealliance/jco) (JavaScript Component Tools).

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

## How it works

1. The `loro-wit` crate defines a WIT (WebAssembly Interface Types) interface for Loro CRDTs
2. `cargo component build` compiles the Rust code to a WebAssembly Component
3. `jco transpile` generates JavaScript/TypeScript bindings from the component
4. The demo script imports and uses these bindings to work with Loro documents

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
- `importBytes(data)` - Import data
- `fork()` - Create a fork with new peer ID

### LoroText
- `insert(pos, text)` - Insert text
- `delete(pos, len)` - Delete text
- `toString()` - Get content as string
- `lenUnicode()` - Get length in unicode chars
- `isEmpty()` - Check if empty

### LoroMap
- `insertString(key, value)` - Insert string
- `insertI64(key, value)` - Insert integer
- `insertBool(key, value)` - Insert boolean
- `delete(key)` - Delete key
- `getJson(key)` - Get value as JSON
- `keys()` - Get all keys
- `getDeepValueJson()` - Get full map as JSON

### LoroList
- `insertString(pos, value)` - Insert string at position
- `pushString(value)` - Push string to end
- `pushI64(value)` - Push integer to end
- `delete(pos, len)` - Delete items
- `getJson(index)` - Get item as JSON
- `len()` - Get length
- `getDeepValueJson()` - Get full list as JSON
