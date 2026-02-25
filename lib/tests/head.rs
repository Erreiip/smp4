#[cfg(test)]
mod tests {

    use lib::head::{head::Head, head_builder::HeadBuilder};
    use pretty_assertions::assert_eq;

    #[test]
    fn head_test_suite() {
        head_protobuff_codec_example();
    }

    #[test]
    fn head_protobuff_codec_example() {
        let head_impl = Head {
            document_start: 12345,
            metadata_start: 67890,
            hash_algos: String::from("SHA256"),
            signature_algo: String::from("ED25519"),
        };

        let encoded = head_impl.encode().expect("NTR");

        let second_head = Head::decode(encoded).expect("NTR");

        assert_eq!(second_head.document_start, 12345);
        assert_eq!(second_head.metadata_start, 67890);
        assert_eq!(second_head.hash_algos, "SHA256");
        assert_eq!(second_head.signature_algo, "ED25519");
    }

    #[test]
    fn head_builder_empty_document() {
        let doc_start = 0;
        let hash_algos = String::from("SHA256");
        let signature_algo = String::from("ED25519");

        let head = HeadBuilder::new(doc_start, doc_start, hash_algos, signature_algo).build();

        assert_eq!(head.metadata_start, head.document_start);
        assert_eq!(head.hash_algos, "SHA256");
        assert_eq!(head.signature_algo, "ED25519");
    }

    #[test]
    fn head_builder_value_one() {
        let doc_start = 1;
        let metadata_start = 1;
        let hash_algos = String::from("SHA256");
        let signature_algo = String::from("ED25519");

        let head = HeadBuilder::new(doc_start, metadata_start, hash_algos, signature_algo).build();

        assert_eq!(head.document_start > 1, true);
        assert_eq!(head.metadata_start, head.document_start + 1);
        assert_eq!(head.hash_algos, "SHA256");
        assert_eq!(head.signature_algo, "ED25519");
    }

    #[test]
    fn head_builder_output_are_consistent() {
        let hash_algos = String::from("SHA256");
        let signature_algo = String::from("ED25519");

        let doc_start1 = 1;
        let metadata_start1 = 1;
        let doc_start2 = 100000;
        let metadata_start2 = 1000000;

        let head1 = HeadBuilder::new(
            doc_start1,
            metadata_start1,
            hash_algos.clone(),
            signature_algo.clone(),
        )
        .build();
        let head2 = HeadBuilder::new(
            doc_start2,
            metadata_start2,
            hash_algos.clone(),
            signature_algo.clone(),
        )
        .build();

        assert_eq!(
            head1.document_start + (doc_start2 - doc_start1),
            head2.document_start
        );
        assert_eq!(
            head1.metadata_start + (doc_start2 - doc_start1) + (metadata_start2 - metadata_start1),
            head2.metadata_start
        );
        assert_eq!(head1.hash_algos, head2.hash_algos);
        assert_eq!(head1.signature_algo, head2.signature_algo);
    }
}
