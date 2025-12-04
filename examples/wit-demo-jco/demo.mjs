// Loro WIT Demo with JCO
// This demonstrates using Loro CRDTs through the WebAssembly Component Model

import { doc } from './gen/loro_wit.js';

const { LoroDoc } = doc;

async function main() {
    console.log("=== Loro WIT Demo with JCO ===\n");

    // Create a new document
    console.log("Creating new LoroDoc...");
    const doc1 = new LoroDoc();
    console.log(`Document peer ID: ${doc1.peerId()}`);

    // Work with text
    console.log("\n--- Text Operations ---");
    const text = doc1.getText("content");
    text.insert(0, "Hello, ");
    text.insert(7, "World!");
    console.log(`Text content: "${text.toString()}"`);
    console.log(`Text length (unicode): ${text.lenUnicode()}`);

    // Work with map
    console.log("\n--- Map Operations ---");
    const map = doc1.getMap("metadata");
    map.insertString("title", "My Document");
    map.insertI64("version", 1n);
    map.insertBool("published", false);
    console.log(`Map keys: ${map.keys().join(", ")}`);
    console.log(`Map value: ${map.getDeepValueJson()}`);

    // Work with list
    console.log("\n--- List Operations ---");
    const list = doc1.getList("items");
    list.pushString("First item");
    list.pushString("Second item");
    list.pushI64(42n);
    console.log(`List length: ${list.len()}`);
    console.log(`List value: ${list.getDeepValueJson()}`);

    // Commit changes
    doc1.commit();

    // Export and import
    console.log("\n--- Export/Import ---");
    const snapshot = doc1.exportSnapshot();
    console.log(`Snapshot size: ${snapshot.length} bytes`);

    // Create another document and import
    const doc2 = new LoroDoc();
    doc2.importBytes(snapshot);
    console.log(`Doc2 text: "${doc2.getText("content").toString()}"`);

    // Fork demonstration
    console.log("\n--- Fork ---");
    const forked = doc1.fork();
    console.log(`Original peer: ${doc1.peerId()}`);
    console.log(`Forked peer: ${forked.peerId()}`);

    // Make changes in forked
    forked.getText("content").insert(0, "[Forked] ");
    forked.commit();
    console.log(`Forked text: "${forked.getText("content").toString()}"`);

    // Get deep value
    console.log("\n--- Full Document State ---");
    console.log(doc1.getDeepValueJson());

    console.log("\n=== Demo Complete ===");
}

main().catch(console.error);
