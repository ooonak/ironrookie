use ed25519_dalek::pkcs8::spki::der::pem::LineEnding;
use ed25519_dalek::pkcs8::{EncodePrivateKey, EncodePublicKey};
use ed25519_dalek::{SigningKey, VerifyingKey};
use rand_core::OsRng;
use rmps::decode::Error;
use std::path::Path;

pub fn write_new_signing_key_set(path: &Path) -> Result<bool, Error> {
    if !path.is_dir() {
        return Ok(false);
    }

    let private_key = path.join("signing_key.pem");
    let public_key = path.join("signing_key_pub.pem");

    if private_key.exists() || public_key.exists() {
        return Ok(false);
    }

    let mut csprng = OsRng;
    let signing_key: SigningKey = SigningKey::generate(&mut csprng);
    signing_key
        .write_pkcs8_pem_file(private_key, LineEnding::default())
        .expect("Could not create private signing key");

    let verifying_key: VerifyingKey = signing_key.verifying_key();
    verifying_key
        .write_public_key_pem_file(public_key, LineEnding::default())
        .expect("Could not create public signing key");

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_new_signing_key_set_tests() {
        let path = Path::new("/tmp/non-existing");
        assert_eq!(write_new_signing_key_set(path).unwrap(), false);

        let path = Path::new("/tmp");
        let private_key = path.join("signing_key.pem");
        let public_key = path.join("signing_key_pub.pem");

        assert_eq!(write_new_signing_key_set(path).unwrap(), true);
        assert_eq!(write_new_signing_key_set(path).unwrap(), false);

        std::fs::remove_file(private_key).unwrap();
        std::fs::remove_file(public_key).unwrap();
    }
}
