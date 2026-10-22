use crate::c_api::trie::{OwnedPayload, TrieAddOp, TrieAddOutcome, TrieFreeCallback};
use crate::rune::Rune;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// RSPayload is assumed to be opaque and managed entirely on the C side.
pub struct RSPayload;

/// TrieNode represents a single node in a trie.
///
/// Non terminal nodes always have a score of 0, meaning you can't insert nodes with score 0 to the
/// trie.
#[derive(Clone)]
pub struct TrieNode {
    fake_contents: Rc<RefCell<HashMap<Vec<Rune>, OwnedPayload>>>,
}

impl TrieNode {
    /// Create a new root Trie node.
    pub fn new() -> Box<Self> {
        Box::new(Self {
            fake_contents: Rc::new(RefCell::new(HashMap::new())),
        })
    }

    // Assumptions:
    // * The original's `len` parameter refers to the number of runes in the `str` parameter
    //   (inferred to be an array of runes from the signature of `__newTrieNode`)
    // * The payload is a single item
    // * The operation is infallible
    pub fn add(
        self: Box<Self>,
        str: &[Rune],
        payload: OwnedPayload,
        _score: f32,
        _op: TrieAddOp,
    ) -> AddOutcome {
        // Fake impl
        let str = str.to_vec();
        self.fake_contents.borrow_mut().insert(str, payload);

        // To demonstrate memory validity, always create a new node here, which
        // will cause the original C ptr to get overwritten.
        let node = self.clone();

        AddOutcome {
            node,
            outcome: TrieAddOutcome::StringAdded,
        }
    }

    pub fn get_value(&self, str: &[Rune], _exact: bool) -> Option<*mut RSPayload> {
        self.fake_contents.borrow().get(str).map(|p| p.payload())
    }
}

pub struct AddOutcome {
    pub node: Box<TrieNode>,
    pub outcome: TrieAddOutcome,
}

#[cfg(test)]
mod tests {
    // Imagine a thorough suite of unittests here, built up through TDD
}
