#[cfg(test)]
mod test {
    use crate::common::{from_base64, gen_secured_random_byte_arr, to_base64};
    use p256::ecdh::diffie_hellman;

    use rand_core::OsRng;
    use tracing::info;
    use utils::log::configuration::init_logger;

    #[test]
    /// Suppose there are 02 people: Alice & Bob
    ///
    fn test_ecdh_p256() {
        use p256::ecdh::SharedSecret;
        use p256::pkcs8::{
            DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey, LineEnding,
        };
        use p256::{PublicKey, SecretKey};

        init_logger();
        const BITLEN: usize = 256usize;

        let _secure_rand = OsRng;
        // Alice creates key_pair and publishes the public_key
        let a_priv_key: String;
        let a_pub_key: String;
        {
            let mut priv_key_bytes = [0u8; BITLEN / 8];
            gen_secured_random_byte_arr(priv_key_bytes.as_mut());

            let priv_key = SecretKey::from_slice(priv_key_bytes.as_ref()).unwrap();
            let priv_key_scalar = priv_key.to_nonzero_scalar();
            let pub_key = PublicKey::from_secret_scalar(&priv_key_scalar);
            let pub_key_der = pub_key.to_public_key_der().unwrap();
            // a_priv_key = to_base64(priv_key_bytes.as_ref()).unwrap();
            // a_priv_key = priv_key
            //     .to_pkcs8_der()
            //     .unwrap()
            //     .to_pem("PRIVATE KEY".as_ref(), LineEnding::LF)
            //     .unwrap()
            //     .to_string();
            a_priv_key = priv_key.to_pkcs8_pem(LineEnding::LF).unwrap().to_string();
            a_pub_key = pub_key_der
                .to_pem("PUBLIC KEY".as_ref(), LineEnding::LF)
                .unwrap();
            info!("Alice's private key: {}", a_priv_key);
            info!("Alice's public key: {}", a_pub_key);
        }

        // Bob creates key_pair and publishes the public_key
        let b_priv_key: String;
        let b_pub_key: String;
        {
            let mut priv_key_bytes = [0u8; BITLEN / 8];
            gen_secured_random_byte_arr(priv_key_bytes.as_mut());

            let priv_key = SecretKey::from_slice(priv_key_bytes.as_ref()).unwrap();
            let priv_key_scalar = priv_key.to_nonzero_scalar();
            let pub_key = PublicKey::from_secret_scalar(&priv_key_scalar);
            // b_priv_key = to_base64(&priv_key).unwrap();
            let pub_key_der = pub_key.to_public_key_der().unwrap();
            b_priv_key = priv_key.to_pkcs8_pem(LineEnding::LF).unwrap().to_string();
            b_pub_key = pub_key_der.to_pem("PUBLIC KEY", LineEnding::LF).unwrap();
            info!("Bob's private key: {}", "None");
            info!("Bob's public key: {}", b_pub_key);
        }

        // Alice side: Bob sends public_key to Alice. Alice generates the shared secret
        {
            let a_priv_key = SecretKey::from_pkcs8_pem(a_priv_key.as_str()).unwrap();
            let b_pub_key = PublicKey::from_public_key_pem(b_pub_key.as_str()).unwrap();
            let shared_secret = diffie_hellman(a_priv_key.to_nonzero_scalar(), b_pub_key.as_ref());
            let shared_secret = shared_secret.raw_secret_bytes().as_slice();
            info!(
                "Alice's shared secret: {:?}, len: {}",
                to_base64(shared_secret),
                shared_secret.len()
            );
        }

        // Bob side: Alice sends public_key to Bob. Bob generates the shared secret
        {
            let b_priv_key = SecretKey::from_pkcs8_pem(b_priv_key.as_str()).unwrap();
            let a_pub_key = PublicKey::from_public_key_pem(a_pub_key.as_str()).unwrap();
            let shared_secret = diffie_hellman(b_priv_key.to_nonzero_scalar(), a_pub_key.as_ref());
            let shared_secret = shared_secret.raw_secret_bytes().as_slice();
            info!(
                "Bob's shared secret: {:?}, len: {}",
                to_base64(shared_secret),
                shared_secret.len()
            );
        }
    }
}
