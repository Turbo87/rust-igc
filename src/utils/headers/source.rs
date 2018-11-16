#[derive(Debug, PartialEq, Eq, Hash, Clone, EnumString)]
pub enum HeaderSource {
    FlightRecorder,
    Observer,
    Other(char),
}

impl HeaderSource {
    pub fn from_byte_unchecked(byte: u8) -> HeaderSource {
        debug_assert!(byte.is_ascii());
        match byte {
            b'F' => HeaderSource::FlightRecorder,
            b'O' => HeaderSource::Observer,
            _ => HeaderSource::Other(byte as char),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_byte() {
        assert_eq!(HeaderSource::from_byte_unchecked(b'F'), HeaderSource::FlightRecorder);
        assert_eq!(HeaderSource::from_byte_unchecked(b'O'), HeaderSource::Observer);
        assert_eq!(HeaderSource::from_byte_unchecked(b'2'), HeaderSource::Other('2'));
        assert_eq!(HeaderSource::from_byte_unchecked(b'?'), HeaderSource::Other('?'));
    }

    proptest! {
        #[test]
        #[allow(unused_must_use)]
        fn from_byte_doesnt_crash(b in 0..127u8) {
            HeaderSource::from_byte_unchecked(b)
        }
    }
}
