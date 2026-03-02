#[cfg(test)]
mod tests {

    use lib::{codec::{cbor::CborEncoder}, tail::tail::TailEncoder};
    use pretty_assertions::assert_eq;

    #[test]
    fn tail_missing_fields() {
        let mut tail = TailEncoder::new(CborEncoder::default());
        tail.add_entry("unused entry", "test");

        assert_eq!(tail.encode().is_err(), true);
    }

    #[test]
    fn tail_fill_fields() {
        let mut tail = TailEncoder::new(CborEncoder::default());
        tail.add_entry(TailEncoder::<CborEncoder>::METADATA_START, "100");
        tail.add_entry(TailEncoder::<CborEncoder>::SIGNATURE_START, "100");
        tail.add_entry(TailEncoder::<CborEncoder>::HASH_ALGS, "SHA3");
        tail.add_entry(TailEncoder::<CborEncoder>::SIGN_ALG, "ECDSA");

        assert_eq!(tail.encode().is_err(), false)
    }
}
