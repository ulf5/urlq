pub use self::urlq::encode::{
    encode_url_plus, encode_url, encode_all_reserved, encode_query, encode_query_plus, encode_path,
    encode_path_segment, encode_userinfo, encode_fragment
};
pub use self::urlq::decode::{decode, decode_plus};

mod urlq;