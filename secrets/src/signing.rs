// https://docs.rs/ed25519-dalek/latest/ed25519_dalek/index.html

use ed25519_dalek::pkcs8::spki::der::pem::LineEnding;
use ed25519_dalek::pkcs8::{DecodePrivateKey, DecodePublicKey};
use ed25519_dalek::pkcs8::{EncodePrivateKey, EncodePublicKey};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey, SIGNATURE_LENGTH};
use rand_core::OsRng;
use rmps::decode::Error;
use std::path::Path;

const SIGNING_KEY_NAME: &'static str = "signing.pem";
const VERIFYING_KEY_NAME: &'static str = "verifying.pem";

pub struct Sign {
    signing_key: SigningKey,
}

impl Sign {
    pub fn new(path: &Path) -> Self {
        let signing_key_file = path.join(SIGNING_KEY_NAME);

        Self {
            signing_key: SigningKey::read_pkcs8_pem_file(signing_key_file)
                .expect("Failed to load signing key"),
        }
    }

    pub fn sign(&self, message: &Vec<u8>) -> [u8; SIGNATURE_LENGTH] {
        self.signing_key.sign(message).to_bytes()
    }
}

pub struct Verify {
    verifying_key: VerifyingKey,
}

impl Verify {
    pub fn new(path: &Path) -> Self {
        let verifying_key_file = path.join(VERIFYING_KEY_NAME);

        Self {
            verifying_key: VerifyingKey::read_public_key_pem_file(verifying_key_file)
                .expect("Failed to load verifying key"),
        }
    }

    pub fn verify(&self, message: &Vec<u8>, signature_bytes: &[u8; SIGNATURE_LENGTH]) -> bool {
        let signature: Signature =
            Signature::try_from(&signature_bytes[..]).expect("Could not load signature");
        self.verifying_key.verify(message, &signature).is_ok()
    }
}

pub fn write_new_signing_key_set(path: &Path) -> Result<bool, Error> {
    if !path.is_dir() {
        return Ok(false);
    }

    let signing_key_file = path.join(SIGNING_KEY_NAME);
    let verifying_key_file = path.join(VERIFYING_KEY_NAME);

    if signing_key_file.exists() || verifying_key_file.exists() {
        return Ok(false);
    }

    let mut csprng = OsRng;
    let signing_key: SigningKey = SigningKey::generate(&mut csprng);
    signing_key
        .write_pkcs8_pem_file(signing_key_file, LineEnding::default())
        .expect("Could not create private signing key");

    let verifying_key: VerifyingKey = signing_key.verifying_key();
    verifying_key
        .write_public_key_pem_file(verifying_key_file, LineEnding::default())
        .expect("Could not create public signing key");

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_and_verify_sign() {
        let path = Path::new("/tmp");
        assert_eq!(write_new_signing_key_set(path).unwrap(), true);

        let sign = Sign::new(path);
        let message = "Hello";
        let signature = sign.sign(&message.as_bytes().to_vec());

        let verify = Verify::new(path);
        assert_eq!(
            verify.verify(&message.as_bytes().to_vec(), &signature),
            true
        );

        let tampered_message = "hellO";
        assert_eq!(
            verify.verify(&tampered_message.as_bytes().to_vec(), &signature),
            false
        );

        std::fs::remove_file(path.join(SIGNING_KEY_NAME)).unwrap();
        std::fs::remove_file(path.join(VERIFYING_KEY_NAME)).unwrap();
    }

    #[test]
    fn test_write_new_signing_key_set() {
        let path = Path::new("/tmp/non-existing");
        assert_eq!(write_new_signing_key_set(path).unwrap(), false);

        let path = Path::new("/tmp");
        let signing_key_file = path.join(SIGNING_KEY_NAME);
        let verifying_key_file = path.join(VERIFYING_KEY_NAME);

        assert_eq!(write_new_signing_key_set(path).unwrap(), true);
        assert_eq!(write_new_signing_key_set(path).unwrap(), false);

        std::fs::remove_file(signing_key_file).unwrap();
        std::fs::remove_file(verifying_key_file).unwrap();
    }
}
