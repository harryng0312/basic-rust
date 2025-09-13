#[cfg(test)]
mod test_ds {
    use crate::common::{gen_secured_random_byte_arr, to_base64};
    use libc::tm;
    use log::info;
    use openssl::ec::{EcGroup, EcKey};
    use openssl::hash::MessageDigest;
    use openssl::nid::Nid;
    use openssl::pkey::{PKey, PKeyRef};
    use openssl::sign::{Signer, Verifier};

    use p256::ecdsa::{
        signature::Signer as ECDSASigner, signature::Verifier as ECDSAVerifier, Signature,
        SigningKey, VerifyingKey,
    };
    use p256::elliptic_curve::consts::U64;
    use p256::elliptic_curve::generic_array::GenericArray;
    use p256::pkcs8::{
        DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey, LineEnding,
    };
    use pem::Pem;
    use utils::log::configuration::init_logger;

    #[test]
    fn test_ecdsa() {
        init_logger();
        // Create EC group
        let msg_digest = MessageDigest::sha256();
        let group = EcGroup::from_curve_name(Nid::SECP256K1).expect("Failed to create EC group");
        let mut priv_key_str: String;
        let mut pub_key_str: String;
        // Generate key_pair
        {
            let ec_key = EcKey::generate(&group).expect("Failed to generate EC key");
            let pkey = PKey::from_ec_key(ec_key).expect("Failed to convert EC key to PKey");
            let priv_key = pkey.ec_key().unwrap().private_key_to_pem().unwrap();
            let pub_key = pkey.ec_key().unwrap().public_key_to_pem().unwrap();
            priv_key_str = String::from_utf8_lossy(&priv_key).to_string();
            pub_key_str = String::from_utf8_lossy(&pub_key).to_string();
            info!("Private Key:\n{}\n", priv_key_str);
            info!("Private Key:\n{}\n", pub_key_str);
        }
        // Data for signing
        let data_bin = b"this is some data to sign";

        // Sign
        let mut signature: Vec<u8>;
        {
            let priv_key = priv_key_str.as_bytes();
            let pkey = PKey::private_key_from_pem(&priv_key).unwrap();
            let mut signer = Signer::new(msg_digest, &pkey).unwrap();
            signer.update(data_bin).unwrap();
            let mut buff = vec![0u8; signer.len().unwrap()];
            info!("Singer buff len: {}", buff.len());
            signature = signer.sign_to_vec().unwrap();
            info!("Signature len: {}", signature.len());
        }
        info!("Signature: {}", to_base64(signature.as_slice()).unwrap());
        // Verify
        let mut valid = false;
        {
            let pub_key = pub_key_str.as_bytes();
            let pkey = PKey::public_key_from_pem(&pub_key).unwrap();
            let mut verifier = Verifier::new(msg_digest, &pkey).unwrap();
            verifier.update(data_bin).unwrap();
            valid = verifier.verify(signature.as_slice()).unwrap();
        }
        // In kết quả
        info!("Signature valid: {}", valid);
    }

    #[test]
    fn test_ecdsa_rs() {
        init_logger();
        const BIT_LEN: usize = 256usize;
        let mut priv_key_str: String;
        let mut pub_key_str: String;
        // Generate key pair
        {
            let mut signing_key_bytes = [0u8; BIT_LEN / 8];
            _ = gen_secured_random_byte_arr(signing_key_bytes.as_mut());

            let priv_key: SigningKey = SigningKey::from_slice(&signing_key_bytes).unwrap();
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
        let mut sign_vec_bin: Vec<u8>;
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
            let mut pub_key = VerifyingKey::from_public_key_pem(pub_key_str.as_str()).unwrap();
            let sign_bytes: GenericArray<u8, U64> =
                GenericArray::from_slice(sign_vec_bin.as_slice()).to_owned();
            // let signature_array: SignatureBytes<NistP256> = GenericArray::clone_from_slice(sign_vec_bin.as_slice());
            let signature: Signature = Signature::from_bytes(&sign_bytes).unwrap();
            let verified_result = pub_key.verify(data_bin, &signature).is_ok();
            info!("Verify result:{}", verified_result);
        }
    }
}
