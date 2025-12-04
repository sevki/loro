//! Loro WIT Collaborative Editing Demo with Wasmtime
//!
//! This demonstrates two peers collaborating on a document using Loro CRDTs
//! through the WebAssembly Component Model with Wasmtime as the host runtime.

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
    println!("=== Loro WIT Collaborative Editing Demo with Wasmtime ===\n");
    println!("Simulating two peers (Alice and Bob) collaborating on a document\n");

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
    let doc_iface = instance.component_loro_wit_doc();

    // Create two documents representing different peers
    let alice_doc = doc_iface.loro_doc().call_constructor(&mut store)?;
    doc_iface.loro_doc().call_set_peer_id(&mut store, alice_doc, 1)?
        .map_err(|e| anyhow::anyhow!("Set peer ID failed: {:?}", e))?;
    
    let bob_doc = doc_iface.loro_doc().call_constructor(&mut store)?;
    doc_iface.loro_doc().call_set_peer_id(&mut store, bob_doc, 2)?
        .map_err(|e| anyhow::anyhow!("Set peer ID failed: {:?}", e))?;

    println!("Alice's peer ID: {}", doc_iface.loro_doc().call_peer_id(&mut store, alice_doc)?);
    println!("Bob's peer ID: {}", doc_iface.loro_doc().call_peer_id(&mut store, bob_doc)?);

    // === Round 1: Alice creates initial document ===
    println!("\n--- Round 1: Initial Edits ---");
    println!("\nAlice: Creating initial document...");
    
    let alice_text = doc_iface.loro_doc().call_get_text(&mut store, alice_doc, "content")?;
    doc_iface.loro_text().call_insert(&mut store, alice_text, 0, "Hello")?
        .map_err(|e| anyhow::anyhow!("Insert failed: {:?}", e))?;
    
    let alice_map = doc_iface.loro_doc().call_get_map(&mut store, alice_doc, "metadata")?;
    doc_iface.loro_map().call_insert_string(&mut store, alice_map, "author", "Alice")?
        .map_err(|e| anyhow::anyhow!("Insert failed: {:?}", e))?;
    
    let alice_list = doc_iface.loro_doc().call_get_list(&mut store, alice_doc, "tasks")?;
    doc_iface.loro_list().call_push_string(&mut store, alice_list, "Task 1")?
        .map_err(|e| anyhow::anyhow!("Push failed: {:?}", e))?;
    
    doc_iface.loro_doc().call_commit(&mut store, alice_doc)?;
    println!("Alice's doc: {}", doc_iface.loro_doc().call_get_deep_value_json(&mut store, alice_doc)?);

    // Sync Alice -> Bob
    println!("\n[Sync] Alice -> Bob");
    let alice_updates = doc_iface.loro_doc().call_export_updates(&mut store, alice_doc)?
        .map_err(|e| anyhow::anyhow!("Export failed: {:?}", e))?;
    doc_iface.loro_doc().call_import_bytes(&mut store, bob_doc, &alice_updates)?
        .map_err(|e| anyhow::anyhow!("Import failed: {:?}", e))?;
    println!("Bob received {} bytes", alice_updates.len());
    println!("Bob's doc after sync: {}", doc_iface.loro_doc().call_get_deep_value_json(&mut store, bob_doc)?);

    // === Round 2: Concurrent edits ===
    println!("\n--- Round 2: Concurrent Edits ---");
    
    // Alice edits
    println!("\nAlice: Adding more content...");
    doc_iface.loro_text().call_insert(&mut store, alice_text, 5, " World")?
        .map_err(|e| anyhow::anyhow!("Insert failed: {:?}", e))?;
    doc_iface.loro_list().call_push_string(&mut store, alice_list, "Task 2 (Alice)")?
        .map_err(|e| anyhow::anyhow!("Push failed: {:?}", e))?;
    doc_iface.loro_doc().call_commit(&mut store, alice_doc)?;
    println!("Alice's doc: {}", doc_iface.loro_doc().call_get_deep_value_json(&mut store, alice_doc)?);

    // Bob edits concurrently
    println!("\nBob: Making concurrent edits...");
    let bob_text = doc_iface.loro_doc().call_get_text(&mut store, bob_doc, "content")?;
    doc_iface.loro_text().call_insert(&mut store, bob_text, 5, "!")?
        .map_err(|e| anyhow::anyhow!("Insert failed: {:?}", e))?;
    
    let bob_map = doc_iface.loro_doc().call_get_map(&mut store, bob_doc, "metadata")?;
    doc_iface.loro_map().call_insert_string(&mut store, bob_map, "reviewer", "Bob")?
        .map_err(|e| anyhow::anyhow!("Insert failed: {:?}", e))?;
    
    let bob_list = doc_iface.loro_doc().call_get_list(&mut store, bob_doc, "tasks")?;
    doc_iface.loro_list().call_push_string(&mut store, bob_list, "Task 3 (Bob)")?
        .map_err(|e| anyhow::anyhow!("Push failed: {:?}", e))?;
    
    doc_iface.loro_doc().call_commit(&mut store, bob_doc)?;
    println!("Bob's doc (before sync): {}", doc_iface.loro_doc().call_get_deep_value_json(&mut store, bob_doc)?);

    // === Round 3: Bidirectional sync ===
    println!("\n--- Round 3: Bidirectional Sync ---");
    
    // Sync Alice -> Bob
    println!("\n[Sync] Alice -> Bob");
    let alice_updates2 = doc_iface.loro_doc().call_export_updates(&mut store, alice_doc)?
        .map_err(|e| anyhow::anyhow!("Export failed: {:?}", e))?;
    doc_iface.loro_doc().call_import_bytes(&mut store, bob_doc, &alice_updates2)?
        .map_err(|e| anyhow::anyhow!("Import failed: {:?}", e))?;
    println!("Bob's doc after receiving Alice's updates: {}", 
        doc_iface.loro_doc().call_get_deep_value_json(&mut store, bob_doc)?);

    // Sync Bob -> Alice
    println!("\n[Sync] Bob -> Alice");
    let bob_updates = doc_iface.loro_doc().call_export_updates(&mut store, bob_doc)?
        .map_err(|e| anyhow::anyhow!("Export failed: {:?}", e))?;
    doc_iface.loro_doc().call_import_bytes(&mut store, alice_doc, &bob_updates)?
        .map_err(|e| anyhow::anyhow!("Import failed: {:?}", e))?;
    println!("Alice's doc after receiving Bob's updates: {}", 
        doc_iface.loro_doc().call_get_deep_value_json(&mut store, alice_doc)?);

    // === Verify convergence ===
    println!("\n--- Verification: Both Documents Converged ---");
    let alice_state = doc_iface.loro_doc().call_get_deep_value_json(&mut store, alice_doc)?;
    let bob_state = doc_iface.loro_doc().call_get_deep_value_json(&mut store, bob_doc)?;
    println!("Alice's final state: {}", alice_state);
    println!("Bob's final state:   {}", bob_state);
    
    if alice_state == bob_state {
        println!("\n✓ SUCCESS: Both documents have converged to the same state!");
    } else {
        println!("\n✗ Documents differ (this shouldn't happen with CRDTs)");
    }

    println!("\n=== Demo Complete ===");
    Ok(())
}
