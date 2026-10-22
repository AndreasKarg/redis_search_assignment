use std::ffi::CStr;

#[cfg(feature = "trie_32_bit_runes")]
#[repr(transparent)]
pub struct Rune(u32);

#[cfg(not(feature = "trie_32_bit_runes"))]
#[repr(transparent)]
pub struct Rune(u16);

pub fn str_to_runes(str: &str) -> Vec<Rune> {
    // Stub: Just cast each byte into a "Rune"
    assert!(str.is_ascii());
    str.bytes().map(|b| Rune(b.into())).collect()
}

#[cfg(test)]
mod tests {
    // Imagine a thorough suite of unittests here, built up through TDD
}
