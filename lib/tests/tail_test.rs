#[cfg(test)]
mod tests {

    use smp4_common::{codec::cbor::CborEncoder, tail::tail::{TailEncoder, TailFields}};
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
        tail.add_entry(TailFields::METADATA_START, "100");
        tail.add_entry(TailFields::SIGNATURE_START, "100");
        tail.add_entry(TailFields::HASH_ALGS, "SHA3");
        tail.add_entry(TailFields::SIGN_ALG, "ECDSA");

        assert_eq!(tail.encode().is_err(), false)
    }
}
