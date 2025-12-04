//! Loro WIT Demo with Wasmtime
//!
//! This demonstrates using Loro CRDTs through the WebAssembly Component Model
//! with Wasmtime as the host runtime.
//!
//! Note: This demo requires wasmtime with component-model support and proper
//! WASI bindings. The bindings are generated from the WIT interface.

use anyhow::Result;
use std::path::PathBuf;
use wasmtime::component::{Component, Linker};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView};

// Generate bindings from the WIT interface
wasmtime::component::bindgen!({
    path: "../../crates/loro-wit/wit",
    world: "loro",
});

// State for the WASI context
struct HostState {
    ctx: WasiCtx,
    table: wasmtime::component::ResourceTable,
}

impl WasiView for HostState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
    fn table(&mut self) -> &mut wasmtime::component::ResourceTable {
        &mut self.table
    }
}

fn main() -> Result<()> {
    println!("=== Loro WIT Demo with Wasmtime ===\n");

    // Create engine with component model support
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;

    // Get component path from args or use default
    let component_path: PathBuf = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../target/wasm32-wasip1/release/loro_wit.wasm")
        });

    println!("Loading component from: {}", component_path.display());
    
    if !component_path.exists() {
        eprintln!("Error: Component file not found!");
        eprintln!("Please build the component first:");
        eprintln!("  cd ../../crates/loro-wit && cargo component build --release");
        return Ok(());
    }

    let component = Component::from_file(&engine, &component_path)?;

    // Create linker with WASI
    let mut linker: Linker<HostState> = Linker::new(&engine);
    wasmtime_wasi::add_to_linker_sync(&mut linker)?;

    // Create store with WASI context
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_env()
        .build();
    
    let mut store = Store::new(&engine, HostState { 
        ctx: wasi, 
        table: wasmtime::component::ResourceTable::new(),
    });

    // Instantiate the component
    let instance = Loro::instantiate(&mut store, &component, &linker)?;

    // Access the doc interface
    let doc_iface = instance.component_loro_wit_doc();

    // Create a new document
    println!("Creating new LoroDoc...");
    let doc = doc_iface.loro_doc().call_constructor(&mut store)?;

    let peer_id = doc_iface.loro_doc().call_peer_id(&mut store, doc)?;
    println!("Document peer ID: {}", peer_id);

    // Work with text
    println!("\n--- Text Operations ---");
    let text = doc_iface.loro_doc().call_get_text(&mut store, doc, "content")?;

    doc_iface
        .loro_text()
        .call_insert(&mut store, text, 0, "Hello, ")?
        .map_err(|e| anyhow::anyhow!("Insert failed: {:?}", e))?;
    
    doc_iface
        .loro_text()
        .call_insert(&mut store, text, 7, "World!")?
        .map_err(|e| anyhow::anyhow!("Insert failed: {:?}", e))?;

    let text_content = doc_iface.loro_text().call_to_string(&mut store, text)?;
    println!("Text content: \"{}\"", text_content);

    let text_len = doc_iface.loro_text().call_len_unicode(&mut store, text)?;
    println!("Text length (unicode): {}", text_len);

    // Work with map
    println!("\n--- Map Operations ---");
    let map = doc_iface.loro_doc().call_get_map(&mut store, doc, "metadata")?;

    doc_iface
        .loro_map()
        .call_insert_string(&mut store, map, "title", "My Document")?
        .map_err(|e| anyhow::anyhow!("Insert failed: {:?}", e))?;
    
    doc_iface
        .loro_map()
        .call_insert_i64(&mut store, map, "version", 1)?
        .map_err(|e| anyhow::anyhow!("Insert failed: {:?}", e))?;

    let keys = doc_iface.loro_map().call_keys(&mut store, map)?;
    println!("Map keys: {}", keys.join(", "));

    let map_json = doc_iface.loro_map().call_get_deep_value_json(&mut store, map)?;
    println!("Map value: {}", map_json);

    // Work with list
    println!("\n--- List Operations ---");
    let list = doc_iface.loro_doc().call_get_list(&mut store, doc, "items")?;

    doc_iface
        .loro_list()
        .call_push_string(&mut store, list, "First item")?
        .map_err(|e| anyhow::anyhow!("Push failed: {:?}", e))?;
    
    doc_iface
        .loro_list()
        .call_push_i64(&mut store, list, 42)?
        .map_err(|e| anyhow::anyhow!("Push failed: {:?}", e))?;

    let list_len = doc_iface.loro_list().call_len(&mut store, list)?;
    println!("List length: {}", list_len);

    let list_json = doc_iface.loro_list().call_get_deep_value_json(&mut store, list)?;
    println!("List value: {}", list_json);

    // Commit and export
    println!("\n--- Export ---");
    doc_iface.loro_doc().call_commit(&mut store, doc)?;

    let snapshot = doc_iface
        .loro_doc()
        .call_export_snapshot(&mut store, doc)?
        .map_err(|e| anyhow::anyhow!("Export failed: {:?}", e))?;
    println!("Snapshot size: {} bytes", snapshot.len());

    // Get full document state
    println!("\n--- Full Document State ---");
    let deep_value = doc_iface.loro_doc().call_get_deep_value_json(&mut store, doc)?;
    println!("{}", deep_value);

    println!("\n=== Demo Complete ===");
    Ok(())
}
