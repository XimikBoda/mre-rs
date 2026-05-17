use alloc::string::String;
use alloc::vec::Vec;

pub fn to_ucs2(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(core::iter::once(0)).collect()
}

pub fn from_ucs2(ucs2: &[u16]) -> String {
    let len = ucs2.iter().position(|&c| c == 0).unwrap_or(ucs2.len());
    String::from_utf16_lossy(&ucs2[..len])
}