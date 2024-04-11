#![cfg_attr(feature = "guest", no_std)]
#![no_main]

use k256::schnorr::Signature;

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

use k256::schnorr::{signature::Verifier, VerifyingKey};

#[jolt::provable]
fn schnorr(message: &[u8], verifying_key_bytes: &[u8], signature_bytes: &[u8]) {
    //
    // Verification
    //
    let verifying_key = VerifyingKey::from_bytes(&verifying_key_bytes).unwrap();
    let signature = Signature::try_from(signature_bytes).unwrap();
    verifying_key
        .verify(message, &signature)
        .expect("invalid signature");
}
