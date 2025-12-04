// Loro WIT Demo with JCO
// This demonstrates collaborative editing using Loro CRDTs through the WebAssembly Component Model
// Two peers make concurrent edits and sync with each other

import { doc } from './gen/loro_wit.js';

const { LoroDoc } = doc;

async function main() {
    console.log("=== Loro WIT Collaborative Editing Demo ===\n");
    console.log("Simulating two peers (Alice and Bob) collaborating on a document\n");

    // Create two documents representing different peers
    const aliceDoc = new LoroDoc();
    aliceDoc.setPeerId(1n);
    const bobDoc = new LoroDoc();
    bobDoc.setPeerId(2n);

    console.log(`Alice's peer ID: ${aliceDoc.peerId()}`);
    console.log(`Bob's peer ID: ${bobDoc.peerId()}`);

    // === Round 1: Initial edits ===
    console.log("\n--- Round 1: Initial Edits ---");
    
    // Alice creates the document structure
    console.log("\nAlice: Creating initial document...");
    aliceDoc.getText("content").insert(0, "Hello");
    aliceDoc.getMap("metadata").insertString("author", "Alice");
    aliceDoc.getList("tasks").pushString("Task 1");
    aliceDoc.commit();
    console.log(`Alice's doc: ${aliceDoc.getDeepValueJson()}`);

    // Sync Alice -> Bob
    console.log("\n[Sync] Alice -> Bob");
    const aliceUpdates1 = aliceDoc.exportUpdates();
    bobDoc.importBytes(aliceUpdates1);
    console.log(`Bob received ${aliceUpdates1.length} bytes`);
    console.log(`Bob's doc after sync: ${bobDoc.getDeepValueJson()}`);

    // === Round 2: Concurrent edits ===
    console.log("\n--- Round 2: Concurrent Edits (Both peers edit simultaneously) ---");

    // Alice edits
    console.log("\nAlice: Adding more content...");
    aliceDoc.getText("content").insert(5, " World");
    aliceDoc.getList("tasks").pushString("Task 2 (Alice)");
    aliceDoc.commit();
    console.log(`Alice's doc: ${aliceDoc.getDeepValueJson()}`);

    // Bob edits concurrently (without seeing Alice's changes yet)
    console.log("\nBob: Making concurrent edits...");
    bobDoc.getText("content").insert(5, "!");  // Will merge with Alice's " World"
    bobDoc.getMap("metadata").insertString("reviewer", "Bob");
    bobDoc.getList("tasks").pushString("Task 3 (Bob)");
    bobDoc.commit();
    console.log(`Bob's doc (before sync): ${bobDoc.getDeepValueJson()}`);

    // === Round 3: Bidirectional sync ===
    console.log("\n--- Round 3: Bidirectional Sync ---");

    // Sync Alice -> Bob
    console.log("\n[Sync] Alice -> Bob");
    const aliceUpdates2 = aliceDoc.exportUpdates();
    bobDoc.importBytes(aliceUpdates2);
    console.log(`Bob's doc after receiving Alice's updates: ${bobDoc.getDeepValueJson()}`);

    // Sync Bob -> Alice
    console.log("\n[Sync] Bob -> Alice");
    const bobUpdates = bobDoc.exportUpdates();
    aliceDoc.importBytes(bobUpdates);
    console.log(`Alice's doc after receiving Bob's updates: ${aliceDoc.getDeepValueJson()}`);

    // === Verify convergence ===
    console.log("\n--- Verification: Both Documents Converged ---");
    console.log(`Alice's final state: ${aliceDoc.getDeepValueJson()}`);
    console.log(`Bob's final state:   ${bobDoc.getDeepValueJson()}`);
    
    const aliceState = aliceDoc.getDeepValueJson();
    const bobState = bobDoc.getDeepValueJson();
    if (aliceState === bobState) {
        console.log("\n✓ SUCCESS: Both documents have converged to the same state!");
    } else {
        console.log("\n✗ Documents differ (this shouldn't happen with CRDTs)");
    }

    // Show version info
    console.log("\n--- Version Information ---");
    console.log(`Alice's frontiers: ${JSON.stringify(aliceDoc.stateFrontiers())}`);
    console.log(`Bob's frontiers: ${JSON.stringify(bobDoc.stateFrontiers())}`);

    console.log("\n=== Demo Complete ===");
}

main().catch(console.error);
