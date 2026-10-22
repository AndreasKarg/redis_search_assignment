#![allow(nonstandard_style)]

use std::ffi::{c_char, CStr};
use std::ptr::NonNull;
use crate::rune::{str_to_runes, Rune};

/// Convert a utf-8 string to constant width runes.
///
/// The number of runes gets written into the `len` parameter.
///
/// # SAFETY
///
/// * `str` must be a valid, non-null pointer to a null-terminated UTF-8 string that does not get
///   modified for the duration of the function.
/// * `len` must be a valid, non-null pointer to a single value.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn strToRunes(str: *const c_char, len: Option<NonNull<usize>>) -> NonNull<Rune>
{
    if str.is_null() {
        panic!("`str` must not be null");
    }

    let mut len = len.unwrap();

    // SAFETY: Invariants enforced through docstring & validation
    let str = unsafe { CStr::from_ptr(str) };
    let str = str.to_str().unwrap();

    let runes = str_to_runes(str);

    let runes = runes.into_boxed_slice();

    // SAFETY: Invariants enforced through docstring & validation
    unsafe {
        *(len.as_mut()) = runes.len();
    }

    // SAFETY: Box guarantees validity and non-null-ness
    unsafe {
        let runes = Box::into_raw(runes) as *mut Rune;
        NonNull::new_unchecked(runes)
    }
}

/// Free a rune string
///
/// # SAFETY
///
/// * `runes` must be a valid, non-null pointer to a rune string created through `strToRunes()`.
/// * `runes` will _not_ be in a valid state after this call and must not be used again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn runeFree(runes: Option<NonNull<Rune>>) {
    if let Some(runes) = runes {
        // SAFETY: Invariants enforced through docstring & validation
        let runes = unsafe { Box::from_non_null(runes) };
        drop(runes);
    }
}
