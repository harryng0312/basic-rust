use crate::common::to_base64;
use libc::tm;
use log::info;
use openssl::ec::{EcGroup, EcKey};
use openssl::hash::MessageDigest;
use openssl::nid::Nid;
use openssl::pkey::{PKey, PKeyRef};
use openssl::sign::{Signer, Verifier};
use std::io::Write;
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
