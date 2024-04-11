fn print_time<F: FnOnce()>(name: &str, f: F) {
    let start_time = std::time::Instant::now();
    f();
    let elapsed_time = start_time.elapsed();
    println!("{} Elapsed time: {:?}", name, elapsed_time);
}

pub fn main() {
    print_time("native fib(1000)", || {
        guest::execute_fib(1000);
    });
    let (prove_fib, _verify_fib) = guest::build_fib();
    print_time("zkVM fib(1000)", || {
        let (_output, _proof) = prove_fib(1000);
    });

    {
        // Schnorr Signature
        use k256::schnorr::{signature::Signer, SigningKey};
        use rand_core::OsRng; // requires 'getrandom' feature

        let message = b"Schnorr signatures prove knowledge of a secret in the random oracle model";
        // Signing
        let signing_key = SigningKey::random(&mut OsRng); // serialize with `.to_bytes()`
        let verifying_key_bytes = signing_key.verifying_key().to_bytes(); // 32-bytes
        let signature = signing_key.sign(message); // returns `k256::schnorr::Signature`
        let signature_bytes = signature.to_bytes(); // 64-bytes
        print_time("native Schnorr verify", || {
            guest::execute_schnorr(
                message,
                verifying_key_bytes.as_slice(),
                signature_bytes.as_slice(),
            );
        });
        let (prove_schnorr, _verify_schnorr) = guest::build_schnorr();
        print_time("zkVM Schnorr verify", || {
            let (_output, _proof) = prove_schnorr(
                message,
                verifying_key_bytes.as_slice(),
                signature_bytes.as_slice(),
            );
        });
        // let is_valid = verify_schnorr(proof);
        // println!("output: {}", output);
        // println!("valid: {}", is_valid);
    }
    /*
    {
        // RSA Signature
        use rsa::pkcs1v15::{SigningKey, VerifyingKey};
        use rsa::pkcs1::EncodeRsaPublicKey;
        use rsa::sha2::{Digest, Sha256};
        use rsa::signature::{Keypair, RandomizedSigner, SignatureEncoding, Verifier};
        use rsa::RsaPrivateKey;

        let mut rng = rand::thread_rng();

        let bits = 2048;
        let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
        let signing_key = SigningKey::<Sha256>::new(private_key);
        let verifying_key = signing_key.verifying_key();

        // Sign
        let data = b"hello world";
        let signature = signing_key.sign_with_rng(&mut rng, data);
        assert_ne!(signature.to_bytes().as_ref(), data.as_slice());
        let signature_bytes = signature.to_bytes();
        let verifying_key_bytes = verifying_key.to_pkcs1_der().unwrap();

        print_time("native RSA verify", || {
            guest::execute_rsa(
                data,
                verifying_key_bytes.as_bytes(),
                &signature_bytes.as_ref(),
            );
        });
        let (prove_rsa, _verify_rsa) = guest::build_rsa();
        print_time("zkVM RSA verify", || {
            let (_output, _proof) = prove_rsa(
                data,
                verifying_key_bytes.as_bytes(),
                &signature_bytes.as_ref(),
            );
        });
    }
    */
}
