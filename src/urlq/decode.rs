use percent_encoding::percent_decode;

pub fn decode(url: &str) -> String {
    percent_decode(url.as_bytes())
        .decode_utf8()
        .expect("Percent decoding failed.")
        .to_string()
}

pub fn decode_plus(url: &str) -> String {
    decode(&url.replace("+", " "))
}

#[cfg(test)]
mod tests {
    use crate::decode_plus;
    use crate::decode;

    #[test]
    fn test_decode_plus() {
        assert_eq!(decode_plus("%7e/abc+def"), "~/abc def");
        assert_eq!(decode_plus("%7e/abc%20def"), "~/abc def");
        assert_eq!(decode_plus("+%2B+"), " + ");
        assert_eq!(decode_plus(""), "");
    }

    #[test]
    fn test_decode() {
        assert_eq!(decode("%7e/abc+def"), "~/abc+def");
        assert_eq!(decode("%7e/abc%20def"), "~/abc def");
        assert_eq!(decode("+%2B+"), "+++");
        assert_eq!(decode(""), "");
    }
}
