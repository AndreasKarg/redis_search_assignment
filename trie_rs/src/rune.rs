#[cfg(feature = "trie_32_bit_runes")]
#[repr(transparent)]
pub struct Rune(u32);

#[cfg(not(feature = "trie_32_bit_runes"))]
#[repr(transparent)]
pub struct Rune(u16);
