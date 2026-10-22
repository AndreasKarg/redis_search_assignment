# Andi's Redis Search Assignment

## Design notes

The original C implementation seems to be using some clever macro logic to operate on monolithic node allocation blocks.
I have left this out of the new design: Each node lives in its own box. This is likely not the most performant solution,
but it keeps the API reasonable and safe.

## Assumptions

* `__trieNode_*` functions/macros are not part of the public API
* Exception: `__newTrieNode` is used from the public side to bootstrap a Trie
* `RSPayload`:
  * The trie never interacts with `RSPayload`.
  * Allocation happens C-side.
  * De-allocation is assumed to be what `TrieFreeCallback` is for.

## Notes

I chose `cbindgen` over `cheadergen` for two reasons:

* `corrosion` has a predefined function to wire the generator into a CMake project
* `cheadergen`'s known limitations would have made the `TRIE_32_BIT_RUNES` compile-time switch harder to replicate

## Open Questions

### How to create the root node?

Without digging further into the lib, there is no obvious way to generate the root node. C's `TrieNode_Add` implementation
assumes a valid pointer to an initialised `TrieNode` struct.

I have chosen to leave this gap unfilled in the Rust-side C API for now.

### The `RSPayload` struct

~~The `RSPayload` type seems to be deeply embedded into the overall library structure.
This makes it difficult to design a safe Rust API around it.~~

~~For this exercise, I have chosen to import the type from the C side via bindgen and treating it as an opaque, never-owned type.
This allows the internal API to treat references as `'static`.~~

Update: See assumptions

## Possible improvements

* The C-side `Rune` type could be replaced with Rust `char`s internally.
* If UTF-8 can be guaranteed for the actual payloads, the API could be refactored to operate on those.

