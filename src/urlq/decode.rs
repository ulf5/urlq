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

    #[test]
    fn plus_decode() {
        assert_eq!(decode_plus("%7e/abc+def"), "~/abc def");
    }
}
