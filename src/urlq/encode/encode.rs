use crate::urlq::encode_sets::{PLUS_QUERY_ENCODE_SET, ALL_ENCODE_SET};
use percent_encoding::percent_encode;
use url::Url;
use url::percent_encoding::SIMPLE_ENCODE_SET;
use percent_encoding::EncodeSet;

pub fn encode_url(url: &str) -> String {
    Url::parse(url)
        .expect(&format!("Couldn't parse url: {}", url))
        .to_string()
}

pub fn encode_url_plus(url: &str) -> String {
    let mut parts = url.splitn(2, "?");
    let first = parts.next().expect("malformed");
    let second = parts.next();
    match second {
        Some(part) => {
            let mut encoded_first = encode_url(first);
            encoded_first.push_str(&handle_query_and_fragment_plus(part));
            return encoded_first;
        }
        None => return encode_url(url)
    }
}

fn handle_query_and_fragment_plus(query_and_fragment: &str) -> String {
    let mut parts = query_and_fragment.splitn(2, '#');
    let query = parts.next().expect("Malformed");
    let fragment = parts.next();

    let mut encoded_query = encode_query_plus(query);
    match fragment {
        Some(fragment) => {
            encoded_query.push_str(&encode(fragment, SIMPLE_ENCODE_SET));
            return encoded_query;
        }
        None => return encoded_query
    }
}

pub fn encode_query_plus(query: &str) -> String {
    encode(query, PLUS_QUERY_ENCODE_SET).replace(' ', "+")
}

pub fn all_encode(url: &str) -> String {
    encode(url, ALL_ENCODE_SET)
}

fn encode(string: &str, encode_set: impl EncodeSet) -> String {
    percent_encode(string.as_bytes(), encode_set).to_string()
}
