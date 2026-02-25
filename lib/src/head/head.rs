pub mod head_proto {
    include!(concat!(env!("OUT_DIR"), "/head_proto.rs"));
}

use bytes::{BytesMut};
use prost::Message;

use head_proto::HeadProto;

pub struct Head {
    pub document_start: u64,
    pub metadata_start: u64,
    pub hash_algos: String,
    pub signature_algo: String,
}

impl Head {

    pub fn encode(&self) -> Option<Vec<u8>> {
        let proto = HeadProto {
            document_start: self.document_start,
            metadata_start: self.metadata_start,
            hash_algos: self.hash_algos.clone(),
            signature_algo: self.signature_algo.clone(),
        };

        let mut buf = BytesMut::with_capacity(proto.encoded_len());
        proto.encode(&mut buf).expect("encoding failed");

        Some(buf.to_vec())
    }

    pub fn decode(to_decode: Vec<u8>) -> Option<Self> {

        let mut bytes_mut: BytesMut = BytesMut::with_capacity(to_decode.len());
        bytes_mut.extend_from_slice(&to_decode);

        let proto_values = match HeadProto::decode(bytes_mut) {
            Ok(value) => value,
            Err(_) => return None
        };

        let head = Head {
            document_start: proto_values.document_start,
            metadata_start: proto_values.metadata_start,
            hash_algos: proto_values.hash_algos,
            signature_algo: proto_values.signature_algo,
        };

        Some(head)
    }
}
