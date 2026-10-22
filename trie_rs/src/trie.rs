use crate::c_api::trie::{TrieAddOp, TrieAddOutcome, TrieFreeCallback};
use crate::rune::Rune;

/// RSPayload is assumed to be opaque and managed entirely on the C side.
pub struct RSPayload;

/// TrieNode represents a single node in a trie.
///
/// Non terminal nodes always have a score of 0, meaning you can't insert nodes with score 0 to the
/// trie.
pub struct TrieNode;

impl TrieNode {
    /// Create a new root Trie node.
    pub fn new() -> Box<Self>{
        unimplemented!("Stub");
    }

    // Assumptions:
    // * The original's `len` parameter refers to the number of runes in the `str` parameter
    //   (inferred to be an array of runes from the signature of `__newTrieNode`)
    // * The payload is a single item
    // * The operation is infallible
    pub fn add(self: Box<Self>, str: &[Rune], payload: *mut RSPayload, score: f32, op: TrieAddOp, freecb: TrieFreeCallback) -> AddOutcome {
        // Stub impl

        AddOutcome {
            node: self,
            outcome: TrieAddOutcome::StringAdded,
        }
    }
}

pub struct AddOutcome {
    pub node: Box<TrieNode>,
    pub outcome: TrieAddOutcome
}

#[cfg(test)]
mod tests {
    // Imagine a thorough suite of unittests here, built up through TDD
}
