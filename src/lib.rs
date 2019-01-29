use percent_encoding::{percent_decode, percent_encode, PATH_SEGMENT_ENCODE_SET};
use url::Url;
use url::percent_encoding::{SIMPLE_ENCODE_SET, define_encode_set};

pub fn decode(url: &str) -> String {
    percent_decode(url.as_bytes())
        .decode_utf8()
        .expect("Percent decoding failed.")
        .to_string()
}

pub fn decode_plus(url: &str) -> String {
    decode(&url.replace("+", " "))
}

// TODO: support plus-encoding
pub fn encode_plus_url(url: &str) -> String {
    let mut parts = url.splitn(2, "?");
    let first = parts.next().expect("malformed");
    let second = parts.next();
    match second {
        Some(part) => {
            let mut encoded_first = encode_url(first);
            encoded_first.push_str(&handle_query_and_fragment(part));
            return encoded_first;
        }
        None => return encode_url(url)
    }
}


define_encode_set! {
    /// This encode set is used in the URL parser for query strings.
    ///
    /// Aside from special chacters defined in the [`SIMPLE_ENCODE_SET`](struct.SIMPLE_ENCODE_SET.html),
    /// space, double quote ("), hash (#), and inequality qualifiers (<), (>) are encoded.
    pub PLUS_QUERY_ENCODE_SET = [SIMPLE_ENCODE_SET] | {'"', '#', '<', '>'}
}

fn handle_query_and_fragment(query_and_fragment: &str) -> String {
    let mut parts = query_and_fragment.splitn(2, '#');
    let query = parts.next().expect("Malformed");
    let fragment = parts.next();

    let mut encoded_query = percent_encode(query.as_bytes(), PLUS_QUERY_ENCODE_SET)
        .to_string()
        .replace(' ', "+");
    match fragment {
        Some(fragment) => {
            encoded_query.push_str(&percent_encode(fragment.as_bytes(), SIMPLE_ENCODE_SET).to_string());
            return encoded_query;
        }
        None => return encoded_query
    }
}

pub fn encode_url(url: &str) -> String {
    let parsed = Url::parse(url);
    match parsed {
        Ok(parsed_url) => String::from(parsed_url.as_str()),
        Err(_) => all_encode(url)
    }
}

pub fn all_encode(url: &str) -> String {
    // TODO: Replace with better (correct) encoding set
    percent_encode(url.as_bytes(), PATH_SEGMENT_ENCODE_SET).to_string()
}

#[cfg(test)]
mod tests {
    use crate::decode_plus;

    #[test]
    fn plus_decode() {
        assert_eq!(decode_plus("%7e/abc+def"), "~/abc def");
    }
}
