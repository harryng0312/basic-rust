use std::error::Error;
use libc::passwd;
use log::info;
use openssl::ec::{EcGroup, EcKey};
use openssl::nid::Nid;
use openssl::symm::Cipher;
use utils::log::configuration::init_logger;
use crate::common::to_base64;

fn gen_keypair(use_pem: bool, cipher: Option<Cipher>, passwd: Option<&[u8]>) -> Result<(String, String), Box<dyn Error>> {
    let ec_group = EcGroup::from_curve_name(Nid::SECP256K1).unwrap();
    let ec_key = EcKey::generate(&ec_group).unwrap();
    let mut priv_key_b64 = "".to_string();
    let mut pub_key_b64 = "".to_string();
    if use_pem {
        if cipher.is_some() && passwd.is_some() {
            priv_key_b64 = String::from_utf8_lossy(
                ec_key.private_key_to_pem_passphrase(cipher.unwrap(), passwd.unwrap())
                    .unwrap().as_slice()).to_string();
        } else {
            priv_key_b64 = String::from_utf8_lossy(ec_key.private_key_to_pem().unwrap().as_slice()).to_string();
        }
        pub_key_b64 = String::from_utf8_lossy(ec_key.public_key_to_pem().unwrap().as_slice()).to_string();
    } else {
        priv_key_b64 = to_base64(ec_key.private_key_to_der().unwrap().as_slice()).unwrap();
        pub_key_b64 = to_base64(ec_key.public_key_to_der().unwrap().as_slice()).unwrap();
    }
    Ok((priv_key_b64, pub_key_b64))
}
#[test]
fn test_gen_keypair() {
    init_logger();
    let (priv_key, pub_key) = gen_keypair(false, None, None).unwrap();
    info!("{:<5}", "DER");
    info!("Private key: {}", priv_key);
    info!("Public  key: {}", pub_key);
}

#[test]
fn test_save_private_key() {
    init_logger();
    let cipher = Cipher::aes_256_cbc();
    let (priv_key, pub_key) = gen_keypair(true, Some(cipher), Some("P@ssw0rd".as_bytes())).unwrap();
    info!("{:<5}", "PEM");
    info!("Private key:\n{}", priv_key);
    info!("Public  key:\n{}", pub_key);
}

#[test]
fn test_save_public_key() {
    init_logger();
}

#[test]
fn test_save_keystore() {
    init_logger();
}