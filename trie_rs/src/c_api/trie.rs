use crate::rune::Rune;
use crate::trie::TrieNode;
use std::ffi::c_void;

#[repr(transparent)]
pub struct t_len(u16);

/// Split node n at string offset n. This returns a new node which has a string up until offset,
/// and a single child holding The old score of n, and its score

// Using `repr(C)` here to match the original's straightforward typedef.
#[repr(C)]
pub enum TrieAddOp {
    ADD_REPLACE,
    ADD_INCR,
}

/// Matches the original C implementation's return value for `TrieNode_Add`:
///
/// 1 if the string did not exist there, or 0 if we just replaced the score.

// Declared `repr(i32)` as the original is a plain `int` - again, to match the original signature,
// which is _not_ an enum. Since C doesn't really distinguish between ints and enums, this makes
// no practical difference, but is still used here to remain faithful to the original.
#[repr(i32)]
pub enum TrieAddOutcome {
    ScoreReplaced = 0,
    StringAdded = 1,
}

type TrieFreeCallback = extern "C" fn(*mut c_void);

#[unsafe(no_mangle)]
#[expect(unused_variables)]
pub unsafe extern "C" fn TrieNode_Add(
    n: *mut *mut TrieNode,
    str: *const Rune,
    len: t_len,
    payload: *mut crate::c_bindings::RSPayload,
    score: f32,
    op: TrieAddOp,
    freecb: TrieFreeCallback,
) -> TrieAddOutcome {
    todo!()
}
