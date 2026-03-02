// #[cfg(test)]
// mod sfile_tests {
//     use lib::sfile::sfile::SFileEncoder;
//     use lib::sign::signer_dilithium2::SignerDilithium2;
//     use lib::sign::verifier::Verifier;
//     use sha3::{Digest, Sha3_256};

//     use std::error::Error;
//     use std::fs::File;
//     use std::io::Read;
//     use tempfile::NamedTempFile;

//     #[test]
//     fn encode_produces_valid_signature() -> Result<(), Box<dyn Error>> {
//         let mut encoder = SFileEncoder::new();

//         encoder.add_metadata("type", "sfile");
//         encoder.add_metadata("format_version", "1");

//         encoder.add_metadata("author", "Alice");
//         encoder.add_metadata("version", "1.0");

//         encoder.add_tail("metadata_start", "value");
//         encoder.add_tail("signature_start", "value");
//         encoder.add_tail("hash_algs", "value");
//         encoder.add_tail("sign_alg", "value");

//         let tmp = NamedTempFile::new()?;
//         let path = tmp.path();

//         encoder.encode(&path)?;

//         let mut file = File::open(&path)?;
//         let mut whole = Vec::new();
//         file.read_to_end(&mut whole)?;

//         let meta_len = {
//             let mut arr = [0u8; 4];
//             arr.copy_from_slice(&whole[0..4]);
//             u32::from_le_bytes(arr) as usize
//         };
//         let meta_bytes = whole[0..meta_len].to_vec();

//         const SIGNATURE_LEN: usize = 2048;
//         let sig_start = meta_len;
//         let sig_end = sig_start + SIGNATURE_LEN;
//         let signature = whole[sig_start..sig_end].to_vec();

//         let payload_without_sig = whole[sig_end..].to_vec();

//         let mut hasher = Sha3_256::new();
//         hasher.update(&meta_bytes);
//         hasher.update(&payload_without_sig);
//         let hash = hasher.finalize().to_vec();

//         let file_size = whole.len() as u64;
//         let verifier = SignerDilithium2::create_verifier();
//         let ok = verifier.verify(signature, hash, file_size);
//         assert!(ok, "La signature générée ne passe pas la vérification");

//         Ok(())
//     }

//     #[test]
//     fn tampered_file_fails_verification() -> Result<(), Box<dyn Error>> {
//         // --------- préparer l’encodeur ----------
//         let mut encoder = SFileEncoder::new();

//         // ----- Mandatory fields (identiques à ceux du test précédent) -----
//         encoder.add_metadata("type", "sfile");
//         encoder.add_metadata("format_version", "1");

//         // ----- Champs additionnels de l’exemple -----
//         encoder.add_metadata("author", "Bob");

//         encoder.add_tail("metadata_start", "value");
//         encoder.add_tail("signature_start", "value");
//         encoder.add_tail("hash_algs", "value");
//         encoder.add_tail("sign_alg", "value");

//         // --------- fichier temporaire ----------
//         let tmp = NamedTempFile::new()?;
//         let path = tmp.path();

//         // --------- encoder ----------
//         encoder.encode(&path)?;

//         // --------- lire le fichier ----------
//         let mut file = File::open(&path)?;
//         let mut whole = Vec::new();
//         file.read_to_end(&mut whole)?;

//         // --------- découpage (identique au test précédent) ----------
//         let meta_len = {
//             let mut arr = [0u8; 4];
//             arr.copy_from_slice(&whole[0..4]);
//             u32::from_le_bytes(arr) as usize
//         };
//         let meta_bytes = whole[0..meta_len].to_vec();

//         const SIGNATURE_LEN: usize = 2048;
//         let sig_start = meta_len;
//         let sig_end = sig_start + SIGNATURE_LEN;
//         let signature = whole[sig_start..sig_end].to_vec();

//         let mut payload_without_sig = whole[sig_end..].to_vec();

//         const MAGIC: [u8; 4] = [0xAA, 0xAA, 0xAA, 0xAA];
//         let magic_pos = whole
//             .windows(4)
//             .position(|w| w == MAGIC)
//             .expect("Magic bits introuvables");

//         let tamper_idx = magic_pos + 4;
//         whole[tamper_idx] ^= 0xFF;

//         payload_without_sig = whole[sig_end..].to_vec();

//         let mut hasher = Sha3_256::new();
//         hasher.update(&meta_bytes);
//         hasher.update(&payload_without_sig);
//         let hash = hasher.finalize().to_vec();

//         let verifier = SignerDilithium2::create_verifier();
//         let ok = verifier.verify(signature, hash, whole.len() as u64);
//         assert!(!ok, "La vérification doit échouer sur un fichier altéré");

//         Ok(())
//     }
// }

// tests/real_file_sha3_test.rs
//
// Run only this test and see the `println!` output with:
//
//     cargo test --test real_file_sha3_test -- --nocapture
//
use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::io::Read;
use std::path::PathBuf;

use lib::sfile::sfile::SFileEncoder; // adjust if your crate path differs
use lib::sign::signer_dilithium2::SignerDilithium2; // idem
use lib::sign::verifier::Verifier;
use sha3::{Digest, Sha3_256}; // <-- SHA‑3‑256
use tempfile::TempDir;

fn split_sfile(bytes: &[u8]) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    const MAGIC: [u8; 4] = [0xAA, 0xAA, 0xAA, 0xAA];
    const SIGNATURE_LEN: usize = 2048;

    // Magic block appears **right after** the signature.
    let magic_pos = bytes
        .windows(4)
        .position(|w| w == MAGIC)
        .expect("Magic bits not found in the file");

    // Length of the metadata = magic position – signature length
    let meta_len = magic_pos
        .checked_sub(SIGNATURE_LEN)
        .expect("Failed to compute metadata length");

    let meta = bytes[0..meta_len].to_vec();
    let sig = bytes[meta_len..magic_pos].to_vec();
    let payload = bytes[magic_pos..].to_vec();

    (meta, sig, payload)
}

#[test]
fn encode_to_real_file_and_verify_sha3() -> Result<(), Box<dyn Error>> {
    let temp_dir = TempDir::new()?;
    let mut file_path: PathBuf = temp_dir.path().into();
    file_path.push("example_sha3.sfile");

    // -----------------------------------------------------------------
    // 2️⃣  **Make sure the file exists** before calling `encode`
    // -----------------------------------------------------------------
    // `OpenOptions::new().write(true).create(true).truncate(true)`
    // creates the file if it does not exist, or empties it if it does.
    {
        let _ = OpenOptions::new()
            .write(true)
            .create(true) // create if missing
            .truncate(true) // empty it if it already existed
            .open(&file_path)?;
    }

    // -----------------------------------------------------------------
    // 3️⃣  Initialise the encoder with the mandatory fields
    // -----------------------------------------------------------------
    let mut encoder = SFileEncoder::new();
    encoder.add_metadata("type", "sfile"); // mandatory
    encoder.add_metadata("format_version", "1"); // mandatory
    encoder.add_metadata("author", "Alice");
    encoder.add_metadata("version", "1.0");

    encoder.add_tail("metadata_start", "value");
    encoder.add_tail("signature_start", "value");
    encoder.add_tail("hash_algs", "value");
    encoder.add_tail("sign_alg", "value");

    // -----------------------------------------------------------------
    // 4️⃣  Encode – writes the signed S‑File to the real file
    // -----------------------------------------------------------------
    encoder.encode(&file_path)?;

    // -----------------------------------------------------------------
    // 5️⃣  Read the entire file (binary)
    // -----------------------------------------------------------------
    let mut raw = Vec::new();
    let mut f = File::open(&file_path)?;
    f.read_to_end(&mut raw)?;

    // -----------------------------------------------------------------
    // 6️⃣  Split the file into metadata / signature / payload
    // -----------------------------------------------------------------
    let (meta_bytes, signature, payload_without_sig) = split_sfile(&raw);

    // -----------------------------------------------------------------
    // 7️⃣  Re‑compute the hash **using SHA‑3‑256** (exactly as `encode` does)
    // -----------------------------------------------------------------
    let mut hasher = Sha3_256::new(); // <-- SHA‑3‑256
    hasher.update(&meta_bytes);
    hasher.update(&payload_without_sig);
    let hash = hasher.finalize().to_vec();

    // -----------------------------------------------------------------
    // 8️⃣  Verify the signature
    // -----------------------------------------------------------------
    let verifier = SignerDilithium2::create_verifier();
    let file_size = raw.len() as u64; // total size, includes the signature
    let ok = verifier.verify(signature, hash, file_size);
    assert!(ok, "Signature verification failed (SHA‑3‑256)");

    // -----------------------------------------------------------------
    // 9️⃣  Print the absolute path so you can open the file manually
    // -----------------------------------------------------------------
    let abs_path = fs::canonicalize(&file_path)?;
    println!(
        "\n✅  Signed S‑File (SHA‑3‑256) written to: {}",
        abs_path.display()
    );

    // -----------------------------------------------------------------
    // 🔟  (Optional) Small hex dump of the first 128 bytes
    // -----------------------------------------------------------------
    println!("--- First 128 bytes (hex) ---");
    for (i, byte) in raw.iter().take(128).enumerate() {
        if i % 16 == 0 {
            print!("\n{:04x}: ", i);
        }
        print!("{:02x} ", byte);
    }
    println!("\n--- End of dump ---\n");

    // TempDir is automatically removed when it goes out of scope,
    // but while the test runs you can inspect the file with `cat`,
    // `hexdump -C`, a hex editor, etc.
    Ok(())
}
