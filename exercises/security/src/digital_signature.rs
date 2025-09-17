#[cfg(test)]
mod test_ds {
    use crate::common::{gen_secured_random_byte_arr, to_base64};
    use tracing::info;
    use utils::log::configuration::init_logger;
    #[test]
    fn test_ecdsa_p256() {
        use p256::ecdsa::{
            signature::Signer as ECDSASigner, signature::Verifier as ECDSAVerifier, Signature,
            SigningKey, VerifyingKey,
        };
        use p256::elliptic_curve::consts::U64;
        use p256::elliptic_curve::generic_array::GenericArray;
        use p256::pkcs8::{
            DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey, LineEnding,
        };

        init_logger();
        const BIT_LEN: usize = 256usize;
        let mut priv_key_str: String;
        let mut pub_key_str: String;
        // Generate key pair
        {
            let mut signing_key_bytes = [0u8; BIT_LEN / 8];
            gen_secured_random_byte_arr(signing_key_bytes.as_mut());

            // let priv_key: SigningKey = SigningKey::from_slice(&signing_key_bytes).unwrap();
            let priv_key: SigningKey = SigningKey::try_from(signing_key_bytes.as_ref()).unwrap();
            let pub_key: VerifyingKey = VerifyingKey::from(&priv_key);

            let priv_pem = priv_key
                .to_pkcs8_der()
                .unwrap()
                .to_pem("PRIVATE KEY", LineEnding::LF)
                .unwrap();
            let pub_pem = pub_key
                .to_public_key_der()
                .unwrap()
                .to_pem("PUBLIC KEY", LineEnding::LF)
                .unwrap();
            priv_key_str = priv_pem.to_string();
            pub_key_str = pub_pem.to_string();
            info!("Private Key:\n{}\n", priv_key_str);
            info!("Private Key:\n{}\n", pub_key_str);
        }

        // Sign
        // Data for signing
        let data_bin = b"this is some data to sign";
        let sign_vec_bin: Vec<u8>;
        {
            let mut priv_key = SigningKey::from_pkcs8_pem(priv_key_str.as_str()).unwrap();
            let sign: Signature = priv_key.sign(data_bin);
            sign_vec_bin = sign.to_bytes().to_vec();
            info!(
                "Signature:{}",
                to_base64(sign_vec_bin.as_slice()).unwrap().to_string()
            );
        }

        // Verify
        {
            let pub_key = VerifyingKey::from_public_key_pem(pub_key_str.as_str()).unwrap();
            let sign_bytes: GenericArray<u8, U64> =
                GenericArray::from_slice(sign_vec_bin.as_slice()).to_owned();
            // let signature_array: SignatureBytes<NistP256> = GenericArray::clone_from_slice(sign_vec_bin.as_slice());
            let signature: Signature = Signature::from_bytes(&sign_bytes).unwrap();
            let verified_result = pub_key.verify(data_bin, &signature).is_ok();
            info!("Verify result:{}", verified_result);
        }
    }

    #[test]
    fn test_ecdsa_k256() {
        use k256::ecdsa::signature::{Signer, Verifier};
        use k256::ecdsa::{Signature, SigningKey, VerifyingKey};
        use k256::elliptic_curve::consts::U64;
        use k256::elliptic_curve::generic_array::GenericArray;
        use k256::pkcs8::{
            DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey, LineEnding,
        };
        init_logger();
        const BIT_LEN: usize = 256usize;
        let mut priv_key_str: String;
        let mut pub_key_str: String;

        // generate key pair
        {
            let mut signing_key_bytes = [0u8; BIT_LEN / 8];
            gen_secured_random_byte_arr(signing_key_bytes.as_mut());

            let priv_key = SigningKey::try_from(signing_key_bytes.as_ref()).unwrap();
            let pub_key = VerifyingKey::from(&priv_key);

            let priv_pem = priv_key
                .to_pkcs8_der()
                .unwrap()
                .to_pem("PRIVATE KEY", LineEnding::LF)
                .unwrap();
            let pub_pem = pub_key
                .to_public_key_der()
                .unwrap()
                .to_pem("PUBLIC KEY", LineEnding::LF)
                .unwrap();
            priv_key_str = priv_pem.to_string();
            pub_key_str = pub_pem.to_string();
            info!("Private Key:\n{}\n", priv_key_str);
            info!("Private Key:\n{}\n", pub_key_str);
        }

        // sign
        let data_bin = b"this is some data to sign";
        let sign_vec_bin: Vec<u8>;
        {
            let mut priv_key = SigningKey::from_pkcs8_pem(priv_key_str.as_str()).unwrap();
            let sign: Signature = priv_key.sign(data_bin);
            sign_vec_bin = sign.to_bytes().to_vec();
            info!(
                "Signature:{}",
                to_base64(sign_vec_bin.as_slice()).unwrap().to_string()
            );
        }

        // verify
        {
            let pub_key = VerifyingKey::from_public_key_pem(pub_key_str.as_str()).unwrap();
            let sign_bytes: GenericArray<u8, U64> =
                GenericArray::from_slice(sign_vec_bin.as_slice()).to_owned();
            // let signature_array: SignatureBytes<NistP256> = GenericArray::clone_from_slice(sign_vec_bin.as_slice());
            let signature: Signature = Signature::from_bytes(&sign_bytes).unwrap();
            let verified_result = pub_key.verify(data_bin, &signature).is_ok();
            info!("Verify result:{}", verified_result);
        }
    }
}
