use crate::urlq::encode_sets::{PLUS_QUERY_ENCODE_SET, ALL_RESERVED_ENCODE_SET, ALL_RESERVED_PLUS_ENCODE_SET};
use percent_encoding::percent_encode;
use url::Url;
use url::percent_encoding::{SIMPLE_ENCODE_SET, PATH_SEGMENT_ENCODE_SET, QUERY_ENCODE_SET, DEFAULT_ENCODE_SET, USERINFO_ENCODE_SET};
use percent_encoding::EncodeSet;
use url::ParseError;
use crate::urlq::encode_sets::CustomEncodeSet;
use crate::urlq::encode_sets::AllCharactersEncodeSet;
use crate::urlq::encode_sets::AllCharactersExceptSpaceEncodeSet;

pub fn encode_url(url: &str) -> Result<String, ParseError> {
    Url::parse(url)
        .map(|u| u.to_string())
}

pub fn encode_url_plus(url: &str) -> Result<String, ParseError> {
    let mut parts = url.splitn(2, '?');
    let first = parts.next().expect("Malformed");
    let second = parts.next();
    match second {
        Some(part) => {
            let encoded_first = encode_url(first);
            encoded_first.map(|mut u: String| {
                u.push('?');
                u.push_str(&handle_query_and_fragment_plus(part));
                u
            })
        }
        None => encode_url(url)
    }
}

fn handle_query_and_fragment_plus(query_and_fragment: &str) -> String {
    let mut parts = query_and_fragment.splitn(2, '#');
    let query = parts.next().expect("Malformed");
    let fragment = parts.next();

    let mut encoded_query = encode_query_plus(query);
    match fragment {
        Some(fragment) => {
            encoded_query.push('#');
            encoded_query.push_str(&encode_fragment(fragment));
            encoded_query
        }
        None => encoded_query
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
    encode(string, ALL_RESERVED_ENCODE_SET)
}

pub fn encode_all_reserved_plus(string: &str) -> String {
    encode(string, ALL_RESERVED_PLUS_ENCODE_SET).replace(' ', "+")
}

pub fn encode_all(string: &str) -> String {
    encode(string, AllCharactersEncodeSet::new())
}

pub fn encode_all_plus(string: &str) -> String {
    encode(string, AllCharactersExceptSpaceEncodeSet::new()).replace(' ', "+")
}

pub fn encode_characters(string: &str, chars_to_encode: &str) -> String {
    encode(string, CustomEncodeSet::from(chars_to_encode))
}

fn encode(string: &str, encode_set: impl EncodeSet) -> String {
    percent_encode(string.as_bytes(), encode_set).to_string()
}

#[cfg(test)]
mod tests {
    use crate::{encode_query, encode_query_plus, encode_url, encode_url_plus, encode_all,
                encode_all_plus, encode_all_reserved, encode_all_reserved_plus};

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
        assert_eq!(encode_url("http://www.example.com/~/abc def%/a?foo bar=1#anchor 1").unwrap(), "http://www.example.com/~/abc%20def%/a?foo%20bar=1#anchor 1");
        assert!(encode_url("#20/abc def").is_err());
        assert!(encode_url("").is_err());
    }

    #[test]
    fn test_encode_url_plus() {
        assert_eq!(encode_url_plus("http://www.example.com/~/abc def%/a?foo bar=1").unwrap(), "http://www.example.com/~/abc%20def%/a?foo+bar=1");
        assert_eq!(encode_url_plus("http://www.example.com/~/abc def%/a?foo bar=1#anchor 1").unwrap(), "http://www.example.com/~/abc%20def%/a?foo+bar=1#anchor 1");
        assert!(encode_url_plus("#20/abc def").is_err());
        assert!(encode_url_plus("").is_err());
    }

    #[test]
    fn test_encode_all_reserved() {
        assert_eq!(encode_all_reserved("?20/abc def"), "%3F20%2Fabc%20def");
        assert_eq!(encode_all_reserved(":/?#[]@!$&,'`()*+,;= %"), "%3A%2F%3F%23%5B%5D%40%21%24%26%2C%27%60%28%29%2A%2B%2C%3B%3D%20%25");
        assert_eq!(encode_all_reserved(""), "");
    }

    #[test]
    fn test_encode_all_reserved_plus() {
        assert_eq!(encode_all_reserved_plus("?20/abc def"), "%3F20%2Fabc+def");
        assert_eq!(encode_all_reserved_plus(":/?#[]@!$&,'`()*+,;= %"), "%3A%2F%3F%23%5B%5D%40%21%24%26%2C%27%60%28%29%2A%2B%2C%3B%3D+%25");
        assert_eq!(encode_all_reserved_plus(""), "");
    }

    #[test]
    fn test_encode_all() {
        assert_eq!(encode_all("?20/abc def"), "%3F%32%30%2F%61%62%63%20%64%65%66");
        assert_eq!(encode_all(":/?#[]@!$&,'`()*+,;= %"), "%3A%2F%3F%23%5B%5D%40%21%24%26%2C%27%60%28%29%2A%2B%2C%3B%3D%20%25");
        assert_eq!(encode_all(""), "");
    }

    #[test]
    fn test_encode_all_plus() {
        assert_eq!(encode_all_plus("?20/abc def"), "%3F%32%30%2F%61%62%63+%64%65%66");
        assert_eq!(encode_all_plus(":/?#[]@!$&,'`()*+,;= %"), "%3A%2F%3F%23%5B%5D%40%21%24%26%2C%27%60%28%29%2A%2B%2C%3B%3D+%25");
        assert_eq!(encode_all_plus(""), "");
    }
}
