use k256::schnorr::{signature::Signer, SigningKey};
use rand_core::OsRng; // requires 'getrandom' feature

pub fn main() {
    let message = b"Schnorr signatures prove knowledge of a secret in the random oracle model";
    //
    // Signing
    //
    let signing_key = SigningKey::random(&mut OsRng); // serialize with `.to_bytes()`
    let verifying_key_bytes = signing_key.verifying_key().to_bytes(); // 32-bytes

    let signature = signing_key.sign(message); // returns `k256::schnorr::Signature`
    let signature_bytes = signature.to_bytes(); // 64-bytes

    let (prove_schnorr, verify_schnorr) = guest::build_schnorr();

    let (_output, proof) = prove_schnorr(
        message,
        verifying_key_bytes.as_slice(),
        signature_bytes.as_slice(),
    );
    let is_valid = verify_schnorr(proof);

    // println!("output: {}", output);
    println!("valid: {}", is_valid);
}
