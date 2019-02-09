pub use self::urlq::encode::{
    encode_url_plus, encode_url, encode_all_reserved, encode_query, encode_query_plus, encode_path,
    encode_path_segment, encode_userinfo, encode_fragment, encode_characters, encode_all_reserved_plus,
    encode_all, encode_all_plus
};
pub use self::urlq::decode::{decode, decode_plus};

mod urlq;