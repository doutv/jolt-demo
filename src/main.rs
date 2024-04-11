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
        // RSA Signature
        use rsa::pkcs1v15::{SigningKey, VerifyingKey};
        use rsa::pkcs1::EncodeRsaPublicKey;
        use rsa::sha2::{Digest, Sha256};
        use rsa::signature::{Keypair, RandomizedSigner, SignatureEncoding, Verifier};
        use rsa::RsaPrivateKey;
        use rand_core::OsRng;

        let bits = 2048;
        let private_key = RsaPrivateKey::new(&mut OsRng, bits).expect("failed to generate a key");
        let signing_key = SigningKey::<Sha256>::new(private_key);
        let verifying_key = signing_key.verifying_key();

        // Sign
        let data = b"hello world";
        let signature = signing_key.sign_with_rng(&mut OsRng, data);
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
}
