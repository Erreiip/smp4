
use crate::head::head::Head;

pub struct HeadBuilder {
    doc_size: u64,
    metadata_size: u64,
    hash_algos: String,
    signature_algo: String,
}

impl HeadBuilder {

    pub fn new(
        doc_size: u64,
        metadata_size: u64,
        hash_algos: String,
        signature_algo: String,
    ) -> Self {
        return HeadBuilder {
            doc_size,
            metadata_size,
            hash_algos,
            signature_algo,
        }
    }

    pub fn build(&self) -> Head {

        let head = Head {
            document_start: self.doc_size,
            metadata_start: self.metadata_size + self.doc_size,
            hash_algos: self.hash_algos.clone(),
            signature_algo: self.signature_algo.clone()
        };

        let buffer = head.encode();
        let len = buffer.expect("Impossible to read the buffer").len() as u64;

        Head {
            document_start: self.doc_size + len,
            metadata_start: self.metadata_size + self.doc_size + len,
            hash_algos: self.hash_algos.clone(),
            signature_algo: self.signature_algo.clone()
        }
    }
}
