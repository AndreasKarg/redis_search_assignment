# Andi's Redis Search Assignment

## Open Questions

### How to create the root node?

Without digging further into the lib, there is no obvious way to generate the root node. C's `TrieNode_Add` implementation
assumes a valid pointer to an initialised `TrieNode` struct.

I have chosen to leave this gap unfilled in the Rust-side C API for now.

### The `RSPayload` struct

The `RSPayload` type seems to be deeply embedded into the overall library structure.
This makes it difficult to design a safe Rust API around it.

For this exercise, I have chosen to import the type from the C side via bindgen and treating it as an opaque, never-owned type.
This allows the internal API to treat references as `'static`.
