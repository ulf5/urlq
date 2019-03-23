use url::percent_encoding::{SIMPLE_ENCODE_SET, define_encode_set, EncodeSet};

define_encode_set! {
    /// This encode set is used for query strings.
    ///
    /// Space is not encoded and can be replaced with plus (+) after encoding.
    ///
    /// Aside from special chacters defined in the [`SIMPLE_ENCODE_SET`](struct.SIMPLE_ENCODE_SET.html),
    /// double quote ("), plus (+), hash (#), and inequality qualifiers (<), (>) are encoded.
    pub PLUS_QUERY_ENCODE_SET = [SIMPLE_ENCODE_SET] | {'"', '#', '<', '>', '+'}
}

define_encode_set! {
    /// This encode set is used for encoding all characters that are reserved in any part of a URI.
    /// Also encodes plus (+) and percent (%)
    pub ALL_RESERVED_ENCODE_SET =  [SIMPLE_ENCODE_SET] | {':', '/', '?', '#', '[', ']', '@', '!', '$', '&','\'', '`', '(', ')', '*', '+', ',', ';', '=', ' ', '%'}
}

define_encode_set! {
    /// This encode set is used for encoding all characters that are reserved in any part of a URI, except space ' '
    /// Also encodes percent (%)
    pub ALL_RESERVED_PLUS_ENCODE_SET =  [SIMPLE_ENCODE_SET] | {':', '/', '?', '#', '[', ']', '@', '!', '$', '&','\'', '`', '(', ')', '*', '+', ',', ';', '=', '%'}
}

#[derive(Clone)]
pub struct CustomEncodeSet {
    chars: Vec<u8>
}

impl CustomEncodeSet {
    pub fn from(string: &str) -> CustomEncodeSet {
        CustomEncodeSet {
            chars: string.as_bytes().to_vec(),
        }
    }
}

impl EncodeSet for CustomEncodeSet {
    fn contains(&self, byte: u8) -> bool {
        self.chars.contains(&byte)
    }
}


#[derive(Clone)]
pub struct AllCharactersEncodeSet {}

impl AllCharactersEncodeSet {
    pub fn new() -> AllCharactersEncodeSet {
        AllCharactersEncodeSet {}
    }
}

impl EncodeSet for AllCharactersEncodeSet {
    fn contains(&self, _: u8) -> bool {
        true
    }
}

#[derive(Clone)]
pub struct AllCharactersExceptSpaceEncodeSet {}

impl AllCharactersExceptSpaceEncodeSet {
    pub fn new() -> AllCharactersExceptSpaceEncodeSet {
        AllCharactersExceptSpaceEncodeSet {}
    }
}

impl EncodeSet for AllCharactersExceptSpaceEncodeSet {
    fn contains(&self, byte: u8) -> bool {
        byte != b' '
    }
}
