#![allow(nonstandard_style)]

use crate::rune::Rune;
use crate::trie::{RSPayload, TrieNode};
use std::ffi::c_void;
use std::ptr::NonNull;
use std::slice;

#[repr(transparent)]
pub struct t_len(u16);

// Notes for reviewer:
// Using `repr(C)` here to match the original's straightforward typedef.
#[repr(C)]
pub enum TrieAddOp {
    ADD_REPLACE,
    ADD_INCR,
}

/// Matches the original C implementation's return value for `TrieNode_Add`:
///
/// 1 if the string did not exist there, or 0 if we just replaced the score.

// Notes for reviewer:
// Declared `repr(i32)` as the original is a plain `int` - again, to match the original signature,
// which is _not_ an enum. Since C doesn't really distinguish between ints and enums, this makes
// no practical difference, but is still used here to remain faithful to the original.
#[repr(i32)]
pub enum TrieAddOutcome {
    ScoreReplaced = 0,
    StringAdded = 1,
}

pub type TrieFreeCallback = extern "C" fn(*mut c_void);

/// Creates a new trie root node.
///
/// SAFETY:
/// You can freely pass the returned pointer to `TrieNode_*` functions.
///
/// All of its memory management is handled through the Rust library.
///
/// Do _not_ `free()` the pointer yourself. Use `TrieNode_Free()` instead!
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TrieNode_New() -> NonNull<TrieNode> {
    let node = TrieNode::new();
    Box::into_non_null(node)
}

/// Add a new string to a trie.
///
/// Returns: Whether a new string was actually added, or an existing entry's score updated
///
/// NOTE: `n` may get overwritten with a new node pointer in the process.
///
/// # SAFETY
///
/// * `n` must be a valid, non-null pointer to a node instance created through `TrieNode_New()`. No
///    other call may use the same node for the duration of this one.
/// * `str` and `len` must be a matching pointer/length encoded array of Runes. The runes are copied
///    into the node. This means: **No** ownership is taken, and **no** reference is kept after the
///    function call. It is the **caller's** responsibility to clean up the memory behind `str`.
///    This may be done immediately after the call returns.
/// * `str` must be non-null and aligned, even if `len` is zero.
/// * `str` must not be modified for the duration of the call.
/// * `payload` is treated as opaque by this library: It is assumed to be a valid pointer to a live
///    `RSPayload` instance. As the library does not interact with the payload, no further checks
///    are performed and it is the **caller's** responsibility to ensure that any retrieved payload
///    is still valid and safe to use.
#[unsafe(no_mangle)]
#[expect(unused_variables)]
pub unsafe extern "C" fn TrieNode_Add(
    n: Option<NonNull<Option<NonNull<TrieNode>>>>,
    str: *const Rune,
    len: t_len,
    payload: *mut RSPayload,
    score: f32,
    op: TrieAddOp,
    freecb: Option<NonNull<TrieFreeCallback>>,
) -> TrieAddOutcome {
    // Notes for reviewer:
    // Rust's current `NonNull` implementation assumes that everything can and should be mutable.
    // In the name of at least communicating _intent_, I use it only where functions actually require
    // mutability.
    // -> `str` remains a const pointer, converted to NonNull inside the function where
    //    the invariant won't leak.
    // -> `payload` is considered arbitrary, with no invariants imposed, and therefore also taken as a
    //    plain pointer.
    // -> `freecb`, being a function pointer, is assumed to be trivially `*const`,
    //    with no special intent communication required.

    // SAFETY: the caller guarantees a live node that nothing else uses.
    let n = unsafe { n.unwrap().as_mut() };

    let instance = n.take().unwrap();

    // SAFETY: the caller guarantees a live node that nothing else uses.
    let instance = unsafe { Box::from_non_null(instance) };

    let freecb = unsafe { freecb.unwrap().as_ref() };

    let str = unsafe { slice::from_raw_parts(str, len.0 as usize) };

    let outcome = instance.add(str, payload, score, op, *freecb);
    let new_node = outcome.node;
    *n = Some(Box::into_non_null(new_node));

    outcome.outcome
}
