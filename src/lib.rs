use percent_encoding::{percent_decode, percent_encode, PATH_SEGMENT_ENCODE_SET};
use url::Url;
use url::percent_encoding::{QUERY_ENCODE_SET, SIMPLE_ENCODE_SET};

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
pub fn encode_plus(url: &str) -> String {
    let options = Url::options();
    url::define_encode_set! {
        pub PLUS_QUERY_ENCODE_SET = [SIMPLE_ENCODE_SET] | { '"', '#', '<', '>'}
    }

    QUERY_ENCODE_SET = PLUS_QUERY_ENCODE_SET;
    encode(url).replace(" ", "+")
}

pub fn encode(url: &str) -> String {
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
