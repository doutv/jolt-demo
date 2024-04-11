#![cfg_attr(feature = "guest", no_std)]
#![no_main]

/// Fibonacci(50)
/// Trace length: 1301
#[jolt::provable]
fn fib(n: u32) -> u128 {
    let mut a: u128 = 0;
    let mut b: u128 = 1;
    let mut sum: u128;
    for _ in 1..n {
        sum = a + b;
        a = b;
        b = sum;
    }

    b
}

/// Verify Schnorr Signature
/// Trace length: 169422912
#[jolt::provable(stack_size = 100000, memory_size = 10000000)]
fn schnorr(message: &[u8], verifying_key_bytes: &[u8], signature_bytes: &[u8]) {
    use k256::schnorr::{Signature, signature::Verifier, VerifyingKey};
    let verifying_key = VerifyingKey::from_bytes(&verifying_key_bytes).unwrap();
    let signature = Signature::try_from(signature_bytes).unwrap();
    verifying_key
        .verify(message, &signature)
        .unwrap();
}

// /// Verify RSA Signature
// /// Trace length:
// #[jolt::provable(stack_size = 100000, memory_size = 10000000)]
// fn rsa(message: &[u8], verifying_key_bytes: &[u8], signature_bytes: &[u8]) {
//     use rsa::pkcs1v15::VerifyingKey;
//     use rsa::signature::Verifier;
//     use rsa::{pkcs1::DecodeRsaPublicKey, pkcs1v15::Signature, sha2::Sha256};
//     let verifying_key: VerifyingKey<Sha256> = VerifyingKey::from_pkcs1_der(&verifying_key_bytes).unwrap();
//     let signature = Signature::try_from(signature_bytes).unwrap();
//     verifying_key
//         .verify(message, &signature)
//         .unwrap();
// }