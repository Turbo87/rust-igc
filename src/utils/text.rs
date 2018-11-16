use encoding::{DecoderTrap, Encoding};
use encoding::all::{UTF_8, ISO_8859_1};

/// Parse a byte slice as either UTF8 or Latin1
pub fn as_text(bytes: &[u8]) -> Option<String> {
    let bytes = bytes.into();
    UTF_8.decode(bytes, DecoderTrap::Strict)
        .or_else(|_| ISO_8859_1.decode(bytes, DecoderTrap::Strict))
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_text() {
        assert_eq!(as_text(b"John Doe").unwrap(), "John Doe");
        assert_eq!(as_text("Jörg Müller".as_bytes()).unwrap(), "Jörg Müller");
        assert_eq!(as_text(b"J\xf6rg M\xfcller").unwrap(), "Jörg Müller");
    }

    proptest! {
        #[test]
        #[allow(unused_must_use)]
        fn test_as_text_with_random_data(input in r"\PC*") {
            as_text(input.as_bytes());
        }
    }
}
