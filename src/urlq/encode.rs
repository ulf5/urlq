use crate::urlq::encode_sets::{PLUS_QUERY_ENCODE_SET, ALL_ENCODE_SET};
use percent_encoding::percent_encode;
use url::Url;
use url::percent_encoding::{SIMPLE_ENCODE_SET, PATH_SEGMENT_ENCODE_SET, QUERY_ENCODE_SET, DEFAULT_ENCODE_SET, USERINFO_ENCODE_SET};
use percent_encoding::EncodeSet;
use url::ParseError;
use crate::urlq::encode_sets::CustomEncodeSet;

pub fn encode_url(url: &str) -> Result<String, ParseError> {
    Url::parse(url)
        .map(|u| u.to_string())
}

pub fn encode_url_plus(url: &str) -> Result<String, ParseError> {
    let mut parts = url.splitn(2, "?");
    let first = parts.next().expect("Malformed");
    let second = parts.next();
    match second {
        Some(part) => {
            let encoded_first = encode_url(first);
            let encoded = encoded_first.map(|mut u: String| {
                u.push_str(&handle_query_and_fragment_plus(part));
                u
            });
            return encoded;
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

pub fn encode_query(query: &str) -> String {
    encode(query, QUERY_ENCODE_SET)
}

pub fn encode_query_plus(query: &str) -> String {
    encode(query, PLUS_QUERY_ENCODE_SET).replace(' ', "+")
}

pub fn encode_path(path: &str) -> String {
    encode(path, DEFAULT_ENCODE_SET)
}

pub fn encode_path_segment(path_segment: &str) -> String {
    encode(path_segment, PATH_SEGMENT_ENCODE_SET)
}

pub fn encode_userinfo(userinfo: &str) -> String {
    encode(userinfo, USERINFO_ENCODE_SET)
}

pub fn encode_fragment(fragment: &str) -> String {
    encode(fragment, SIMPLE_ENCODE_SET)
}

pub fn encode_all_reserved(string: &str) -> String {
    encode(string, ALL_ENCODE_SET)
}

pub fn encode_characters(string: &str, chars_to_encode: &str) -> String {
    encode(string, CustomEncodeSet::from(chars_to_encode))
}

fn encode(string: &str, encode_set: impl EncodeSet) -> String {
    percent_encode(string.as_bytes(), encode_set).to_string()
}

#[cfg(test)]
mod tests {
    use crate::{encode_query, encode_query_plus, encode_url, encode_url_plus};

    #[test]
    fn test_encode_query_plus() {
        assert_eq!(encode_query_plus("~/abc def"), "~/abc+def");
        assert_eq!(encode_query_plus("#20/abc def"), "%2320/abc+def");
        assert_eq!(encode_query_plus("?20/abc def"), "?20/abc+def");
        assert_eq!(encode_query_plus(" + "), "+%2B+");
        assert_eq!(encode_query_plus(""), "");
    }

    #[test]
    fn test_encode_query() {
        assert_eq!(encode_query("~/abc def"), "~/abc%20def");
        assert_eq!(encode_query("#20/abc def"), "%2320/abc%20def");
        assert_eq!(encode_query("?20/abc def"), "?20/abc%20def");
        assert_eq!(encode_query(" + "), "%20+%20");
        assert_eq!(encode_query(""), "");
    }

    #[test]
    fn test_encode_url() {
        assert_eq!(encode_url("http://www.example.com/~/abc def%").unwrap(), "http://www.example.com/~/abc%20def%");
        //assert_eq!(encode_url("#20/abc def"), "%2320/abc%20def");
        //assert_eq!(encode_url("?20/abc def"), "?20/abc%20def");
        //assert_eq!(encode_url(" + "), "%20+%20");
        //assert_eq!(encode_url(""), "");
    }
}
